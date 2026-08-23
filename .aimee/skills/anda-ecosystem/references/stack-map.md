# LDC Labs Anda stack (quick map)

## Layers
1. **Protocols** — KIP (memory language), Agent Protocols (identity/discourse)
2. **Anda DB** — embedded DB: B-Tree + BM25 + HNSW; object_store backends (fs/S3/GCS/Azure/memory); optional AES encryption via anda_object_store
3. **Cognitive Nexus** — KIP executor on Anda DB (concepts + propositions collections)
4. **Anda Brain** — NL formation / recall / sleep over KIP
5. **Anda engine/Bot** — agent runtime + product UX
6. **ic-oss** — ICP object storage for durable blobs/memory states

## Upstream
- https://github.com/ldclabs/KIP
- https://github.com/ldclabs/anda-db
- https://github.com/ldclabs/anda-brain
- https://github.com/ldclabs/anda
- https://github.com/ldclabs/anda-bot
- https://github.com/ldclabs/ic-oss
- https://github.com/ldclabs/agent-protocols
- https://anda.ai/

## Local offline docs
`/root/src/repos/ai-agency/knowledge/anda/` (READMEs + SPEC + anda_db_docs + brain_posts)

## Runtime on this VPS
- Rust nexus: `anda-nexus.service` :8091 `POST /kip` method `execute_kip` params `{"command":"..."}`
- Python Brain: `kip_memory.brain` formation/recall/maintenance
- Bridge MCP: :7790 tools `anda_brain_*`, `kip_*`
- Agno: toolbelt `anda_brain` + FileSystemKnowledge
