#!/usr/bin/env zsh

# Configuration variables for omega plugin
# Using typeset to keep variables local to plugin scope and prevent public exposure

typeset -h _OMEGA_BIN="${OMEGA_BIN:-omega}"
typeset -h _OMEGA_CONVERSATION_PATTERN=":"
typeset -h _OMEGA_MAX_COMMIT_DIFF="${OMEGA_MAX_COMMIT_DIFF:-100000}"

typeset -h _OMEGA_COMMANDS=""

# Hidden variables to be used only via the OmegaCLI
typeset -h _OMEGA_CONVERSATION_ID
typeset -h _OMEGA_ACTIVE_AGENT

# Previous conversation ID for :conversation - (like cd -)
typeset -h _OMEGA_PREVIOUS_CONVERSATION_ID

# Session-scoped model and provider overrides (set via :model / :m).
# When non-empty, these are passed as --model / --provider to every omega
# invocation for the lifetime of the current shell session.
typeset -h _OMEGA_SESSION_MODEL
typeset -h _OMEGA_SESSION_PROVIDER

# Session-scoped reasoning effort override (set via :reasoning-effort / :re).
# When non-empty, exported as OMEGA_REASONING__EFFORT for every omega invocation.
typeset -h _OMEGA_SESSION_REASONING_EFFORT

# Terminal context capture settings
# Master switch for terminal context capture (preexec/precmd hooks)
typeset -h _OMEGA_TERM="${OMEGA_TERM:-true}"
# Maximum number of commands to keep in the ring buffer (metadata: cmd + exit code)
typeset -h _OMEGA_TERM_MAX_COMMANDS="${OMEGA_TERM_MAX_COMMANDS:-5}"
# OSC 133 semantic prompt marker emission: "auto", "on", or "off"
typeset -h _OMEGA_TERM_OSC133="${OMEGA_TERM_OSC133:-auto}"
# Ring buffer arrays for context capture
typeset -ha _OMEGA_TERM_COMMANDS=()
typeset -ha _OMEGA_TERM_EXIT_CODES=()
typeset -ha _OMEGA_TERM_TIMESTAMPS=()
