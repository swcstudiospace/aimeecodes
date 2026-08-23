use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use aimee_api::Environment;

use crate::editor::{AimeeEditor, ReadResult};
use crate::model::{AimeeCommandManager, AppCommand};
use crate::prompt::AimeePrompt;
use crate::title_display::TitleDisplayExt;
use crate::tracker;

/// Console implementation for handling user input via command line.
pub struct Console {
    command: Arc<AimeeCommandManager>,
    editor: Mutex<AimeeEditor>,
}

impl Console {
    /// Creates a new instance of `Console`.
    pub fn new(
        env: Environment,
        custom_history_path: Option<PathBuf>,
        command: Arc<AimeeCommandManager>,
    ) -> Self {
        let editor = Mutex::new(AimeeEditor::new(env, custom_history_path, command.clone()));
        Self { command, editor }
    }
}

impl Console {
    pub async fn prompt(&self, prompt: &mut AimeePrompt) -> anyhow::Result<AppCommand> {
        loop {
            let mut aimee_editor = self.editor.lock().unwrap();
            let user_input = aimee_editor.prompt(prompt)?;

            drop(aimee_editor);
            match user_input {
                ReadResult::Continue => continue,
                ReadResult::Exit => return Ok(AppCommand::Exit),
                ReadResult::Empty => continue,
                ReadResult::ApprovalCycled(mode) => {
                    println!(
                        "{}",
                        aimee_domain::TitleFormat::info(format!(
                            "Approval mode: {}  (Shift+Tab to cycle confirm/auto/yolo)",
                            mode.label()
                        ))
                        .display()
                    );
                    continue;
                }
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
