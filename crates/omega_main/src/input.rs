use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use omega_api::Environment;

use crate::editor::{OmegaEditor, ReadResult};
use crate::model::{AppCommand, OmegaCommandManager};
use crate::prompt::OmegaPrompt;
use crate::tracker;

/// Console implementation for handling user input via command line.
pub struct Console {
    command: Arc<OmegaCommandManager>,
    editor: Mutex<OmegaEditor>,
}

impl Console {
    /// Creates a new instance of `Console`.
    pub fn new(
        env: Environment,
        custom_history_path: Option<PathBuf>,
        command: Arc<OmegaCommandManager>,
    ) -> Self {
        let editor = Mutex::new(OmegaEditor::new(env, custom_history_path, command.clone()));
        Self { command, editor }
    }
}

impl Console {
    pub async fn prompt(&self, prompt: &mut OmegaPrompt) -> anyhow::Result<AppCommand> {
        loop {
            let mut omega_editor = self.editor.lock().unwrap();
            let user_input = omega_editor.prompt(prompt)?;

            drop(omega_editor);
            match user_input {
                ReadResult::Continue => continue,
                ReadResult::Exit => return Ok(AppCommand::Exit),
                ReadResult::Empty => continue,
                ReadResult::Success(text) => {
                    tracker::prompt(text.clone());
                    return self.command.parse(&text);
                }
            }
        }
    }

    /// Sets the buffer content for the next prompt
    pub fn set_buffer(&self, content: String) {
        let mut editor = self.editor.lock().unwrap();
        editor.set_buffer(content);
    }
}
