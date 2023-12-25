use super::Line;
use crate::line::LineProcessor;

pub struct Empty;

impl LineProcessor for Empty {
    fn process_line(&self, line: &str) -> Option<Line> {
        if line.trim().is_empty() {
            Some(Line::Empty)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line::LineProcessor;

    #[test]
    fn empty_line() {
        assert_eq!(Some(Line::Empty), Empty.process_line(""));
        assert_eq!(Some(Line::Empty), Empty.process_line("     "));
        assert_eq!(Some(Line::Empty), Empty.process_line("\t"));
    }

    #[test]
    fn non_empty_line() {
        assert_eq!(None, Empty.process_line("class: bold, open"));
    }
}
