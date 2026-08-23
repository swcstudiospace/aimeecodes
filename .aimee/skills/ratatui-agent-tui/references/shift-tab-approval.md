# Shift+Tab approval cycle (Aimee TUI)

Confirm → Auto → Yolo. Type: `aimee_domain::ApprovalMode` (`static AtomicU8`, not `const`).

- rustyline: bind `KeyCode::BackTab` with `Modifiers::NONE` **and** `SHIFT`.
- Handler: `ApprovalMode::cycle()`, set a mutex flag, return `Cmd::Interrupt`.
- `ReadResult::ApprovalCycled` vs Ctrl+C `Continue` — distinguish with the flag.
- Auto/Yolo skip permission prompts and `should_continue` (“continue anyway?”) or swarms stall.
- Default Yolo matches unrestricted config. Title: `Approval mode: yolo`.
