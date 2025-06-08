use strum::IntoEnumIterator;

use super::Command;

pub(super) fn run() -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("<strong>Commands</strong>".to_string());
    for (name, help) in
        Command::iter().filter_map(|c| c.help().map(|h| (c.to_string(), h.to_owned())))
    {
        let spaces: String = " ".repeat(12 - name.len());
        let line = format!("  <em>{name}</em>{spaces}{help}");
        lines.push(line);
    }
    lines
}
