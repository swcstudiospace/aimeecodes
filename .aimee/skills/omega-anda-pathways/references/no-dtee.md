# Anda dTEE is not a local workspace runtime

Do not invent a dTEE/TEE runtime for `/goal` agents or `aimee pod up`.

- People say “iclabs”; correct to **ldclabs**.
- Grep of `aimee_anda` / `aimee_anda_icp` / `anda-bot` found no dTEE API.
- ldclabs TEE lives in **`ldclabs/anda-cloud`** and **`ldclabs/ic-tee`** (ICP), not a Docker/devcontainer hook.
- In-tree memory path: Nexus `:8091` + `aimee_anda` pathways / KIP / eternal local receipts.
- Agent sandboxes are **Docker pods** (`aimee pod`, skill `omega-loops-cli`). `pod doctor` must report dTEE missing.
- Do not stub a TEE crate or claim pods are confidential execution.
