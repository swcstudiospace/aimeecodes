#!/usr/bin/env zsh

# Main command dispatcher and widget registration

# Action handler: Set active agent or execute command
# Flow:
# 1. Check if user_action is a CUSTOM command -> execute with `cmd` subcommand
# 2. If no input_text -> switch to agent (for AGENT type commands)
# 3. If input_text -> execute command with active agent context
function _omega_action_default() {
    local user_action="$1"
    local input_text="$2"
    local command_type=""
    
    # Validate that the command exists in show-commands (if user_action is provided)
    if [[ -n "$user_action" ]]; then
        local commands_list=$(_omega_get_commands)
        if [[ -n "$commands_list" ]]; then
            # Check if the user_action is in the list of valid commands and extract the row
            local command_row=$(echo "$commands_list" | grep "^${user_action}\b")
            if [[ -z "$command_row" ]]; then
                echo
                _omega_log error "Command '\033[1m${user_action}\033[0m' not found"
                return 0
            fi
            
            # Extract the command type from the second field (TYPE column)
            # Format: "COMMAND_NAME    TYPE    DESCRIPTION"
            command_type=$(echo "$command_row" | awk '{print $2}')
            # Case-insensitive comparison using :l (lowercase) modifier
            if [[ "${command_type:l}" == "custom" ]]; then
                # Generate conversation ID if needed (don't track previous for auto-generation)
                if [[ -z "$_OMEGA_CONVERSATION_ID" ]]; then
                    local new_id=$($_OMEGA_BIN conversation new)
                    # Use helper but don't track previous for auto-generation
                    _OMEGA_CONVERSATION_ID="$new_id"
                fi
                
                echo
                # Execute custom command with execute subcommand
                if [[ -n "$input_text" ]]; then
                    _omega_exec cmd execute --cid "$_OMEGA_CONVERSATION_ID" "$user_action" "$input_text"
                else
                    _omega_exec cmd execute --cid "$_OMEGA_CONVERSATION_ID" "$user_action"
                fi
                return 0
            fi
        fi
    fi
    
    # If input_text is empty, just set the active agent (only for AGENT type commands)
    if [[ -z "$input_text" ]]; then
        if [[ -n "$user_action" ]]; then
            if [[ "${command_type:l}" != "agent" ]]; then
                echo
                _omega_log error "Command '\033[1m${user_action}\033[0m' not found"
                return 0
            fi
            echo
            # Set the agent in the local variable
            _OMEGA_ACTIVE_AGENT="$user_action"
            _omega_log info "\033[1;37m${_OMEGA_ACTIVE_AGENT:u}\033[0m \033[90mis now the active agent\033[0m"
        fi
        return 0
    fi
    
    # Generate conversation ID if needed (don't track previous for auto-generation)
    if [[ -z "$_OMEGA_CONVERSATION_ID" ]]; then
        local new_id=$($_OMEGA_BIN conversation new)
        # Use direct assignment here - no previous to track for auto-generation
        _OMEGA_CONVERSATION_ID="$new_id"
    fi
    
    echo
    
    # Only set the agent if user explicitly specified one
    if [[ -n "$user_action" ]]; then
        _OMEGA_ACTIVE_AGENT="$user_action"
    fi
    
    # Execute the omega command directly with proper escaping
    _omega_exec_interactive -p "$input_text" --cid "$_OMEGA_CONVERSATION_ID"
    
    # Start background sync job if enabled and not already running
    _omega_start_background_sync
    # Start background update check
    _omega_start_background_update
}

function omega-accept-line() {
    # Save the original command for history
    local original_buffer="$BUFFER"
    
    # Parse the buffer first in parent shell context to avoid subshell issues
    local user_action=""
    local input_text=""
    
    # Check if the line starts with any of the supported patterns
    if [[ "$BUFFER" =~ "^:([a-zA-Z][a-zA-Z0-9_-]*)( (.*))?$" ]]; then
        # Action with or without parameters: :foo or :foo bar baz
        user_action="${match[1]}"
        # Only use match[3] if the second group (space + params) was actually matched
        if [[ -n "${match[2]}" ]]; then
            input_text="${match[3]}"
        else
            input_text=""
        fi
    elif [[ "$BUFFER" =~ "^: (.*)$" ]]; then
        # Default action with parameters: : something
        user_action=""
        input_text="${match[1]}"
    else
        # For non-:commands, use normal accept-line
        zle accept-line
        return
    fi
    
    # Add the original command to history before transformation
    print -s -- "$original_buffer"
    
    CURSOR=${#BUFFER}
    zle redisplay
    
    # Handle aliases - convert to their actual agent names
    case "$user_action" in
        ask)
            user_action="sage"
        ;;
        plan)
            user_action="muse"
        ;;
    esac
    
    # ⚠️  IMPORTANT: When adding a new command here, you MUST also update:
    #     crates/omega_main/src/built_in_commands.json
    #     Add a new entry: {"command": "name", "description": "Description [alias: x]"}
    #
    # Naming convention: shell commands should follow Object-Action (e.g., provider-login).
    #
    # ZLE-dispatched Omega commands bypass zsh preexec/precmd hooks, so emit
    # OSC 133 markers explicitly. Ghostty uses these markers to distinguish the
    # prompt from command output during window resize/reflow.
    _omega_osc133_emit "B"
    _omega_osc133_emit "C"
    
    # Dispatch to appropriate action handler using pattern matching
    case "$user_action" in
        new|n)
            _omega_action_new "$input_text"
        ;;
        info|i)
            _omega_action_info
        ;;
        dump|d)
            _omega_action_dump "$input_text"
        ;;
        compact)
            _omega_action_compact
        ;;
        retry|r)
            _omega_action_retry
        ;;
        help)
            _omega_action_help
        ;;
        agent|a)
            _omega_action_agent "$input_text"
        ;;
        conversation|c)
            _omega_action_conversation "$input_text"
        ;;
        conversation-tree|ct)
            _omega_action_conversation_tree
        ;;
        config-model|cm)
            _omega_action_model "$input_text"
        ;;
        model|m)
            _omega_action_session_model "$input_text"
        ;;
        config-reload|cr|model-reset|mr)
            _omega_action_config_reload
        ;;
        reasoning-effort|re)
            _omega_action_reasoning_effort "$input_text"
        ;;
        config-reasoning-effort|cre)
            _omega_action_config_reasoning_effort "$input_text"
        ;;
        config-commit-model|ccm)
            _omega_action_commit_model "$input_text"
        ;;
        config-suggest-model|csm)
            _omega_action_suggest_model "$input_text"
        ;;
        tools|t)
            _omega_action_tools
        ;;
        config|env|e)
            _omega_action_config
        ;;
        config-edit|ce)
            _omega_action_config_edit
        ;;
        skill)
            _omega_action_skill
        ;;
        edit|ed)
            _omega_action_editor "$input_text"
            local action_status=$?
            _omega_osc133_emit "D;$action_status"
            _omega_osc133_emit "A"
            # Note: editor action intentionally modifies BUFFER and handles its own prompt reset
            return $action_status
        ;;
        commit)
            _omega_action_commit "$input_text"
        ;;
        commit-preview)
            _omega_action_commit_preview "$input_text"
            local action_status=$?
            _omega_osc133_emit "D;$action_status"
            _omega_osc133_emit "A"
            # Note: commit action intentionally modifies BUFFER and handles its own prompt reset
            return $action_status
        ;;
        suggest|s)
            _omega_action_suggest "$input_text"
            local action_status=$?
            _omega_osc133_emit "D;$action_status"
            _omega_osc133_emit "A"
            # Note: suggest action intentionally modifies BUFFER and handles its own prompt reset
            return $action_status
        ;;
        clone)
            _omega_action_clone "$input_text"
        ;;
        rename|rn)
            _omega_action_rename "$input_text"
        ;;
        conversation-rename)
            _omega_action_conversation_rename "$input_text"
        ;;
        copy)
            _omega_action_copy
        ;;
        workspace-sync|sync)
            _omega_action_sync
        ;;
        workspace-init|sync-init)
            _omega_action_sync_init
        ;;
        workspace-status|sync-status)
            _omega_action_sync_status
        ;;
        workspace-info|sync-info)
            _omega_action_sync_info
        ;;
        provider-login|login|provider)
            _omega_action_login "$input_text"
        ;;
        supergrok|xai-oauth|grok-oauth|supergrok-heavy)
            _omega_action_supergrok
        ;;
        logout)
            _omega_action_logout "$input_text"
        ;;
        *)
            _omega_action_default "$user_action" "$input_text"
        ;;
    esac
    
    local action_status=$?
    _omega_osc133_emit "D;$action_status"
    _omega_osc133_emit "A"
    
    # Centralized reset after all actions complete
    # This ensures consistent prompt state without requiring each action to call _omega_reset
    # Exceptions: editor, commit-preview, and suggest actions return early as they intentionally modify BUFFER
    _omega_reset
    return $action_status
}
