#!/usr/bin/env zsh

# Authentication action handlers

# Action handler: Login to provider
function _aimee_action_login() {
    local input_text="$1"
    echo

    local provider
    provider=$(_aimee_select_with_query "$input_text" provider)

    if [[ -n "$provider" ]]; then
        _aimee_exec_interactive provider login "$provider"
    fi
}

# Action handler: SuperGrok / SuperGrok Heavy OAuth device login (no API key)
function _aimee_action_supergrok() {
    echo
    _aimee_exec_interactive provider login xai_oauth
}

# Action handler: Logout from provider
function _aimee_action_logout() {
    local input_text="$1"
    echo

    local provider
    provider=$(_aimee_select_with_query "$input_text" provider --configured)

    if [[ -n "$provider" ]]; then
        _aimee_exec provider logout "$provider"
    fi
}
