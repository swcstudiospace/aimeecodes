# Rustyline prompt chips (every turn)

Splash chips vanish after the first reply. Keep slash agents visible by
exporting `chips_ansi() -> String` (one line, no trailing newline) and
**prepending it to `render_prompt_left`**.

Do not `println!` before `readline` — that scrolls a new chip row every
turn. Embed ANSI in the prompt: strip for rustyline `raw()`, keep for
`styled()`.

Key style: `fg=void, bg=accent, BOLD`. Label: `fg=accent, bg=void, BOLD`.

Omega Loops: `banner::chips_ansi` → `OmegaPrompt::render_prompt_left`.
Tests: `banner::tests::test_chips_ansi_is_single_line_with_omega` and
`prompt::tests::test_render_prompt_left` (locks `:omega`).
