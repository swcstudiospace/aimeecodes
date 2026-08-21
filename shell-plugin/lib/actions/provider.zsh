#!/usr/bin/env zsh

# Provider selection action handlers

# Action handler: Select the provider for the current session.
# Sets _AIMEE_SESSION_PROVIDER in the shell environment so that every
# subsequent aimee invocation uses that provider via --provider flag
# without touching the permanent global configuration.
function _aimee_action_session_provider() {
    local input_text="$1"
    echo

    local selected
    selected=$(_aimee_select_with_query "$input_text" provider)

    if [[ -n "$selected" ]]; then
        _AIMEE_SESSION_PROVIDER="$selected"
        _aimee_log success "Session provider set to \033[1m${selected}\033[0m"
    fi
}
