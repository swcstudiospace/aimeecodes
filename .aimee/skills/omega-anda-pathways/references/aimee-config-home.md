# Enable Anda on the live Aimee home

Live file on this VPS: **`~/.forge/.aimee.toml`**.

`ConfigReader` uses the first existing of `aimee`, `.aimee`, `omega`, `.omega`, `forge`, `.forge`. **Do not `mkdir ~/.aimee`** — it wins over `.forge` and the SuperGrok session vanishes.

```toml
[anda]
enabled = true
kip_enabled = true
nexus_url = "http://127.0.0.1:8091"
eternal_enabled = true
eternal_mode = "local"
eternal_label_prefix = "aimee"
log_responses = true
log_turn_end = true
hard_fail = false
```

`aimee pod doctor` probes nexus as `anda kip` and must still report `anda dTEE missing`. Crates are `aimee_anda` / `aimee_anda_icp`. See `references/no-dtee.md`.
