use std::path::PathBuf;
use std::sync::LazyLock;

use config::ConfigBuilder;
use config::builder::DefaultState;

use crate::AimeeConfig;
use crate::legacy::LegacyConfig;

/// Loads all `.env` files found while walking up from the current working
/// directory to the root, with priority given to closer (lower) directories.
/// Executed at most once per process.
static LOAD_DOT_ENV: LazyLock<()> = LazyLock::new(|| {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut paths = vec![];
    let mut current = PathBuf::new();

    for component in cwd.components() {
        current.push(component);
        paths.push(current.clone());
    }

    paths.reverse();

    for path in paths {
        let env_file = path.join(".env");
        if env_file.is_file() {
            dotenvy::from_path(&env_file).ok();
        }
    }
});

/// Caches base-path resolution for the process lifetime.
static BASE_PATH: LazyLock<PathBuf> = LazyLock::new(ConfigReader::resolve_base_path);

/// Merges [`AimeeConfig`] from layered sources using a builder pattern.
#[derive(Default)]
pub struct ConfigReader {
    builder: ConfigBuilder<DefaultState>,
}

impl ConfigReader {
    /// Returns the path to the legacy JSON config file
    /// (`~/.aimee/.config.json`).
    pub fn config_legacy_path() -> PathBuf {
        Self::base_path().join(".config.json")
    }

    /// Returns the path to the primary TOML config file
    /// (`~/.aimee/.aimee.toml`).
    pub fn config_path() -> PathBuf {
        Self::base_path().join(".aimee.toml")
    }

    /// Returns the base directory for all Aimee config files.
    ///
    /// Resolution order:
    /// 1. `AIMEE_CONFIG` environment variable, if set.
    /// 2. `~/aimee` or `~/.aimee` when that directory already exists.
    /// 3. `~/forge` or `~/.forge` (Forge Code legacy) when that directory
    ///    exists, so existing users keep their config without disruption.
    /// 4. `~/.aimee` as the default path for new installs.
    pub fn base_path() -> PathBuf {
        BASE_PATH.clone()
    }

    fn resolve_base_path() -> PathBuf {
        if let Ok(path) = std::env::var("AIMEE_CONFIG") {
            return PathBuf::from(path);
        }
        if let Ok(path) = std::env::var("OMEGA_CONFIG") {
            return PathBuf::from(path);
        }

        let base = dirs::home_dir().unwrap_or(PathBuf::from("."));
        for candidate in ["aimee", ".aimee", "omega", ".omega", "forge", ".forge"] {
            let path = base.join(candidate);
            if path.exists() {
                tracing::info!(path = %path.display(), "Using existing config directory");
                return path;
            }
        }

        tracing::info!("Using new path");
        base.join(".aimee")
    }

    /// Adds the provided TOML string as a config source without touching the
    /// filesystem.
    pub fn read_toml(mut self, contents: &str) -> Self {
        self.builder = self
            .builder
            .add_source(config::File::from_str(contents, config::FileFormat::Toml));

        self
    }

    /// Adds the embedded default config (`../.aimee.toml`) as a source.
    pub fn read_defaults(self) -> Self {
        let defaults = include_str!("../.aimee.toml");

        self.read_toml(defaults)
    }

    /// Adds `AIMEE_`-prefixed environment variables as a config source.
    ///
    /// Legacy `OMEGA_` variables are still read, then overwritten by `AIMEE_`
    /// when both are set.
    pub fn read_env(mut self) -> Self {
        let env_source = |prefix: &str| {
            config::Environment::with_prefix(prefix)
                .prefix_separator("_")
                .separator("__")
                .try_parsing(true)
                .list_separator(",")
                .with_list_parse_key("retry.status_codes")
                .with_list_parse_key("http.root_cert_paths")
        };
        self.builder = self
            .builder
            .add_source(env_source("OMEGA"))
            .add_source(env_source("AIMEE"));

        self
    }

    /// Builds and deserializes all accumulated sources into a [`AimeeConfig`].
    ///
    /// Triggers `.env` file loading (at most once per process) by walking up
    /// the directory tree from the current working directory, with closer
    /// directories taking priority.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be built or deserialized.
    pub fn build(self) -> crate::Result<AimeeConfig> {
        *LOAD_DOT_ENV;
        let config = self.builder.build()?;
        Ok(config.try_deserialize::<AimeeConfig>()?)
    }

    /// Adds `~/.aimee/.aimee.toml` as a config source, silently skipping if
    /// absent.
    ///
    /// A leftover `.omega.toml` in the same directory is loaded first so
    /// existing installs keep their settings until they save a new file.
    pub fn read_global(mut self) -> Self {
        let base = Self::base_path();
        self.builder = self
            .builder
            .add_source(config::File::from(base.join(".omega.toml")).required(false))
            .add_source(config::File::from(base.join(".aimee.toml")).required(false));
        self
    }

    /// Reads `~/.aimee/.config.json` (legacy format) and adds it as a source,
    /// silently skipping errors.
    pub fn read_legacy(self) -> Self {
        let content = LegacyConfig::read(&Self::config_legacy_path());
        if let Ok(content) = content {
            self.read_toml(&content)
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Mutex, MutexGuard};

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::ModelConfig;

    /// Serializes tests that mutate environment variables to prevent races.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    /// Holds env vars set for a test's duration and removes them on drop, while
    /// holding [`ENV_MUTEX`].
    struct EnvGuard {
        keys: Vec<&'static str>,
        _lock: MutexGuard<'static, ()>,
    }

    impl EnvGuard {
        /// Acquires [`ENV_MUTEX`], sets each `(key, value)` pair in the
        /// environment, and removes each key in `remove` if present. All
        /// set keys are cleaned up on drop.
        #[must_use]
        fn set(pairs: &[(&'static str, &str)]) -> Self {
            Self::set_and_remove(pairs, &[])
        }

        /// Like [`set`] but also removes the listed keys before the test runs.
        #[must_use]
        fn set_and_remove(pairs: &[(&'static str, &str)], remove: &[&'static str]) -> Self {
            let lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
            let keys = pairs.iter().map(|(k, _)| *k).collect();
            for key in remove {
                unsafe { std::env::remove_var(key) };
            }
            for (key, value) in pairs {
                unsafe { std::env::set_var(key, value) };
            }
            Self { keys, _lock: lock }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for key in &self.keys {
                unsafe { std::env::remove_var(key) };
            }
        }
    }

    #[test]
    fn test_base_path_uses_aimee_config_env_var() {
        let _guard = EnvGuard::set(&[("AIMEE_CONFIG", "/custom/aimee/dir")]);
        let actual = ConfigReader::resolve_base_path();
        let expected = PathBuf::from("/custom/aimee/dir");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_base_path_falls_back_to_home_dir_when_env_var_absent() {
        // Hold the env mutex and ensure AIMEE_CONFIG is absent so this test
        // cannot race with test_base_path_uses_aimee_config_env_var.
        let _guard = EnvGuard::set_and_remove(&[], &["AIMEE_CONFIG"]);

        let actual = ConfigReader::resolve_base_path();
        // Without AIMEE_CONFIG the path is an existing branded or Forge-legacy
        // directory, otherwise the new default `.aimee`.
        let name = actual.file_name().unwrap();
        assert!(
            name == "aimee" || name == ".aimee" || name == "forge" || name == ".forge",
            "Expected base_path to end with 'aimee', '.aimee', 'forge', or '.forge', got: {:?}",
            name
        );
    }

    #[test]
    fn test_read_parses_without_error() {
        let actual = ConfigReader::default().read_defaults().build();
        assert!(actual.is_ok(), "read() failed: {:?}", actual.err());
    }

    #[test]
    fn test_legacy_layer_does_not_overwrite_defaults() {
        // Simulate what `read_legacy` does: serialize a AimeeConfig that only
        // carries session/commit/suggest (all other fields are None) and layer
        // it on top of the embedded defaults. The default values must survive.
        let legacy = AimeeConfig {
            session: Some(ModelConfig {
                provider_id: "anthropic".to_string(),
                model_id: "claude-3".to_string(),
            }),
            ..Default::default()
        };
        let legacy_toml = toml_edit::ser::to_string_pretty(&legacy).unwrap();

        let actual = ConfigReader::default()
            // Read legacy first and then defaults
            .read_toml(&legacy_toml)
            .read_defaults()
            .build()
            .unwrap();

        // Session should come from the legacy layer
        assert_eq!(
            actual.session,
            Some(ModelConfig {
                provider_id: "anthropic".to_string(),
                model_id: "claude-3".to_string(),
            })
        );

        // Default values from .aimee.toml must be retained, not reset to zero
        assert_eq!(actual.max_parallel_file_reads, 64);
        assert_eq!(actual.max_read_lines, 2000);
        assert_eq!(actual.tool_timeout_secs, 300);
        assert_eq!(actual.max_search_lines, 1000);
        assert_eq!(actual.tool_supported, true);
    }

    #[test]
    fn test_read_session_from_env_vars() {
        let _guard = EnvGuard::set(&[
            ("AIMEE_SESSION__PROVIDER_ID", "fake-provider"),
            ("AIMEE_SESSION__MODEL_ID", "fake-model"),
        ]);

        let actual = ConfigReader::default()
            .read_defaults()
            .read_env()
            .build()
            .unwrap();

        let expected = Some(ModelConfig {
            provider_id: "fake-provider".to_string(),
            model_id: "fake-model".to_string(),
        });
        assert_eq!(actual.session, expected);
    }

    #[test]
    fn test_use_aimee_committer_defaults_to_true() {
        let actual = ConfigReader::default().read_defaults().build().unwrap();

        assert_eq!(actual.use_aimee_committer, true);
    }

    #[test]
    fn test_use_aimee_committer_can_be_disabled() {
        let toml = "use_aimee_committer = false\n";

        let actual = ConfigReader::default()
            .read_defaults()
            .read_toml(toml)
            .build()
            .unwrap();

        assert_eq!(actual.use_aimee_committer, false);
    }
}
