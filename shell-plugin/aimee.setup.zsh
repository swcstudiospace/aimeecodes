# !! Contents within this block are managed by 'aimee zsh setup' !!
# !! Do not edit manually - changes will be overwritten !!

# Add required zsh plugins if not already present
if [[ ! " ${plugins[@]} " =~ " zsh-autosuggestions " ]]; then
    plugins+=(zsh-autosuggestions)
fi
if [[ ! " ${plugins[@]} " =~ " zsh-syntax-highlighting " ]]; then
    plugins+=(zsh-syntax-highlighting)
fi

# Load aimee shell plugin (commands, completions, keybindings) if not already loaded
if [[ -z "$_AIMEE_PLUGIN_LOADED" ]]; then
    eval "$(aimee zsh plugin)"
fi

# Load aimee shell theme (prompt with AI context) if not already loaded
if [[ -z "$_AIMEE_THEME_LOADED" ]]; then
    eval "$(aimee zsh theme)"
fi
