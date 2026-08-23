#!/usr/bin/env zsh

# Configuration variables for aimee plugin
# Using typeset to keep variables local to plugin scope and prevent public exposure

typeset -h _AIMEE_BIN="${AIMEE_BIN:-aimee}"
typeset -h _AIMEE_CONVERSATION_PATTERN=":"
typeset -h _AIMEE_MAX_COMMIT_DIFF="${AIMEE_MAX_COMMIT_DIFF:-100000}"

typeset -h _AIMEE_COMMANDS=""

# Hidden variables to be used only via the AimeeCLI
typeset -h _AIMEE_CONVERSATION_ID
typeset -h _AIMEE_ACTIVE_AGENT

# Previous conversation ID for :conversation - (like cd -)
typeset -h _AIMEE_PREVIOUS_CONVERSATION_ID

# Session-scoped model and provider overrides (set via :model / :m).
# When non-empty, these are passed as --model / --provider to every aimee
# invocation for the lifetime of the current shell session.
typeset -h _AIMEE_SESSION_MODEL
typeset -h _AIMEE_SESSION_PROVIDER

# Session-scoped reasoning effort override (set via :reasoning-effort / :re).
# When non-empty, exported as AIMEE_REASONING__EFFORT for every aimee invocation.
typeset -h _AIMEE_SESSION_REASONING_EFFORT

# Terminal context capture settings
# Master switch for terminal context capture (preexec/precmd hooks)
typeset -h _AIMEE_TERM="${AIMEE_TERM:-true}"
# Maximum number of commands to keep in the ring buffer (metadata: cmd + exit code)
typeset -h _AIMEE_TERM_MAX_COMMANDS="${AIMEE_TERM_MAX_COMMANDS:-5}"
# OSC 133 semantic prompt marker emission: "auto", "on", or "off"
typeset -h _AIMEE_TERM_OSC133="${AIMEE_TERM_OSC133:-auto}"
# Ring buffer arrays for context capture
typeset -ha _AIMEE_TERM_COMMANDS=()
typeset -ha _AIMEE_TERM_EXIT_CODES=()
typeset -ha _AIMEE_TERM_TIMESTAMPS=()
