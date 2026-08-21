#!/usr/bin/env zsh

# Enable prompt substitution for RPROMPT
setopt PROMPT_SUBST

# Model and agent info with token count
# Fully formatted output directly from Rust
# Returns ZSH-formatted string ready for use in RPROMPT
function _aimee_prompt_info() {
    local aimee_bin="${_AIMEE_BIN:-${AIMEE_BIN:-aimee}}"
    
    # Get fully formatted prompt from aimee (single command).
    # Pass session model/provider as CLI flags when set so the rprompt
    # reflects the active session override rather than global config.
    local -a aimee_cmd
    aimee_cmd=("$aimee_bin")
    aimee_cmd+=(zsh rprompt)
    [[ -n "$_AIMEE_SESSION_MODEL" ]] && local -x AIMEE_SESSION__MODEL_ID="$_AIMEE_SESSION_MODEL"
    [[ -n "$_AIMEE_SESSION_PROVIDER" ]] && local -x AIMEE_SESSION__PROVIDER_ID="$_AIMEE_SESSION_PROVIDER"
    [[ -n "$_AIMEE_SESSION_REASONING_EFFORT" ]] && local -x AIMEE_REASONING__EFFORT="$_AIMEE_SESSION_REASONING_EFFORT"
    _AIMEE_CONVERSATION_ID=$_AIMEE_CONVERSATION_ID _AIMEE_ACTIVE_AGENT=$_AIMEE_ACTIVE_AGENT COLUMNS=$COLUMNS "${aimee_cmd[@]}" 2>/dev/null
}

# Right prompt: agent and model with token count (uses single aimee prompt command)
# Set RPROMPT if empty, otherwise append to existing value
if [[ -z "$_AIMEE_THEME_LOADED" ]]; then
    RPROMPT='$(_aimee_prompt_info)'"${RPROMPT:+ ${RPROMPT}}"
fi
