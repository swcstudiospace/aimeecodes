#!/usr/bin/env zsh

# Key bindings and widget registration for omega plugin

# Register ZLE widgets
zle -N omega-accept-line
zle -N omega-completion

# Custom bracketed-paste handler that wraps dropped file paths in @[] syntax
# and fixes syntax highlighting after paste.
#
# Path detection and wrapping is delegated to `omega zsh format` (Rust) so
# that all parsing logic lives in one well-tested place.
function omega-bracketed-paste() {
    # Call the built-in bracketed-paste widget first
    zle .$WIDGET "$@"
    
    # Only auto-wrap when the line is a omega command (starts with ':').
    # This avoids mangling paths pasted into normal shell commands like
    # 'vim /some/path' or 'cat /some/path'.
    if [[ "$BUFFER" == :* ]]; then
        local formatted=$("$_OMEGA_BIN" zsh format --buffer "$BUFFER")
        if [[ -n "$formatted" && "$formatted" != "$BUFFER" ]]; then
            BUFFER="$formatted"
            CURSOR=${#BUFFER}
        fi
    fi
    
    # Explicitly redisplay the buffer to ensure paste content is visible
    # This is critical for large or multiline pastes
    zle redisplay
    
    # Reset the prompt to trigger syntax highlighting refresh
    # The redisplay before reset-prompt ensures the buffer is fully rendered
    zle reset-prompt
}

# Re-applied after zsh-vi-mode's `zvm_init` precmd hook, which rebuilds the
# main/viins/vicmd keymaps and otherwise silently clobbers these bindings.
function _omega_apply_keybindings() {
    zle -N bracketed-paste omega-bracketed-paste
    bindkey '^M' omega-accept-line
    bindkey '^J' omega-accept-line
    bindkey '^I' omega-completion
}

_omega_apply_keybindings

# Harmless no-op when zsh-vi-mode (jeffreytse/zsh-vi-mode) isn't loaded.
typeset -ga zvm_after_init_commands
zvm_after_init_commands+=('_omega_apply_keybindings')
