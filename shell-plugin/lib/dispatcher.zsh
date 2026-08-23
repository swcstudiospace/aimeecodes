#!/usr/bin/env zsh

# Main command dispatcher and widget registration

# Action handler: Set active agent or execute command
# Flow:
# 1. Check if user_action is a CUSTOM command -> execute with `cmd` subcommand
# 2. If no input_text -> switch to agent (for AGENT type commands)
# 3. If input_text -> execute command with active agent context
function _aimee_action_default() {
    local user_action="$1"
    local input_text="$2"
    local command_type=""
    
    # Validate that the command exists in show-commands (if user_action is provided)
    if [[ -n "$user_action" ]]; then
        local commands_list=$(_aimee_get_commands)
        if [[ -n "$commands_list" ]]; then
            # Check if the user_action is in the list of valid commands and extract the row
            local command_row=$(echo "$commands_list" | grep "^${user_action}\b")
            if [[ -z "$command_row" ]]; then
                echo
                _aimee_log error "Command '\033[1m${user_action}\033[0m' not found"
                return 0
            fi
            
            # Extract the command type from the second field (TYPE column)
            # Format: "COMMAND_NAME    TYPE    DESCRIPTION"
            command_type=$(echo "$command_row" | awk '{print $2}')
            # Case-insensitive comparison using :l (lowercase) modifier
            if [[ "${command_type:l}" == "custom" ]]; then
                # Generate conversation ID if needed (don't track previous for auto-generation)
                if [[ -z "$_AIMEE_CONVERSATION_ID" ]]; then
                    local new_id=$($_AIMEE_BIN conversation new)
                    # Use helper but don't track previous for auto-generation
                    _AIMEE_CONVERSATION_ID="$new_id"
                fi
                
                echo
                # Execute custom command with execute subcommand
                if [[ -n "$input_text" ]]; then
                    _aimee_exec cmd execute --cid "$_AIMEE_CONVERSATION_ID" "$user_action" "$input_text"
                else
                    _aimee_exec cmd execute --cid "$_AIMEE_CONVERSATION_ID" "$user_action"
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
                _aimee_log error "Command '\033[1m${user_action}\033[0m' not found"
                return 0
            fi
            echo
            # Set the agent in the local variable
            _AIMEE_ACTIVE_AGENT="$user_action"
            _aimee_log info "\033[1;37m${_AIMEE_ACTIVE_AGENT:u}\033[0m \033[90mis now the active agent\033[0m"
        fi
        return 0
    fi
    
    # Generate conversation ID if needed (don't track previous for auto-generation)
    if [[ -z "$_AIMEE_CONVERSATION_ID" ]]; then
        local new_id=$($_AIMEE_BIN conversation new)
        # Use direct assignment here - no previous to track for auto-generation
        _AIMEE_CONVERSATION_ID="$new_id"
    fi
    
    echo
    
    # Only set the agent if user explicitly specified one
    if [[ -n "$user_action" ]]; then
        _AIMEE_ACTIVE_AGENT="$user_action"
    fi
    
    # Execute the aimee command directly with proper escaping
    _aimee_exec_interactive -p "$input_text" --cid "$_AIMEE_CONVERSATION_ID"
    
    # Start background sync job if enabled and not already running
    _aimee_start_background_sync
    # Start background update check
    _aimee_start_background_update
}

function aimee-accept-line() {
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
    #     crates/aimee_main/src/built_in_commands.json
    #     Add a new entry: {"command": "name", "description": "Description [alias: x]"}
    #
    # Naming convention: shell commands should follow Object-Action (e.g., provider-login).
    #
    # ZLE-dispatched Aimee commands bypass zsh preexec/precmd hooks, so emit
    # OSC 133 markers explicitly. Ghostty uses these markers to distinguish the
    # prompt from command output during window resize/reflow.
    _aimee_osc133_emit "B"
    _aimee_osc133_emit "C"
    
    # Dispatch to appropriate action handler using pattern matching
    case "$user_action" in
        new|n)
            _aimee_action_new "$input_text"
        ;;
        info|i)
            _aimee_action_info
        ;;
        dump|d)
            _aimee_action_dump "$input_text"
        ;;
        compact)
            _aimee_action_compact
        ;;
        retry|r)
            _aimee_action_retry
        ;;
        help)
            _aimee_action_help
        ;;
        agent|a)
            _aimee_action_agent "$input_text"
        ;;
        conversation|c)
            _aimee_action_conversation "$input_text"
        ;;
        conversation-tree|ct)
            _aimee_action_conversation_tree
        ;;
        config-model|cm)
            _aimee_action_model "$input_text"
        ;;
        model|m)
            _aimee_action_session_model "$input_text"
        ;;
        config-reload|cr|model-reset|mr)
            _aimee_action_config_reload
        ;;
        reasoning-effort|re)
            _aimee_action_reasoning_effort "$input_text"
        ;;
        config-reasoning-effort|cre)
            _aimee_action_config_reasoning_effort "$input_text"
        ;;
        config-commit-model|ccm)
            _aimee_action_commit_model "$input_text"
        ;;
        config-suggest-model|csm)
            _aimee_action_suggest_model "$input_text"
        ;;
        tools|t)
            _aimee_action_tools
        ;;
        config|env|e)
            _aimee_action_config
        ;;
        config-edit|ce)
            _aimee_action_config_edit
        ;;
        skill)
            _aimee_action_skill
        ;;
        edit|ed)
            _aimee_action_editor "$input_text"
            local action_status=$?
            _aimee_osc133_emit "D;$action_status"
            _aimee_osc133_emit "A"
            # Note: editor action intentionally modifies BUFFER and handles its own prompt reset
            return $action_status
        ;;
        commit)
            _aimee_action_commit "$input_text"
        ;;
        commit-preview)
            _aimee_action_commit_preview "$input_text"
            local action_status=$?
            _aimee_osc133_emit "D;$action_status"
            _aimee_osc133_emit "A"
            # Note: commit action intentionally modifies BUFFER and handles its own prompt reset
            return $action_status
        ;;
        suggest|s)
            _aimee_action_suggest "$input_text"
            local action_status=$?
            _aimee_osc133_emit "D;$action_status"
            _aimee_osc133_emit "A"
            # Note: suggest action intentionally modifies BUFFER and handles its own prompt reset
            return $action_status
        ;;
        clone)
            _aimee_action_clone "$input_text"
        ;;
        rename|rn)
            _aimee_action_rename "$input_text"
        ;;
        conversation-rename)
            _aimee_action_conversation_rename "$input_text"
        ;;
        copy)
            _aimee_action_copy
        ;;
        workspace-sync|sync)
            _aimee_action_sync
        ;;
        workspace-init|sync-init)
            _aimee_action_sync_init
        ;;
        workspace-status|sync-status)
            _aimee_action_sync_status
        ;;
        workspace-info|sync-info)
            _aimee_action_sync_info
        ;;
        provider-login|login|provider)
            _aimee_action_login "$input_text"
        ;;
        supergrok|xai-oauth|grok-oauth|supergrok-heavy)
            _aimee_action_supergrok
        ;;
        logout)
            _aimee_action_logout "$input_text"
        ;;
        *)
            _aimee_action_default "$user_action" "$input_text"
        ;;
    esac
    
    local action_status=$?
    _aimee_osc133_emit "D;$action_status"
    _aimee_osc133_emit "A"
    
    # Centralized reset after all actions complete
    # This ensures consistent prompt state without requiring each action to call _aimee_reset
    # Exceptions: editor, commit-preview, and suggest actions return early as they intentionally modify BUFFER
    _aimee_reset
    return $action_status
}
