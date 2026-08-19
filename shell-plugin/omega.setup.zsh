# !! Contents within this block are managed by 'omega zsh setup' !!
# !! Do not edit manually - changes will be overwritten !!

# Add required zsh plugins if not already present
if [[ ! " ${plugins[@]} " =~ " zsh-autosuggestions " ]]; then
    plugins+=(zsh-autosuggestions)
fi
if [[ ! " ${plugins[@]} " =~ " zsh-syntax-highlighting " ]]; then
    plugins+=(zsh-syntax-highlighting)
fi

# Load omega shell plugin (commands, completions, keybindings) if not already loaded
if [[ -z "$_OMEGA_PLUGIN_LOADED" ]]; then
    eval "$(omega zsh plugin)"
fi

# Load omega shell theme (prompt with AI context) if not already loaded
if [[ -z "$_OMEGA_THEME_LOADED" ]]; then
    eval "$(omega zsh theme)"
fi
