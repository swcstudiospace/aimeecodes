#!/usr/bin/env zsh

# Configuration action handlers (agent, provider, model, tools, skill)

# Action handler: Select agent
function _omega_action_agent() {
    local input_text="$1"
    
    echo
    
    # If an agent ID is provided directly, use it
    if [[ -n "$input_text" ]]; then
        local agent_id="$input_text"
        
        # Validate that the agent exists (skip header line)
        local agent_exists=$($_OMEGA_BIN list agents --porcelain 2>/dev/null | tail -n +2 | grep -q "^${agent_id}\b" && echo "true" || echo "false")
        if [[ "$agent_exists" == "false" ]]; then
            _omega_log error "Agent '\033[1m${agent_id}\033[0m' not found"
            return 0
        fi
        
        # Set the agent as active
        _OMEGA_ACTIVE_AGENT="$agent_id"
        
        # Print log about agent switching
        _omega_log success "Switched to agent \033[1m${agent_id}\033[0m"
        
        return 0
    fi
    
    # Use omega select agent for interactive picking
    local agent_id
    agent_id=$(_omega_select_with_query "$input_text" agent)
    
    if [[ -n "$agent_id" ]]; then
        _OMEGA_ACTIVE_AGENT="$agent_id"
        _omega_log success "Switched to agent \033[1m${agent_id}\033[0m"
    fi
}

# Action handler: Select model for the current session only.
# When the selected model belongs to a different provider, switches it first.
function _omega_action_model() {
    local input_text="$1"
    echo

    local model_id provider_id
    if _omega_select_model_pair_global "$input_text"; then
        model_id="${reply[1]}"
        provider_id="${reply[2]}"
        _omega_exec config set model "$provider_id" "$model_id"
    fi
}

# Action handler: Select model for commit message generation
# Calls `omega config set commit <provider_id> <model_id>` on selection.
function _omega_action_commit_model() {
    local input_text="$1"
    echo

    local model_id provider_id
    if _omega_select_model_pair "$input_text"; then
        model_id="${reply[1]}"
        provider_id="${reply[2]}"
        _omega_exec config set commit "$provider_id" "$model_id"
    fi
}

# Action handler: Select model for command suggestion generation
# Calls `omega config set suggest <provider_id> <model_id>` on selection.
function _omega_action_suggest_model() {
    local input_text="$1"
    echo

    local model_id provider_id
    if _omega_select_model_pair "$input_text"; then
        model_id="${reply[1]}"
        provider_id="${reply[2]}"
        _omega_exec config set suggest "$provider_id" "$model_id"
    fi
}

# Action handler: Sync workspace for codebase search
function _omega_action_sync() {
    echo
    # Use _omega_exec_interactive so that the consent prompt (and any other
    # interactive prompts) can access /dev/tty even though ZLE owns the
    # terminal's stdin/stdout pipes.
    # --init initializes the workspace first if it has not been set up yet
    _omega_exec_interactive workspace sync --init
}

# Action handler: inits workspace for codebase search
function _omega_action_sync_init() {
    echo
    # Use _omega_exec_interactive so that the consent prompt can access /dev/tty
    _omega_exec_interactive workspace init
}

# Action handler: Show sync status of workspace files
function _omega_action_sync_status() {
    echo
    _omega_exec workspace status "."
}

# Action handler: Show workspace info with sync details
function _omega_action_sync_info() {
    echo
    _omega_exec workspace info "."
}

# Action handler: Select model for the current session only.
# Sets _OMEGA_SESSION_MODEL and _OMEGA_SESSION_PROVIDER in the shell environment
# so that every subsequent omega invocation uses those values via --model /
# --provider flags without touching the permanent global configuration.
function _omega_action_session_model() {
    local input_text="$1"
    echo

    if _omega_select_model_pair "$input_text"; then
        _OMEGA_SESSION_MODEL="${reply[1]}"
        _OMEGA_SESSION_PROVIDER="${reply[2]}"
        _omega_log success "Session model set to \033[1m${_OMEGA_SESSION_MODEL}\033[0m (provider: \033[1m${_OMEGA_SESSION_PROVIDER}\033[0m)"
    fi
}

# Action handler: Reload config by resetting all session-scoped overrides.
# Clears _OMEGA_SESSION_MODEL, _OMEGA_SESSION_PROVIDER, and
# _OMEGA_SESSION_REASONING_EFFORT so that every subsequent omega invocation
# falls back to the permanent global configuration.
function _omega_action_config_reload() {
    echo

    if [[ -z "$_OMEGA_SESSION_MODEL" && -z "$_OMEGA_SESSION_PROVIDER" && -z "$_OMEGA_SESSION_REASONING_EFFORT" ]]; then
        _omega_log info "No session overrides active (already using global config)"
        return 0
    fi

    _OMEGA_SESSION_MODEL=""
    _OMEGA_SESSION_PROVIDER=""
    _OMEGA_SESSION_REASONING_EFFORT=""

    _omega_log success "Session overrides cleared — using global config"
}

# Action handler: Select reasoning effort for the current session only.
# Sets _OMEGA_SESSION_REASONING_EFFORT in the shell environment so that
# every subsequent omega invocation uses the selected value via the
# OMEGA_REASONING__EFFORT env var without modifying the permanent config.
function _omega_action_reasoning_effort() {
    local input_text="$1"
    echo

    local selected
    selected=$(_omega_select_with_query "$input_text" reasoning-effort)

    if [[ -n "$selected" ]]; then
        _OMEGA_SESSION_REASONING_EFFORT="$selected"
        _omega_log success "Session reasoning effort set to \033[1m${selected}\033[0m"
    fi
}

# Action handler: Set reasoning effort in global config.
# Calls `omega config set reasoning-effort <effort>` on selection,
# writing the chosen effort level permanently to ~/omega/.omega.toml.
function _omega_action_config_reasoning_effort() {
    local input_text="$1"
    echo

    local selected
    selected=$(_omega_select_with_query "$input_text" reasoning-effort)

    if [[ -n "$selected" ]]; then
        _omega_exec config set reasoning-effort "$selected"
    fi
}

# Action handler: Show config list
function _omega_action_config() {
    echo
    _omega_exec config list
}

# Action handler: Open the global omega config file in an editor
function _omega_action_config_edit() {
    echo

    # Determine editor in order of preference: OMEGA_EDITOR > EDITOR > nano
    local editor_cmd="${OMEGA_EDITOR:-${EDITOR:-nano}}"

    # Validate editor exists
    if ! command -v "${editor_cmd%% *}" &>/dev/null; then
        _omega_log error "Editor not found: $editor_cmd (set OMEGA_EDITOR or EDITOR)"
        return 1
    fi

    # Resolve config file path via the omega binary (honours OMEGA_CONFIG,
    # new ~/.omega path, and legacy ~/omega fallback automatically)
    local config_file
    config_file=$($_OMEGA_BIN config path 2>/dev/null)
    if [[ -z "$config_file" ]]; then
        _omega_log error "Failed to resolve config path from '$_OMEGA_BIN config path'"
        return 1
    fi

    local config_dir
    config_dir=$(dirname "$config_file")

    # Ensure the config directory exists
    if [[ ! -d "$config_dir" ]]; then
        mkdir -p "$config_dir" || {
            _omega_log error "Failed to create $config_dir directory"
            return 1
        }
    fi

    # Create the config file if it does not yet exist
    if [[ ! -f "$config_file" ]]; then
        touch "$config_file" || {
            _omega_log error "Failed to create $config_file"
            return 1
        }
    fi

    # Open editor with its own TTY session
    (eval "$editor_cmd '$config_file'" </dev/tty >/dev/tty 2>&1)
    local exit_code=$?

    if [[ $exit_code -ne 0 ]]; then
        _omega_log error "Editor exited with error code $exit_code"
    fi

    _omega_reset
}

# Action handler: Show tools
function _omega_action_tools() {
    echo
    # Ensure OMEGA_ACTIVE_AGENT always has a value, default to "omega"
    local agent_id="${_OMEGA_ACTIVE_AGENT:-omega}"
    _omega_exec list tools "$agent_id"
}

# Action handler: Show skills
function _omega_action_skill() {
    echo
    _omega_exec list skill
}
