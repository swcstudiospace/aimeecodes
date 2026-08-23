# Single-node k3s on this VPS

Layout: `/root/src/repos/k8s-platform/`

## Installed shape

- k3s `v1.36.3+k3s1`, secrets encryption on
- Traefik + ServiceLB **disabled** (host nginx owns 80/443)
- ingress-nginx NodePorts **30080/30443**
- cert-manager + ClusterIssuers selfsigned / letsencrypt-staging / prod
- 8G `/swapfile` (host was ~2 GiB free, no swap)
- Demo: ns `demo` / host `k8s-hello.local` via host nginx → NodePort

## Edge model

```text
Client → host nginx :80/:443 → 127.0.0.1:30080 ingress-nginx → Service/Pod
```

Include: `ingress-edge/nginx-k8s-backend.conf.inc`

## Honest scope

Not multi-AZ HA. Add nodes before calling it enterprise. Keep workload requests tiny on 15 GiB shared with Hermes/IDEs.
