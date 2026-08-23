# Infrastructure — aimee_infra

The adapter layer: concrete implementations of every port the services need — filesystem, HTTP, authentication, MCP, gRPC, environment.

## `AimeeInfra`

The umbrella implementation type, constructed once at startup with `(cwd, config)`. Everything below it is swappable at the trait level, which is what makes the service layer testable without real networks or disks.

## Capabilities

| Concern | Implementation notes |
|---|---|
| File IO | Backs read/write/patch/remove tools; consistent anyhow context |
| HTTP | reqwest-based client with workspace TLS features |
| Auth | Provider credential flows incl. OAuth device login |
| MCP | Client for Model Context Protocol servers (rmcp) |
| gRPC | Tonic client for the `AimeeService` contract (`aimee.proto`) |
| Env / walker | Environment access and directory walking for discovery |

## Rules

Infra implements ports; it invents no policy. Timeouts, retries, and permissions are decided above it (app/services/config). External responses — HTTP bodies, MCP payloads, fetched files — enter the system here and are treated as untrusted from this point inward.

## Testing against it

Because services depend on traits rather than `AimeeInfra` directly, tests substitute fakes for any port. The shared fixture loaders in `aimee_test_kit` cover common file setups.

## See also

* [Services](services.md)
* [gRPC contract](../reference/proto.md)
* [Security model](../operations/security.md)

<!-- sources: AIMEE.md §4,§5, AGENTS.md libraries policy -->
