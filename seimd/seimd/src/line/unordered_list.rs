use regex::Regex;
use crate::line::{Line, LineProcessor};

pub struct UnorderedList {
    expressions: Vec<Regex>
}

impl Default for UnorderedList {
    fn default() -> Self {
        Self {
            expressions: vec![
                Regex::new(r"^\* (.*)").unwrap(),
                Regex::new(r"^- (.*)").unwrap(),
                Regex::new(r"^\+ (.*)").unwrap(),
            ]
        }
    }
}

impl LineProcessor for UnorderedList {
    fn process_line(&self, line: &str) -> Option<Line> {
        self.expressions.iter()
            .flat_map(|re| re.captures(line))
            .map(|caps| (&caps[1]).to_string())
            .map(|content| Line::UnorderedList(vec![content]))
            .next()
    }
}

#[cfg(test)]
mod tests {
    use crate::line::{Line, LineProcessor};
    use crate::line::unordered_list::UnorderedList;

    #[test]
    fn valid_list() {
        let ul = UnorderedList::default();
        let expected = Some(Line::UnorderedList(vec!["line".to_string()]));
        assert_eq!(expected, ul.process_line("- line"));
        assert_eq!(expected, ul.process_line("+ line"));
        assert_eq!(expected, ul.process_line("* line"));
    }

}