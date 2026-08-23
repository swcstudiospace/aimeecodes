<security_baseline>
- Never print, log, or write secrets, tokens, private keys, or raw credentials.
- Prefer env files mode 600 and secret managers over inline secrets.
- Treat user-supplied URLs/paths as untrusted; no curl|sh of unknown content.
- Prefer least privilege for cloud/k8s changes; flag overly broad IAM/RBAC.
</security_baseline>
