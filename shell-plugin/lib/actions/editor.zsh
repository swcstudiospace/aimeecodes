#!/usr/bin/env zsh

# Editor and command suggestion action handlers

# Action handler: Open external editor for command composition
function _omega_action_editor() {
    local initial_text="$1"
    echo
    
    # Determine editor in order of preference: OMEGA_EDITOR > EDITOR > nano
    local editor_cmd="${OMEGA_EDITOR:-${EDITOR:-nano}}"
    
    # Validate editor exists
    if ! command -v "${editor_cmd%% *}" &>/dev/null; then
        _omega_log error "Editor not found: $editor_cmd (set OMEGA_EDITOR or EDITOR)"
        return 1
    fi
    
    # Create .omega directory if it doesn't exist
    local omega_dir=".omega"
    if [[ ! -d "$omega_dir" ]]; then
        mkdir -p "$omega_dir" || {
            _omega_log error "Failed to create .omega directory"
            return 1
        }
    fi
    
    # Create temporary file with git-like naming: OMEGA_EDITMSG.md
    local temp_file="${omega_dir}/OMEGA_EDITMSG.md"
    touch "$temp_file" || {
        _omega_log error "Failed to create temporary file"
        return 1
    }
    
    # Ensure cleanup on exit
    trap "rm -f '$temp_file'" EXIT INT TERM
    
    # Pre-populate with initial text if provided
    if [[ -n "$initial_text" ]]; then
        echo "$initial_text" > "$temp_file"
    fi
    
    # Open editor in subshell with its own TTY session
    (eval "$editor_cmd '$temp_file'" </dev/tty >/dev/tty 2>&1)
    local editor_exit_code=$?
    
    if [ $editor_exit_code -ne 0 ]; then
        _omega_log error "Editor exited with error code $editor_exit_code"
        _omega_reset
        return 1
    fi
    
    # Read and process content
    local content
    content=$(cat "$temp_file" | tr -d '\r')
    
    if [ -z "$content" ]; then
        _omega_log info "Editor closed with no content"
        BUFFER=""
        CURSOR=0
        zle reset-prompt
        return 0
    fi
    
    # Insert into buffer with : prefix
    BUFFER=": $content"
    CURSOR=${#BUFFER}
    
    zle reset-prompt
}

# Action handler: Generate shell command from natural language
# Usage: :? <description>
function _omega_action_suggest() {
    local description="$1"
    
    if [[ -z "$description" ]]; then
        _omega_log error "Please provide a command description"
        return 0
    fi
    
    echo

    # Generate the command
    local generated_command
    generated_command=$(FORCE_COLOR=true CLICOLOR_FORCE=1 _omega_exec suggest "$description")

    if [[ -n "$generated_command" ]]; then
        # Replace the buffer with the generated command
        BUFFER="$generated_command"
        CURSOR=${#BUFFER}
        zle reset-prompt
    else
        _omega_log error "Failed to generate command"
    fi
}
