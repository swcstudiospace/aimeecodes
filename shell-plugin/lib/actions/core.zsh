#!/usr/bin/env zsh

# Core action handlers for basic omega operations

# Action handler: Start a new conversation
function _omega_action_new() {
    local input_text="$1"
    
    # Clear conversation and save as previous (like cd -)
    _omega_clear_conversation
    _OMEGA_ACTIVE_AGENT="omega"
    
    echo
    
    # If input_text is provided, send it to the new conversation
    if [[ -n "$input_text" ]]; then
        # Generate new conversation ID and switch to it
        local new_id=$($_OMEGA_BIN conversation new)
        _omega_switch_conversation "$new_id"
        
        # Execute the omega command with the input text
        _omega_exec_interactive -p "$input_text" --cid "$_OMEGA_CONVERSATION_ID"
        
        # Start background sync job if enabled and not already running
        _omega_start_background_sync
        # Start background update check
        _omega_start_background_update
    else
        # Only show banner if no input text (starting fresh conversation)
        _omega_exec banner
    fi
}

# Action handler: Show session info
function _omega_action_info() {
    echo
    if [[ -n "$_OMEGA_CONVERSATION_ID" ]]; then
        _omega_exec info --cid "$_OMEGA_CONVERSATION_ID"
    else
        _omega_exec info
    fi
}

# Action handler: Dump conversation
function _omega_action_dump() {
    local input_text="$1"
    if [[ "$input_text" == "html" ]]; then
        _omega_handle_conversation_command "dump" "--html"
    else
        _omega_handle_conversation_command "dump"
    fi
}

# Action handler: Compact conversation
function _omega_action_compact() {
    _omega_handle_conversation_command "compact"
}

# Action handler: Retry last message
function _omega_action_retry() {
    _omega_handle_conversation_command "retry"
}

# Action handler: Show available commands (mirrors :help in the REPL)
function _omega_action_help() {
    echo
    $_OMEGA_BIN list command
}

# Helper function to handle conversation commands that require an active conversation
function _omega_handle_conversation_command() {
    local subcommand="$1"
    shift  # Remove first argument, remaining args become extra parameters
    
    echo
    
    # Check if OMEGA_CONVERSATION_ID is set
    if [[ -z "$_OMEGA_CONVERSATION_ID" ]]; then
        _omega_log error "No active conversation. Start a conversation first or use :conversation to see existing ones"
        return 0
    fi
    
    # Execute the conversation command with conversation ID and any extra arguments
    _omega_exec conversation "$subcommand" "$_OMEGA_CONVERSATION_ID" "$@"
}
