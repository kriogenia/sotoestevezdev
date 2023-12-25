use crate::line::{Line, LineProcessor};

pub struct Markup;

impl LineProcessor for Markup {
    fn process_line(&self, line: &str) -> Option<Line> {
        Some(Line::Markup(line.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::LineProcessor;

    #[test]
    fn wraps_line() {
        let line = "no matter what **this** _should work_";
        assert_eq!(
            Some(Line::Markup(line.to_string())),
            Markup.process_line(&line)
        );
    }
}
