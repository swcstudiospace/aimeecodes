# OAuth PKCE checklist (MCP Python SDK 2.x)

## Provider surface

Implement `OAuthAuthorizationServerProvider`:

- `get_client` / `register_client`
- `authorize` → redirect URL with `code` + `state`
- `load_authorization_code` / `exchange_authorization_code`
- `load_refresh_token` / `exchange_refresh_token` (optional but useful)
- `load_access_token` / `revoke_token`

Wire only:

```python
MCPServer(..., auth=AuthSettings(...), auth_server_provider=provider)
# Do NOT also pass token_verifier
```

## PKCE

- Require `code_challenge` on authorize (S256)
- TokenHandler verifies `code_verifier` against stored challenge
- Single-use codes: pop on exchange; second exchange fails

## Token model

- `OAuthToken.token_type` must be `"Bearer"` (capital B)
- Opaque access tokens stored server-side with TTL
- `load_access_token(CONNECTOR_TOKEN)` → synthetic AccessToken for legacy curl

## DCR

- Enable `ClientRegistrationOptions(enabled=True, default_scopes=["mcp"], valid_scopes=["mcp"])`
- Force registered `client_secret = CONNECTOR_TOKEN` so operators have one secret

## Protected resource metadata

Route is **not** always `/.well-known/oauth-protected-resource`.  
For resource `https://host/mcp` expect:

`/.well-known/oauth-protected-resource/mcp`

## Tests

```python
from asgi_lifespan import LifespanManager
async with LifespanManager(app):
    async with AsyncClient(transport=ASGITransport(app=app), ...) as client:
        ...
```

Without lifespan: `RuntimeError: Task group is not initialized`.

## nginx

Proxy all of `/` to the connector so OAuth + well-known work — not only `/mcp`.
