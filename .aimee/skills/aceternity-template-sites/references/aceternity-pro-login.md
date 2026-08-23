# Aceternity UI Pro — magic-link login (browser)

**Site:** `https://ui.aceternity.com`  
**Auth:** NextAuth · email provider id = **`nodemailer`** · also has `github`, `google`

## Why not only the UI form

Automated `browser_type` + “Sign In with Email” often leaves an empty field and no toast. The request may not fire. Use the NextAuth endpoints in the **same browser context** so cookies stick.

## Send magic link

```js
// Run in page console / browser_console on ui.aceternity.com
const csrf = (await fetch('/api/auth/csrf', { credentials: 'include' }).then(r => r.json())).csrfToken;
const body = new URLSearchParams({
  email: 'USER@example.com',
  csrfToken: csrf,
  callbackUrl: 'https://ui.aceternity.com/',
  json: 'true',
});
const res = await fetch('/api/auth/signin/nodemailer', {
  method: 'POST',
  credentials: 'include',
  headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
  body: body.toString(),
  redirect: 'follow',
});
// Expect final URL: .../api/auth/verify-request?provider=nodemailer&type=email
return { status: res.status, url: res.url };
```

Optional discover:

```js
await fetch('/api/auth/providers', { credentials: 'include' }).then(r => r.json());
// → { nodemailer: { type: 'email', signinUrl: '.../signin/nodemailer', ... }, github, google }
```

## Complete login

1. Tell user: check inbox for Aceternity magic link.
2. They paste the full URL (path `/api/auth/callback/nodemailer?...&token=...&email=...`).
3. `browser_navigate` to that URL **in the session that requested the link**.
4. Land on homepage; nav shows **Orders** + email.

## Verify Pro

```js
await fetch('/api/auth/session', { credentials: 'include' }).then(r => r.json());
// user.isAllAccess === true, user.isLifetime === true, licenseType e.g. "PERSONAL"
```

Useful pages once in: `/orders`, `/blocks`, `/components`, `/templates`, productized template docs/preview.

## Security

- Magic links are **single-use**. Do not log full tokens to long-lived files or public channels after use.
- Never invent passwords. Do not store session tokens in skills; only the flow.
