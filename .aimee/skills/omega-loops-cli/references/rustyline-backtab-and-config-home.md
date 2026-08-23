# rustyline BackTab + Aimee config home

## BackTab

Bind `KeyEvent(KeyCode::BackTab, Modifiers::NONE)` **once**. A second `BackTab+SHIFT` bind panics rustyline radix_trie at `AimeeEditor::new` (`multiple-keys with the same bit representation`). That is `UI::init`, so `aimee banner` / `pod doctor` crash too. Handle YOLO in the prompt loop; do not add `AppCommand::ApprovalCycled`.

See also `ratatui-agent-tui` → `references/rustyline-backtab.md`.

## Config home

`ConfigReader` picks the first existing of `aimee`, `.aimee`, `omega`, `.omega`, `forge`, `.forge`. This VPS uses **`~/.forge/.aimee.toml`**. Never `mkdir ~/.aimee` — it steals `base_path` and drops the SuperGrok session. Put `[anda]` on the existing file.

## Git

`/root/src/repos/aimeecodes` may have **no `.git`** while `github.com/swcstudiospace/aimeecodes` is non-empty. Do not `git init` over that; ask clone-vs-attach.
