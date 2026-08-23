use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use aimee_app::{
    AgentRepository, CommandInfra, DirectoryReaderInfra, EnvironmentInfra, FileDirectoryInfra,
    FileInfoInfra, FileReaderInfra, FileRemoverInfra, FileWriterInfra, GrpcInfra, HttpInfra,
    KVStore, McpServerInfra, StrategyFactory, UserInfra, WalkedFile, Walker, WalkerInfra,
};
use aimee_config::AimeeConfig;
use aimee_domain::{
    AnyProvider, AuthCredential, ChatCompletionMessage, ChatRepository, CommandOutput, Context,
    Conversation, ConversationId, ConversationRepository, Environment, FileInfo,
    FuzzySearchRepository, McpServerConfig, MigrationResult, Model, ModelId, Provider, ProviderId,
    ProviderRepository, ResultStream, SearchMatch, Skill, SkillRepository, Snapshot,
    SnapshotRepository, TextPatchBlock, TextPatchRepository,
};
use aimee_eventsource::EventSource;
use bytes::Bytes;
// Re-export CacacheStorage from aimee_infra
pub use aimee_infra::CacacheStorage;
use reqwest::Response;
use reqwest::header::HeaderMap;
use url::Url;

use crate::agent::AimeeAgentRepository;
use crate::context_engine::AimeeContextEngineRepository;
use crate::conversation::ConversationRepositoryImpl;
use crate::database::{DatabasePool, PoolConfig};
use crate::fs_snap::AimeeFileSnapshotService;
use crate::fuzzy_search::AimeeFuzzySearchRepository;
use crate::provider::{AimeeChatRepository, AimeeProviderRepository};
use crate::skill::AimeeSkillRepository;
use crate::validation::AimeeValidationRepository;

/// Repository layer that implements all domain repository traits
///
/// This struct aggregates all repository implementations and provides a single
/// point of access for data persistence operations.
#[derive(Clone)]
pub struct AimeeRepo<F> {
    infra: Arc<F>,
    file_snapshot_service: Arc<AimeeFileSnapshotService>,
    conversation_repository: Arc<ConversationRepositoryImpl>,
    mcp_cache_repository: Arc<CacacheStorage>,
    provider_repository: Arc<AimeeProviderRepository<F>>,
    chat_repository: Arc<AimeeChatRepository<F>>,
    codebase_repo: Arc<AimeeContextEngineRepository<F>>,
    agent_repository: Arc<AimeeAgentRepository<F>>,
    skill_repository: Arc<AimeeSkillRepository<F>>,
    validation_repository: Arc<AimeeValidationRepository<F>>,
    fuzzy_search_repository: Arc<AimeeFuzzySearchRepository<F>>,
}

impl<
    F: EnvironmentInfra<Config = aimee_config::AimeeConfig>
        + FileReaderInfra
        + FileWriterInfra
        + GrpcInfra
        + HttpInfra,
> AimeeRepo<F>
{
    pub fn new(infra: Arc<F>) -> Self {
        let env = infra.get_environment();
        let file_snapshot_service = Arc::new(AimeeFileSnapshotService::new(env.clone()));
        let db_pool =
            Arc::new(DatabasePool::try_from(PoolConfig::new(env.database_path())).unwrap());
        let conversation_repository = Arc::new(ConversationRepositoryImpl::new(
            db_pool.clone(),
            env.workspace_hash(),
        ));

        let mcp_cache_repository = Arc::new(CacacheStorage::new(
            env.cache_dir().join("mcp_cache"),
            Some(3600),
        )); // 1 hour TTL

        let provider_repository = Arc::new(AimeeProviderRepository::new(infra.clone()));
        let chat_repository = Arc::new(AimeeChatRepository::new(infra.clone()));

        let codebase_repo = Arc::new(AimeeContextEngineRepository::new(infra.clone()));
        let agent_repository = Arc::new(AimeeAgentRepository::new(infra.clone()));
        let skill_repository = Arc::new(AimeeSkillRepository::new(infra.clone()));
        let validation_repository = Arc::new(AimeeValidationRepository::new(infra.clone()));
        let fuzzy_search_repository = Arc::new(AimeeFuzzySearchRepository::new(infra.clone()));
        Self {
            infra,
            file_snapshot_service,
            conversation_repository,
            mcp_cache_repository,
            provider_repository,
            chat_repository,
            codebase_repo,
            agent_repository,
            skill_repository,
            validation_repository,
            fuzzy_search_repository,
        }
    }
}

#[async_trait::async_trait]
impl<F: Send + Sync> SnapshotRepository for AimeeRepo<F> {
    async fn insert_snapshot(&self, file_path: &Path) -> anyhow::Result<Snapshot> {
        self.file_snapshot_service.insert_snapshot(file_path).await
    }

    async fn undo_snapshot(&self, file_path: &Path) -> anyhow::Result<()> {
        self.file_snapshot_service.undo_snapshot(file_path).await
    }
}

#[async_trait::async_trait]
impl<F: Send + Sync> ConversationRepository for AimeeRepo<F> {
    async fn upsert_conversation(&self, conversation: Conversation) -> anyhow::Result<()> {
        self.conversation_repository
            .upsert_conversation(conversation)
            .await
    }

    async fn get_conversation(
        &self,
        conversation_id: &ConversationId,
    ) -> anyhow::Result<Option<Conversation>> {
        self.conversation_repository
            .get_conversation(conversation_id)
            .await
    }

    async fn get_all_conversations(
        &self,
        limit: Option<usize>,
    ) -> anyhow::Result<Option<Vec<Conversation>>> {
        self.conversation_repository
            .get_all_conversations(limit)
            .await
    }

    async fn get_last_conversation(&self) -> anyhow::Result<Option<Conversation>> {
        self.conversation_repository.get_last_conversation().await
    }

    async fn delete_conversation(&self, conversation_id: &ConversationId) -> anyhow::Result<()> {
        self.conversation_repository
            .delete_conversation(conversation_id)
            .await
    }
}

#[async_trait::async_trait]
impl<
    F: EnvironmentInfra<Config = aimee_config::AimeeConfig>
        + FileReaderInfra
        + FileWriterInfra
        + HttpInfra
        + Send
        + Sync,
> ChatRepository for AimeeRepo<F>
{
    async fn chat(
        &self,
        model_id: &ModelId,
        context: Context,
        provider: Provider<Url>,
    ) -> ResultStream<ChatCompletionMessage, anyhow::Error> {
        self.chat_repository.chat(model_id, context, provider).await
    }

    async fn models(&self, provider: Provider<Url>) -> anyhow::Result<Vec<Model>> {
        self.chat_repository.models(provider).await
    }
}

#[async_trait::async_trait]
impl<
    F: EnvironmentInfra<Config = aimee_config::AimeeConfig>
        + FileReaderInfra
        + FileWriterInfra
        + HttpInfra
        + Send
        + Sync,
> ProviderRepository for AimeeRepo<F>
{
    async fn get_all_providers(&self) -> anyhow::Result<Vec<AnyProvider>> {
        self.provider_repository.get_all_providers().await
    }

    async fn get_provider(&self, id: ProviderId) -> anyhow::Result<aimee_domain::ProviderTemplate> {
        self.provider_repository.get_provider(id).await
    }

    async fn upsert_credential(&self, credential: AuthCredential) -> anyhow::Result<()> {
        // All providers now use file-based credentials
        self.provider_repository.upsert_credential(credential).await
    }

    async fn get_credential(&self, id: &ProviderId) -> anyhow::Result<Option<AuthCredential>> {
        self.provider_repository.get_credential(id).await
    }

    async fn remove_credential(&self, id: &ProviderId) -> anyhow::Result<()> {
        // All providers now use file-based credentials
        self.provider_repository.remove_credential(id).await
    }

    async fn migrate_env_credentials(&self) -> anyhow::Result<Option<MigrationResult>> {
        self.provider_repository.migrate_env_to_file().await
    }
}

#[async_trait::async_trait]
impl<F: EnvironmentInfra<Config = aimee_config::AimeeConfig> + Send + Sync> EnvironmentInfra
    for AimeeRepo<F>
{
    type Config = aimee_config::AimeeConfig;

    fn get_environment(&self) -> Environment {
        self.infra.get_environment()
    }

    fn get_config(&self) -> anyhow::Result<aimee_config::AimeeConfig> {
        self.infra.get_config()
    }

    fn update_environment(
        &self,
        ops: Vec<aimee_domain::ConfigOperation>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        self.infra.update_environment(ops)
    }

    fn get_env_var(&self, key: &str) -> Option<String> {
        self.infra.get_env_var(key)
    }

    fn get_env_vars(&self) -> BTreeMap<String, String> {
        self.infra.get_env_vars()
    }
}

#[async_trait::async_trait]
impl<F: Send + Sync> KVStore for AimeeRepo<F> {
    async fn cache_get<K, V>(&self, key: &K) -> anyhow::Result<Option<V>>
    where
        K: std::hash::Hash + Sync,
        V: serde::Serialize + serde::de::DeserializeOwned + Send,
    {
        self.mcp_cache_repository.cache_get(key).await
    }

    async fn cache_set<K, V>(&self, key: &K, value: &V) -> anyhow::Result<()>
    where
        K: std::hash::Hash + Sync,
        V: serde::Serialize + Sync,
    {
        self.mcp_cache_repository.cache_set(key, value).await
    }

    async fn cache_clear(&self) -> anyhow::Result<()> {
        self.mcp_cache_repository.cache_clear().await
    }
}

#[async_trait::async_trait]
impl<F: HttpInfra> HttpInfra for AimeeRepo<F> {
    async fn http_get(&self, url: &Url, headers: Option<HeaderMap>) -> anyhow::Result<Response> {
        self.infra.http_get(url, headers).await
    }

    async fn http_post(
        &self,
        url: &Url,
        headers: Option<HeaderMap>,
        body: Bytes,
    ) -> anyhow::Result<Response> {
        self.infra.http_post(url, headers, body).await
    }

    async fn http_delete(&self, url: &Url) -> anyhow::Result<Response> {
        self.infra.http_delete(url).await
    }

    async fn http_eventsource(
        &self,
        url: &Url,
        headers: Option<HeaderMap>,
        body: Bytes,
    ) -> anyhow::Result<EventSource> {
        self.infra.http_eventsource(url, headers, body).await
    }
}

#[async_trait::async_trait]
impl<F> FileReaderInfra for AimeeRepo<F>
where
    F: FileReaderInfra + Send + Sync,
{
    async fn read_utf8(&self, path: &Path) -> anyhow::Result<String> {
        self.infra.read_utf8(path).await
    }

    fn read_batch_utf8(
        &self,
        batch_size: usize,
        paths: Vec<PathBuf>,
    ) -> impl futures::Stream<Item = (PathBuf, anyhow::Result<String>)> + Send {
        self.infra.read_batch_utf8(batch_size, paths)
    }

    async fn read(&self, path: &Path) -> anyhow::Result<Vec<u8>> {
        self.infra.read(path).await
    }

    async fn range_read_utf8(
        &self,
        path: &Path,
        start_line: u64,
        end_line: u64,
    ) -> anyhow::Result<(String, FileInfo)> {
        self.infra.range_read_utf8(path, start_line, end_line).await
    }
}

#[async_trait::async_trait]
impl<F> WalkerInfra for AimeeRepo<F>
where
    F: WalkerInfra + Send + Sync,
{
    async fn walk(&self, config: Walker) -> anyhow::Result<Vec<WalkedFile>> {
        self.infra.walk(config).await
    }
}

#[async_trait::async_trait]
impl<F> FileWriterInfra for AimeeRepo<F>
where
    F: FileWriterInfra + Send + Sync,
{
    async fn write(&self, path: &Path, contents: Bytes) -> anyhow::Result<()> {
        self.infra.write(path, contents).await
    }
    async fn append(&self, path: &Path, contents: Bytes) -> anyhow::Result<()> {
        self.infra.append(path, contents).await
    }
    async fn write_temp(&self, prefix: &str, ext: &str, content: &str) -> anyhow::Result<PathBuf> {
        self.infra.write_temp(prefix, ext, content).await
    }
}

#[async_trait::async_trait]
impl<F> FileInfoInfra for AimeeRepo<F>
where
    F: FileInfoInfra + Send + Sync,
{
    async fn is_binary(&self, path: &Path) -> anyhow::Result<bool> {
        self.infra.is_binary(path).await
    }
    async fn is_file(&self, path: &Path) -> anyhow::Result<bool> {
        self.infra.is_file(path).await
    }
    async fn exists(&self, path: &Path) -> anyhow::Result<bool> {
        self.infra.exists(path).await
    }
    async fn file_size(&self, path: &Path) -> anyhow::Result<u64> {
        self.infra.file_size(path).await
    }
}

#[async_trait::async_trait]
impl<F> FileDirectoryInfra for AimeeRepo<F>
where
    F: FileDirectoryInfra + Send + Sync,
{
    async fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        self.infra.create_dirs(path).await
    }
}

#[async_trait::async_trait]
impl<F> FileRemoverInfra for AimeeRepo<F>
where
    F: FileRemoverInfra + Send + Sync,
{
    async fn remove(&self, path: &Path) -> anyhow::Result<()> {
        self.infra.remove(path).await
    }
}

#[async_trait::async_trait]
impl<F> DirectoryReaderInfra for AimeeRepo<F>
where
    F: DirectoryReaderInfra + Send + Sync,
{
    async fn list_directory_entries(
        &self,
        directory: &Path,
    ) -> anyhow::Result<Vec<(PathBuf, bool)>> {
        self.infra.list_directory_entries(directory).await
    }

    async fn read_directory_files(
        &self,
        directory: &Path,
        pattern: Option<&str>, // Optional glob pattern like "*.md"
    ) -> anyhow::Result<Vec<(PathBuf, String)>> {
        self.infra.read_directory_files(directory, pattern).await
    }
}

#[async_trait::async_trait]
impl<F> UserInfra for AimeeRepo<F>
where
    F: UserInfra + Send + Sync,
{
    async fn prompt_question(&self, question: &str) -> anyhow::Result<Option<String>> {
        self.infra.prompt_question(question).await
    }

    async fn select_one<T: Clone + std::fmt::Display + Send + 'static>(
        &self,
        message: &str,
        options: Vec<T>,
    ) -> anyhow::Result<Option<T>> {
        self.infra.select_one(message, options).await
    }

    async fn select_one_enum<T>(&self, message: &str) -> anyhow::Result<Option<T>>
    where
        T: Clone + std::fmt::Display + Send + 'static + strum::IntoEnumIterator + std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.infra.select_one_enum(message).await
    }

    async fn select_many<T: std::fmt::Display + Clone + Send + 'static>(
        &self,
        message: &str,
        options: Vec<T>,
    ) -> anyhow::Result<Option<Vec<T>>> {
        self.infra.select_many(message, options).await
    }
}

#[async_trait::async_trait]
impl<F> McpServerInfra for AimeeRepo<F>
where
    F: McpServerInfra + Send + Sync,
{
    type Client = F::Client;

    async fn connect(
        &self,
        config: McpServerConfig,
        env_vars: &BTreeMap<String, String>,
        environment: &Environment,
    ) -> anyhow::Result<F::Client> {
        self.infra.connect(config, env_vars, environment).await
    }
}

#[async_trait::async_trait]
impl<F> CommandInfra for AimeeRepo<F>
where
    F: CommandInfra + Send + Sync,
{
    async fn execute_command(
        &self,
        command: String,
        working_dir: PathBuf,
        silent: bool,
        env_vars: Option<Vec<String>>,
    ) -> anyhow::Result<CommandOutput> {
        self.infra
            .execute_command(command, working_dir, silent, env_vars)
            .await
    }

    async fn execute_command_raw(
        &self,
        command: &str,
        working_dir: PathBuf,
        env_vars: Option<Vec<String>>,
    ) -> anyhow::Result<std::process::ExitStatus> {
        self.infra
            .execute_command_raw(command, working_dir, env_vars)
            .await
    }
}

#[async_trait::async_trait]
impl<F: FileInfoInfra + EnvironmentInfra<Config = AimeeConfig> + DirectoryReaderInfra + Send + Sync>
    AgentRepository for AimeeRepo<F>
{
    async fn get_agents(&self) -> anyhow::Result<Vec<aimee_domain::Agent>> {
        self.agent_repository.get_agents().await
    }

    async fn get_agent_infos(&self) -> anyhow::Result<Vec<aimee_domain::AgentInfo>> {
        self.agent_repository.get_agent_infos().await
    }
}

#[async_trait::async_trait]
impl<F: FileInfoInfra + EnvironmentInfra + FileReaderInfra + WalkerInfra + Send + Sync>
    SkillRepository for AimeeRepo<F>
{
    async fn load_skills(&self) -> anyhow::Result<Vec<Skill>> {
        self.skill_repository.load_skills().await
    }
}

impl<F: StrategyFactory> StrategyFactory for AimeeRepo<F> {
    type Strategy = F::Strategy;

    fn create_auth_strategy(
        &self,
        provider_id: ProviderId,
        auth_method: aimee_domain::AuthMethod,
        required_params: Vec<aimee_domain::URLParamSpec>,
    ) -> anyhow::Result<Self::Strategy> {
        self.infra
            .create_auth_strategy(provider_id, auth_method, required_params)
    }
}

#[async_trait::async_trait]
impl<F: GrpcInfra + Send + Sync> aimee_domain::WorkspaceIndexRepository for AimeeRepo<F> {
    async fn authenticate(&self) -> anyhow::Result<aimee_domain::WorkspaceAuth> {
        self.codebase_repo.authenticate().await
    }

    async fn create_workspace(
        &self,
        working_dir: &std::path::Path,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<aimee_domain::WorkspaceId> {
        self.codebase_repo
            .create_workspace(working_dir, auth_token)
            .await
    }

    async fn upload_files(
        &self,
        upload: &aimee_domain::FileUpload,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<aimee_domain::FileUploadInfo> {
        self.codebase_repo.upload_files(upload, auth_token).await
    }

    async fn search(
        &self,
        query: &aimee_domain::CodeSearchQuery<'_>,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<Vec<aimee_domain::Node>> {
        self.codebase_repo.search(query, auth_token).await
    }

    async fn list_workspaces(
        &self,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<Vec<aimee_domain::WorkspaceInfo>> {
        self.codebase_repo.list_workspaces(auth_token).await
    }

    async fn get_workspace(
        &self,
        workspace_id: &aimee_domain::WorkspaceId,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<Option<aimee_domain::WorkspaceInfo>> {
        self.codebase_repo
            .get_workspace(workspace_id, auth_token)
            .await
    }

    async fn list_workspace_files(
        &self,
        workspace: &aimee_domain::WorkspaceFiles,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<Vec<aimee_domain::FileHash>> {
        self.codebase_repo
            .list_workspace_files(workspace, auth_token)
            .await
    }

    async fn delete_files(
        &self,
        deletion: &aimee_domain::FileDeletion,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<()> {
        self.codebase_repo.delete_files(deletion, auth_token).await
    }

    async fn delete_workspace(
        &self,
        workspace_id: &aimee_domain::WorkspaceId,
        auth_token: &aimee_domain::ApiKey,
    ) -> anyhow::Result<()> {
        self.codebase_repo
            .delete_workspace(workspace_id, auth_token)
            .await
    }
}

#[async_trait::async_trait]
impl<F: GrpcInfra + Send + Sync> aimee_domain::ValidationRepository for AimeeRepo<F> {
    async fn validate_file(
        &self,
        path: impl AsRef<std::path::Path> + Send,
        content: &str,
    ) -> anyhow::Result<Vec<aimee_domain::SyntaxError>> {
        self.validation_repository
            .validate_file(path, content)
            .await
    }
}

#[async_trait::async_trait]
impl<F: GrpcInfra + Send + Sync> FuzzySearchRepository for AimeeRepo<F> {
    async fn fuzzy_search(
        &self,
        needle: &str,
        haystack: &str,
        search_all: bool,
    ) -> anyhow::Result<Vec<SearchMatch>> {
        self.fuzzy_search_repository
            .fuzzy_search(needle, haystack, search_all)
            .await
    }
}

#[async_trait::async_trait]
impl<F: GrpcInfra + Send + Sync> TextPatchRepository for AimeeRepo<F> {
    async fn build_text_patch(
        &self,
        haystack: &str,
        old_string: &str,
        new_string: &str,
    ) -> anyhow::Result<TextPatchBlock> {
        let request = tonic::Request::new(crate::proto_generated::BuildTextPatchRequest {
            haystack: haystack.to_string(),
            old_string: old_string.to_string(),
            new_string: new_string.to_string(),
        });

        let channel = self.infra.channel()?;
        let mut client =
            crate::proto_generated::aimee_service_client::AimeeServiceClient::new(channel);
        let response = client.build_text_patch(request).await?.into_inner();

        Ok(TextPatchBlock { patch: response.patch, patched_text: response.patched_text })
    }
}

impl<F: GrpcInfra> GrpcInfra for AimeeRepo<F> {
    fn channel(&self) -> anyhow::Result<tonic::transport::Channel> {
        self.infra.channel()
    }

    fn hydrate(&self) {
        self.infra.hydrate();
    }
}

impl<F: aimee_domain::ConsoleWriter> aimee_domain::ConsoleWriter for AimeeRepo<F> {
    fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
        self.infra.write(buf)
    }

    fn write_err(&self, buf: &[u8]) -> std::io::Result<usize> {
        self.infra.write_err(buf)
    }

    fn flush(&self) -> std::io::Result<()> {
        self.infra.flush()
    }

    fn flush_err(&self) -> std::io::Result<()> {
        self.infra.flush_err()
    }
}
