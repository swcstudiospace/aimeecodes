#!/usr/bin/env zsh

# Core action handlers for basic aimee operations

# Action handler: Start a new conversation
function _aimee_action_new() {
    local input_text="$1"
    
    # Clear conversation and save as previous (like cd -)
    _aimee_clear_conversation
    _AIMEE_ACTIVE_AGENT="aimee"
    
    echo
    
    # If input_text is provided, send it to the new conversation
    if [[ -n "$input_text" ]]; then
        # Generate new conversation ID and switch to it
        local new_id=$($_AIMEE_BIN conversation new)
        _aimee_switch_conversation "$new_id"
        
        # Execute the aimee command with the input text
        _aimee_exec_interactive -p "$input_text" --cid "$_AIMEE_CONVERSATION_ID"
        
        # Start background sync job if enabled and not already running
        _aimee_start_background_sync
        # Start background update check
        _aimee_start_background_update
    else
        # Only show banner if no input text (starting fresh conversation)
        _aimee_exec banner
    fi
}

# Action handler: Show session info
function _aimee_action_info() {
    echo
    if [[ -n "$_AIMEE_CONVERSATION_ID" ]]; then
        _aimee_exec info --cid "$_AIMEE_CONVERSATION_ID"
    else
        _aimee_exec info
    fi
}

# Action handler: Dump conversation
function _aimee_action_dump() {
    local input_text="$1"
    if [[ "$input_text" == "html" ]]; then
        _aimee_handle_conversation_command "dump" "--html"
    else
        _aimee_handle_conversation_command "dump"
    fi
}

# Action handler: Compact conversation
function _aimee_action_compact() {
    _aimee_handle_conversation_command "compact"
}

# Action handler: Retry last message
function _aimee_action_retry() {
    _aimee_handle_conversation_command "retry"
}

# Action handler: Show available commands (mirrors :help in the REPL)
function _aimee_action_help() {
    echo
    $_AIMEE_BIN list command
}

# Helper function to handle conversation commands that require an active conversation
function _aimee_handle_conversation_command() {
    local subcommand="$1"
    shift  # Remove first argument, remaining args become extra parameters
    
    echo
    
    # Check if AIMEE_CONVERSATION_ID is set
    if [[ -z "$_AIMEE_CONVERSATION_ID" ]]; then
        _aimee_log error "No active conversation. Start a conversation first or use :conversation to see existing ones"
        return 0
    fi
    
    # Execute the conversation command with conversation ID and any extra arguments
    _aimee_exec conversation "$subcommand" "$_AIMEE_CONVERSATION_ID" "$@"
}
