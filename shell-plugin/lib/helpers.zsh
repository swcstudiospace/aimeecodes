#!/usr/bin/env zsh

# Core utility functions for omega plugin

# Lazy loader for commands cache
# Loads the commands list only when first needed, avoiding startup cost
function _omega_get_commands() {
    if [[ -z "$_OMEGA_COMMANDS" ]]; then
        _OMEGA_COMMANDS="$(CLICOLOR_FORCE=0 $_OMEGA_BIN list commands --porcelain 2>/dev/null)"
    fi
    echo "$_OMEGA_COMMANDS"
}

# Helper function to execute omega commands consistently
# This ensures proper handling of special characters and consistent output
function _omega_exec() {
    local agent_id="${_OMEGA_ACTIVE_AGENT:-omega}"
    local -a cmd
    cmd=($_OMEGA_BIN --agent "$agent_id")

    # Expose terminal context arrays as US-separated (\x1F) env vars so that
    # the Rust TerminalContextService can read them via get_env_var.
    # ASCII Unit Separator (\x1F) is used instead of `:` because commands
    # can legitimately contain colons (URLs, port mappings, paths, etc.).
    # Use `local -x` so the variables are exported only to the child omega
    # process and do not leak into the caller's shell environment.
    if [[ "$_OMEGA_TERM" == "true" && ${#_OMEGA_TERM_COMMANDS} -gt 0 ]]; then
        # Join the ring-buffer arrays with the ASCII Unit Separator (\x1F).
        # We use IFS-based joining ("${arr[*]}") rather than ${(j.SEP.)arr} because
        # zsh does NOT expand $'...' ANSI-C escapes inside parameter expansion flags.
        local _old_ifs="$IFS" _sep=$'\x1f'
        IFS="$_sep"
        local -x _OMEGA_TERM_COMMANDS="${_OMEGA_TERM_COMMANDS[*]}"
        local -x _OMEGA_TERM_EXIT_CODES="${_OMEGA_TERM_EXIT_CODES[*]}"
        local -x _OMEGA_TERM_TIMESTAMPS="${_OMEGA_TERM_TIMESTAMPS[*]}"
        IFS="$_old_ifs"
    fi

    cmd+=("$@")
    [[ -n "$_OMEGA_SESSION_MODEL" ]] && local -x OMEGA_SESSION__MODEL_ID="$_OMEGA_SESSION_MODEL"
    [[ -n "$_OMEGA_SESSION_PROVIDER" ]] && local -x OMEGA_SESSION__PROVIDER_ID="$_OMEGA_SESSION_PROVIDER"
    [[ -n "$_OMEGA_SESSION_REASONING_EFFORT" ]] && local -x OMEGA_REASONING__EFFORT="$_OMEGA_SESSION_REASONING_EFFORT"
    "${cmd[@]}"
}

# Like _omega_exec but connects stdin/stdout to /dev/tty so that interactive
# prompts (rustyline, nucleo-picker, etc.) work correctly when omega is launched as a
# child of a ZLE widget. ZLE owns the terminal and replaces the process's
# stdin/stdout with its own pipes, so without this redirect any readline
# library would see a non-tty stdin and return EOF immediately.
# Do NOT use inside $(...) command substitutions - use _omega_exec instead.
function _omega_exec_interactive() {
    local agent_id="${_OMEGA_ACTIVE_AGENT:-omega}"
    local -a cmd
    cmd=($_OMEGA_BIN --agent "$agent_id")

    # Expose terminal context arrays as US-separated (\x1F) env vars so that
    # the Rust TerminalContextService can read them via get_env_var.
    # ASCII Unit Separator (\x1F) is used instead of `:` because commands
    # can legitimately contain colons (URLs, port mappings, paths, etc.).
    # Use `local -x` so the variables are exported only for the duration of
    # this function call (i.e. inherited by the child omega process) and do
    # not leak into the caller's shell environment.
    if [[ "$_OMEGA_TERM" == "true" && ${#_OMEGA_TERM_COMMANDS} -gt 0 ]]; then
        local _old_ifs="$IFS" _sep=$'\x1f'
        IFS="$_sep"
        local -x _OMEGA_TERM_COMMANDS="${_OMEGA_TERM_COMMANDS[*]}"
        local -x _OMEGA_TERM_EXIT_CODES="${_OMEGA_TERM_EXIT_CODES[*]}"
        local -x _OMEGA_TERM_TIMESTAMPS="${_OMEGA_TERM_TIMESTAMPS[*]}"
        IFS="$_old_ifs"
    fi

    cmd+=("$@")
    [[ -n "$_OMEGA_SESSION_MODEL" ]] && local -x OMEGA_SESSION__MODEL_ID="$_OMEGA_SESSION_MODEL"
    [[ -n "$_OMEGA_SESSION_PROVIDER" ]] && local -x OMEGA_SESSION__PROVIDER_ID="$_OMEGA_SESSION_PROVIDER"
    [[ -n "$_OMEGA_SESSION_REASONING_EFFORT" ]] && local -x OMEGA_REASONING__EFFORT="$_OMEGA_SESSION_REASONING_EFFORT"
    "${cmd[@]}" </dev/tty >/dev/tty
}

function _omega_select() {
    [[ -n "$_OMEGA_SESSION_MODEL" ]] && local -x OMEGA_SESSION__MODEL_ID="$_OMEGA_SESSION_MODEL"
    [[ -n "$_OMEGA_SESSION_PROVIDER" ]] && local -x OMEGA_SESSION__PROVIDER_ID="$_OMEGA_SESSION_PROVIDER"
    [[ -n "$_OMEGA_SESSION_REASONING_EFFORT" ]] && local -x OMEGA_REASONING__EFFORT="$_OMEGA_SESSION_REASONING_EFFORT"
    CLICOLOR_FORCE=0 $_OMEGA_BIN select "$@" </dev/tty 2>/dev/tty
}

function _omega_select_global() {
    CLICOLOR_FORCE=0 $_OMEGA_BIN select "$@" </dev/tty 2>/dev/tty
}

function _omega_select_with_query() {
    local query="$1"
    shift

    if [[ -n "$query" ]]; then
        _omega_select "$@" --query "$query"
    else
        _omega_select "$@"
    fi
}

function _omega_select_with_query_global() {
    local query="$1"
    shift

    if [[ -n "$query" ]]; then
        _omega_select_global "$@" --query "$query"
    else
        _omega_select_global "$@"
    fi
}

function _omega_select_model_pair() {
    local result
    result=$(_omega_select_with_query "$1" model)

    if [[ -z "$result" ]]; then
        reply=()
        return 1
    fi

    reply=("${(@f)result}")
    [[ ${#reply[@]} -ge 2 ]]
}

function _omega_select_model_pair_global() {
    local result
    result=$(_omega_select_with_query_global "$1" model)

    if [[ -z "$result" ]]; then
        reply=()
        return 1
    fi

    reply=("${(@f)result}")
    [[ ${#reply[@]} -ge 2 ]]
}

function _omega_reset() {
  # Print newlines equal to the current buffer display line count so that
  # ZLE's zrefresh() clears these padding lines instead of conversation
  # output. When BUFFER spans multiple display lines (newlines or terminal
  # wrap), ZLE tracks the multi-line state internally (olnct). On
  # reset-prompt, zrefresh() compares olnct (old multi-line count) against
  # nlnct (new count after clearing) and clears the delta lines. By
  # printing padding here, the cleared lines are blank ones we inserted
  # rather than the omega conversation output that precedes them.
  local pad="${BUFFERLINES:-1}" _i
  for ((_i=1; _i<pad; _i++)); do print; done

  # Clear buffer and reset cursor position
  BUFFER=""
  CURSOR=0
  # Force widget redraw and prompt reset
  zle -I
  zle reset-prompt
}

# Helper function to print messages with consistent formatting based on log level
# Usage: _omega_log <level> <message>
# Levels: error, info, success, warning, debug
# Color scheme matches crates/omega_main/src/title_display.rs
function _omega_log() {
    local level="$1"
    local message="$2"
    local timestamp="\033[90m[$(date '+%H:%M:%S')]\033[0m"
    
    case "$level" in
        error)
            # Category::Error - Red ⏺
            echo "\033[31m⏺\033[0m ${timestamp} \033[31m${message}\033[0m"
            ;;
        info)
            # Category::Info - White ⏺
            echo "\033[37m⏺\033[0m ${timestamp} \033[37m${message}\033[0m"
            ;;
        success)
            # Category::Action/Completion - Yellow ⏺
            echo "\033[33m⏺\033[0m ${timestamp} \033[37m${message}\033[0m"
            ;;
        warning)
            # Category::Warning - Bright yellow ⚠️
            echo "\033[93m⚠️\033[0m ${timestamp} \033[93m${message}\033[0m"
            ;;
        debug)
            # Category::Debug - Cyan ⏺ with dimmed text
            echo "\033[36m⏺\033[0m ${timestamp} \033[90m${message}\033[0m"
            ;;
        *)
            echo "${message}"
            ;;
    esac
}

# Helper function to check if a workspace is indexed
# Usage: _omega_is_workspace_indexed <workspace_path>
# Returns: 0 if workspace is indexed, 1 otherwise
function _omega_is_workspace_indexed() {
    local workspace_path="$1"
    $_OMEGA_BIN workspace info "$workspace_path" >/dev/null 2>&1
    return $?
}

# Start background sync job for current workspace if not already running
# Uses canonical path hash to identify workspace
function _omega_start_background_sync() {
    # Check if sync is enabled (default to true if not set)
    local sync_enabled="${OMEGA_SYNC_ENABLED:-true}"
    if [[ "$sync_enabled" != "true" ]]; then
        return 0
    fi

    # Get canonical workspace path
    local workspace_path=$(pwd -P)

    # Check if workspace is indexed before attempting sync
    {
        # Run sync once in background
        # Close all output streams immediately to prevent any flashing
        # Redirect stdin to /dev/null to prevent hanging if sync tries to read input
        exec >/dev/null 2>&1 </dev/null
        setopt NO_NOTIFY NO_MONITOR
        if ! _omega_is_workspace_indexed "$workspace_path"; then
            return 0
        fi
        # Should fail if sync-init or sync --init has not been performed even once
        $_OMEGA_BIN workspace sync "$workspace_path"
    } &!
}

# Start background update check if not already running
# Mirrors the background sync pattern to silently check for and apply updates
function _omega_start_background_update() {
    {
        # Run update check in background
        # Close all output streams immediately to prevent any flashing
        # Redirect stdin to /dev/null to prevent hanging
        exec >/dev/null 2>&1 </dev/null
        setopt NO_NOTIFY NO_MONITOR
        $_OMEGA_BIN update --no-confirm
    } &!
}

