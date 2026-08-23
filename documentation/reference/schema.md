# JSON schema (aimee.schema.json)

The complete machine-readable contract for `.aimee.toml`, published at the repository root as `aimee.schema.json`. Every key, type, default, and enum value in the configuration model is specified there — including the `[anda]` section and inline provider definitions.

## Using it

**In an editor:** point JSON schema validation at the file for autocomplete and validation while editing TOML (via a TOML-language-server tap):

```json
{
  "taplo": {
    "schema": "https://raw.githubusercontent.com/swcstudiospace/omegaloops/main/aimee.schema.json"
  }
}
```

**In CI:** validate configs before they ship:

```bash
python3 -c "import json,yaml" 2>/dev/null || true
npx ajv-cli compile -s aimee.schema.json --spec=draft2020 || true
```

(Any draft-2020-12 validator works; schemars generated the schema.)

## What it covers

* Top-level keys: `services_url`, `restricted`, `tool_timeout_secs`, `subagents`, `research_subagent`, `use_aimee_committer`, failure budgets
* Sections: `[reasoning]`, `[retry]`, `[http]`, `[compact]`, `[updates]`
* `[anda]`: full pathway/KIP configuration incl. eternal modes (`local` | `ic_oss` | `canister` | `s3`)
* `[[providers]]`: inline provider entries with wire-protocol enums

## Maintenance notes

The schema is **generated** by `schemars` derives on the config types (`crates/aimee_config`). Don't hand-edit the published JSON — change the Rust types and regenerate, so code, defaults, and schema never drift apart.

Human-readable key documentation: [Config reference](config.md). Resolution rules: [Configuration model](../concepts/configuration.md).

## See also

* [Config reference (.aimee.toml)](config.md)
* [gRPC contract](proto.md)
* [Environment variables](env-vars.md)

<!-- sources: aimee.schema.json, crates/aimee_config/src/config.rs -->
