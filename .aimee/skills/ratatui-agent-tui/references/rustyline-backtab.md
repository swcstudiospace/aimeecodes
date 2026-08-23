# rustyline BackTab: bind once

`KeyEvent(KeyCode::BackTab, Modifiers::NONE)` only.

A second `BackTab + SHIFT` bind panics rustyline `radix_trie` at `AimeeEditor::new` (`multiple-keys with the same bit representation`). That runs in `UI::init`, so `aimee banner` / `aimee pod doctor` die too.

`BackTab` already is Shift+Tab. Handle YOLO cycle in the prompt loop; do not add `AppCommand::ApprovalCycled`.
