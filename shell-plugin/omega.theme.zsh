#!/usr/bin/env zsh

# Enable prompt substitution for RPROMPT
setopt PROMPT_SUBST

# Model and agent info with token count
# Fully formatted output directly from Rust
# Returns ZSH-formatted string ready for use in RPROMPT
function _omega_prompt_info() {
    local omega_bin="${_OMEGA_BIN:-${OMEGA_BIN:-omega}}"
    
    # Get fully formatted prompt from omega (single command).
    # Pass session model/provider as CLI flags when set so the rprompt
    # reflects the active session override rather than global config.
    local -a omega_cmd
    omega_cmd=("$omega_bin")
    omega_cmd+=(zsh rprompt)
    [[ -n "$_OMEGA_SESSION_MODEL" ]] && local -x OMEGA_SESSION__MODEL_ID="$_OMEGA_SESSION_MODEL"
    [[ -n "$_OMEGA_SESSION_PROVIDER" ]] && local -x OMEGA_SESSION__PROVIDER_ID="$_OMEGA_SESSION_PROVIDER"
    [[ -n "$_OMEGA_SESSION_REASONING_EFFORT" ]] && local -x OMEGA_REASONING__EFFORT="$_OMEGA_SESSION_REASONING_EFFORT"
    _OMEGA_CONVERSATION_ID=$_OMEGA_CONVERSATION_ID _OMEGA_ACTIVE_AGENT=$_OMEGA_ACTIVE_AGENT COLUMNS=$COLUMNS "${omega_cmd[@]}" 2>/dev/null
}

# Right prompt: agent and model with token count (uses single omega prompt command)
# Set RPROMPT if empty, otherwise append to existing value
if [[ -z "$_OMEGA_THEME_LOADED" ]]; then
    RPROMPT='$(_omega_prompt_info)'"${RPROMPT:+ ${RPROMPT}}"
fi
