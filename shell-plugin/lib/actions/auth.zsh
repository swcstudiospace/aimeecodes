#!/usr/bin/env zsh

# Authentication action handlers

# Action handler: Login to provider
function _omega_action_login() {
    local input_text="$1"
    echo

    local provider
    provider=$(_omega_select_with_query "$input_text" provider)

    if [[ -n "$provider" ]]; then
        _omega_exec_interactive provider login "$provider"
    fi
}

# Action handler: SuperGrok / SuperGrok Heavy OAuth device login (no API key)
function _omega_action_supergrok() {
    echo
    _omega_exec_interactive provider login xai_oauth
}

# Action handler: Logout from provider
function _omega_action_logout() {
    local input_text="$1"
    echo

    local provider
    provider=$(_omega_select_with_query "$input_text" provider --configured)

    if [[ -n "$provider" ]]; then
        _omega_exec provider logout "$provider"
    fi
}
