# Hybrid rustyline + ratatui splash

When the agent loop is still **rustyline** (not a full ratatui app), the branded splash must not take the alternate screen.

## Pattern

1. Compose in a ratatui `Buffer` (`render_into` art, then tagline, then chips).
2. Walk `buf.area()` cells; emit SGR for `Color::Rgb` fg/bg + `Modifier::BOLD`.
3. `print!` the ANSI string; leave the cursor below the splash.
4. Rustyline starts as usual.

Chips (keyboard reminders, no mouse): inverted key + label on void.

```
 :omega  implement    :muse  plan    :sage  research
```

Key style: `fg=void, bg=accent, BOLD`. Label style: `fg=accent, bg=void, BOLD`.

## Omega Loops

- Art: pyfiglet `smslant` “OMEGA LOOPS” in `crates/omega_main/src/banner`
- Live path: `banner::display` → `print_ratatui_splash` → `buffer_to_ansi`
- Palette: `crates/omega_main/src/theme.rs`
- Tests: `cargo test -p omega_main --lib -- banner::`

## Dep

`ratatui = { version = "0.30", default-features = false, features = ["crossterm"] }`
