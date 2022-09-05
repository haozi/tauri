use super::InvokeContext;
use crate::Runtime;
use serde::Deserialize;
use tauri_macros::{command_enum, module_command_handler, CommandModule};

/// The API descriptor.
#[command_enum]
#[derive(Deserialize, CommandModule)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  #[cmd(url_all, "url > all")]
  PathToFileURL { path: String },
}

impl Cmd {
  #[module_command_handler(url_all)]
  fn path_to_file_url<R: Runtime>(
    _context: InvokeContext<R>,
    path: String,
  ) -> super::Result<String> {
    let resolved_path = super::path::resolve(vec![path])?;
    Ok(
      url::Url::from_file_path(resolved_path)
        .map_err(|_| anyhow::anyhow!("Failed to parse `file:` URL from file path"))?
        .as_str()
        .to_string(),
    )
  }
}
