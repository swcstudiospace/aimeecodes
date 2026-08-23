---
name: framework-monorepo-streamline
description: "Use when streamlining framework monorepo packages."
version: 1.0.0
---

# Framework monorepo streamline

For **in-house frameworks** (React 19+, multi-package monorepos) when the user wants a constructive package review, boundary repair, or “streamline this framework” — not generic app feature work.

## When to use

- Review `packages/` of a custom framework
- Hooks/runtime that claim to replace or wrap React
- Broken imports after splitting packages out of a core package
- Multithreading / NAPI / WASM verticals next to React hooks
- First-framework foundations (bootstrap, stability stamps, subpath exports)

## Workflow

### 1. Map ownership before editing

```bash
# per-package file counts + entry points
for p in packages/*/; do echo "$(basename "$p"): $(find "$p" -name '*.ts' -o -name '*.tsx' | wc -l)"; done
# resolve relative imports that escape the package
# from packages/foo/src, ../components → packages/foo/components (often MISSING after extract)
```

Build a table: package → role → health (compiles? deps correct? duplicates?).

### 2. Detect extract-without-rewire (common failure)

Symptoms:
- Files moved to `packages/hooks` (etc.) still import `../components`, `../types`, `../integrations`
- Those paths resolved inside the **new** package, not `packages/core`
- Main barrel eagerly re-exports broken modules → whole package fails to load

**Fix pattern:**
1. Slim root entry to modules that **actually resolve**
2. Quarantine broken modules under `src/legacy/` with a README restore plan — do **not** export them
3. Either move hooks back next to providers, **or** depend on `@scope/core` and import public exports only
4. Avoid circular deps: leaf package (hooks) must not be required by core if core is also required by hooks — prefer hooks → nothing | optional native; core → hooks OK

### 3. Hooks runtime design rules (React)

| Do | Don't |
|----|--------|
| Named `use*` exports (lint + Rules of Hooks) | `const k = useX(); k.state(0)` bag of hook constructors |
| Composition hook returns **values / handles** | Composition hook returns nested factories that call hooks |
| Enhanced `useState` stays **2-tuple** React-compatible | 3-tuple / overloaded return that breaks drop-in use |
| History/undo as `useFrameworkState` opt-in | Always allocate history state |
| SSR-guard `window` / `navigator` / `localStorage` | Read DOM in initializer on server |
| Effect debounce/throttle **preserve cleanup** | `setTimeout(effect)` dropping effect cleanup |

Marketing claims (“Rust-powered React replacement”) must match architecture: JS wrappers over React + optional native NAPI is fine — say so.

### 4. Dual-export / naming collisions

Search for the same export name from two modules on the barrel (`useKatalyst` config stub + unified facade). Keep **one** public name; rename provider-config hooks (`useXConfig`) and deprecate stubs.

### 5. Native vertical pattern (optional binary)

```
native package (NAPI/WASM)  — optional peer
        ↓ dynamic import
runtime.ts  — never throw if missing
        ↓ Worker → main-thread fallback
Provider + thin useHook
        ↓
composition handle (.threads / .runtime)
```

- Fabricated metrics (magic multipliers) are not a vertical — delete or mark TODO
- Core provider should try shared runtime first, then legacy native path
- Deno: classic `new Worker(blobUrl)` may fail — try/catch → main fallback; tests use `--no-check` or `.ts` import extensions

### 6. Eight first-framework foundations (minimum bar)

When bootstrapping or repairing a framework package, introduce if missing:

1. `canUseDOM` / SSR guards  
2. `invariant` / `warning` / `deprecate`  
3. `createFramework()` bootstrap (appId, feature flags, storage namespace)  
4. Public `types.ts` + stability stamp (`experimental` \| `beta` \| `stable`)  
5. Subpath `exports` (`.`, `./core`, optional `./react`, feature slices)  
6. React-compatible primitive contracts  
7. Lint-friendly named hooks (optional namespace object of `use*` only)  
8. Always-on fallbacks for optional native deps  

### 7. Docs to leave behind

- `packages/FRAMEWORK_FOUNDATIONS.md` — monorepo ownership map  
- `packages/<pkg>/ARCHITECTURE.md` — vertical diagram + API sketch  
- `src/legacy/README.md` — what broke, intended home, restore steps  

### 8. Verification

- Unit-test pure runtime (bootstrap, fallback map/batch) without full React DOM if install is heavy  
- Assert every `package.json` `exports` target exists  
- Assert root barrel does not import `legacy/`  
- Deno: relative imports need `.ts` extensions; Worker may need module/main fallback  

## Pitfalls

- **Document-every-file first** on a 2k+ file monorepo — user usually wants constructive package review; confirm scope  
- **Eager import of all integrations** on the unified entry kills tree-shaking and fails the package if one path breaks  
- **Tests that mock the entire API** without importing the implementation — delete or rewrite against real modules  
- **core/hooks duplicates packages/hooks** — one source of truth; the other re-exports  
- **Claiming drop-in `import React from '@pkg/react'`** without `exports['./react']`  

## References

- `references/hooks-boundary-checklist.md` — quick audit checklist from Katalyst v0.2 pass  
