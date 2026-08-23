---
name: tailscale-serve-host-edge
description: "Use when exposing apps on Tailscale HTTPS paths or k3s edge."
version: 1.0.0
author: Hermes Agent
license: MIT
metadata:
  hermes:
    tags: [tailscale, serve, nginx, k3s, ingress, hosting, edge, vps]
    related_skills: [hermes-linear-kanban-sync, hermes-agent]
    created_by: agent
---

# Tailscale Serve + host edge (multi-app :443)

## When to use

- User wants private Tailscale URLs **without typing `:9456` / `:8800`**
- Multiple local services on one node (Hermes dashboard, Vikunja, OpenClaw, …)
- Single-node k3s with **host nginx** keeping public `:80/:443` and ingress-nginx on NodePorts
- RAM-tight VPS (~15 GiB) already running Hermes/agents

**Don't use for:** multi-AZ enterprise HA clusters (needs more nodes), public Funnel without explicit ask.

## Tailscale Serve facts (this host)

| Fact | Detail |
|------|--------|
| Machine FQDN | `srv1778002.hedgehog-mooneye.ts.net` (only valid HTTPS cert domain) |
| Nested subdomain certs | **Invalid** — `kanban.srv….ts.net` fails `tailscale cert` |
| Service VIP `svc:name` | Requires **tagged** node (`service hosts must be tagged nodes`) |
| Second hostname | Separate `tailscaled` + interactive `up --hostname=…` (not unattended) |
| `--set-path=/foo` | Public URL is `https://machine/foo`; **path is stripped** before proxy to backend |

### Current Serve map (reference)

```text
:443  /        → 127.0.0.1:9119   Hermes dashboard
:443  /kanban  → 127.0.0.1:3457   nginx rewriter → Vikunja :3456
:8443 /        → 127.0.0.1:18789  OpenClaw (etc.)
```

```bash
tailscale serve status
tailscale serve status --json
tailscale serve --bg --https=443 --set-path=/kanban http://127.0.0.1:3457
# disable a port: tailscale serve --https=9456 off
```

## Pattern: path app that needs site-root (SPA)

Apps like Vikunja emit absolute `/assets` and `/api` URLs.

1. Local app binds `127.0.0.1:<app>`
2. nginx on `127.0.0.1:<edge>` proxies to app at **`/`** (because Serve strips path)
3. nginx `sub_filter` rewrites browser-facing URLs to `/<path>/…`
4. Set app `PUBLICURL` / base URL to `https://<machine>.ts.net/<path>/`
5. Serve: `--set-path=/<path> http://127.0.0.1:<edge>`
6. Keep Hermes (or primary app) on `/`

**sub_filter rules:** only match known prefixes (`href="/`, `src="/`, `"/api/`, `"/assets/`, …). Never blanket `"/` — corrupts `"/>` in HTML. Optional undo: `'"/kanban/>' → '"/>'`.

Vikunja dual-track details: skill `hermes-linear-kanban-sync` → `references/vikunja-human-ui.md`.

## Pattern: single-node k3s + host nginx edge

Honest scope: **not** multi-control-plane HA. Prod-oriented single node.

```bash
# RAM headroom first
fallocate -l 8G /swapfile && chmod 600 /swapfile && mkswap /swapfile && swapon /swapfile
echo '/swapfile none swap sw 0 0' >> /etc/fstab
sysctl vm.swappiness=10

# k3s: do NOT steal host :80/:443
curl -sfL https://get.k3s.io -o /tmp/get-k3s.sh && chmod +x /tmp/get-k3s.sh
/tmp/get-k3s.sh server \
  --write-kubeconfig-mode 600 \
  --disable traefik \
  --disable servicelb \
  --secrets-encryption \
  --kubelet-arg='eviction-hard=memory.available<300Mi,nodefs.available<10%'

export KUBECONFIG=/etc/rancher/k3s/k3s.yaml
helm upgrade --install ingress-nginx ingress-nginx/ingress-nginx \
  -n ingress-nginx --create-namespace \
  --set controller.service.type=NodePort \
  --set controller.service.nodePorts.http=30080 \
  --set controller.service.nodePorts.https=30443 \
  --set controller.ingressClassResource.default=true --wait

helm upgrade --install cert-manager jetstack/cert-manager \
  -n cert-manager --create-namespace --set crds.enabled=true --wait
```

Host nginx include for k8s-hosted vhosts:

`/root/src/repos/k8s-platform/ingress-edge/nginx-k8s-backend.conf.inc`  
→ `proxy_pass http://127.0.0.1:30080` (+ ACME `/.well-known` to same).

Platform docs: `/root/src/repos/k8s-platform/README.md`.

### Shell pitfall

kubelet eviction args contain `<` — **quote** them or the shell treats them as redirections:

```bash
--kubelet-arg='eviction-hard=memory.available<300Mi,nodefs.available<10%'
```

### docker compose in agent terminals

Some agent runtimes treat `docker compose up -d` as long-lived and block — use `background=true`, or `docker restart` only when env is unchanged. Env changes need `compose up -d --force-recreate`.

## User preference (this operator)

- Prefer **portless** Tailscale URLs (`https://host/path`) over `https://host:PORT`
- Prefer a **`kanban.`-style name** when possible; if MagicDNS/cert blocks it, ship `/kanban` on 443 immediately and document the tag/second-node path for a true subdomain
- Do not clobber Hermes on `/` when adding apps

## Pitfalls

1. Binding a new service on TS `:443` `/` → knocks out Hermes
2. Assuming nested TS hostnames get certs
3. `svc:` serve without node tags
4. Path serve without strip-aware backend
5. k3s default Traefik fighting host nginx on 80/443
6. Installing full k8s on OOM host without swap / resource limits
7. Calling single-node k3s “enterprise multi-AZ HA”

## Verification

```bash
tailscale serve status
curl -fsSk -o /dev/null -w '%{http_code}\n' https://srv1778002.hedgehog-mooneye.ts.net/
curl -fsSk -o /dev/null -w '%{http_code}\n' https://srv1778002.hedgehog-mooneye.ts.net/kanban/
kubectl get nodes,ingressclass 2>/dev/null
curl -fsS -o /dev/null -w '%{http_code}\n' -H 'Host: k8s-hello.local' http://127.0.0.1/
```

## Support files

- `references/vikunja-kanban-path-443.md` — Vikunja on TS `:443` `/kanban` (strip + sub_filter)
- `references/k3s-single-node-host.md` — k3s install shape on this VPS

## Related

- `hermes-linear-kanban-sync` — Vikunja dual-track + connector ports (`:8792` not `:8788`)
- Host k8s manifests: `/root/src/repos/k8s-platform/`
