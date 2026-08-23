# Hooks / package boundary audit checklist

Use after mapping `packages/*`. Katalyst v0.2 applied this to `@katalyst/hooks`.

## Graph

- [ ] List every `from '../…'` under the package `src/`
- [ ] Resolve each path on disk; count MISSING
- [ ] Check `package.json` dependencies vs imports (`@scope/core`, native peers)
- [ ] Check for circular: does core also import this package?

## Barrel

- [ ] Root `index` only exports resolving modules
- [ ] No dual export of the same name (e.g. two `useFramework`)
- [ ] `exports` map includes every public subpath (`./react`, `./core`, …)
- [ ] Broken integrations live in `src/legacy/` + README, not root

## React hooks API

- [ ] No facade that returns unbound hook functions as `k.state` / `k.effect`
- [ ] Composition hook returns values (viewport, threads handle, config)
- [ ] Enhanced `useState` is 2-tuple; history is separate hook
- [ ] Debounced `useEffect` returns cleanup that clears timer **and** effect cleanup
- [ ] SSR: no `window`/`navigator`/`localStorage` in render-time initializers without guards

## Native / MT vertical

- [ ] Dynamic import of native; catch → Worker → main
- [ ] No hardcoded fake pool metrics as “health”
- [ ] Provider + thin hook; core re-exports or bridges one runtime
- [ ] Pure runtime tests without full React install

## Docs / identity

- [ ] `createX()` bootstrap + storage namespacing
- [ ] Version + stability stamp
- [ ] ARCHITECTURE.md with vertical diagram
- [ ] Honest claims (not “replaces React” unless reconciler is yours)

## Katalyst anchors (session)

- Repo: `/root/src/repos/katalyst`
- Hooks v0.2: `packages/hooks` — composition `useKatalyst`, MT in `src/multithreading/`
- Legacy: `packages/hooks/src/legacy/` (19 files)
- Summary: `packages/FRAMEWORK_FOUNDATIONS.md`
- Verify: `cd packages/hooks && deno test -A --no-check tests/`
