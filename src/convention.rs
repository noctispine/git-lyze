use regex::Regex;

#[derive(Debug)]
pub struct ConventionBuilder {
    regex: Regex,
}

#[derive(Debug, Default)]
pub struct ParsedCommitInfo {
    pub type_: String,
    pub optional_scope: Option<String>,
    pub description: Option<String>,
}

const OPTIONAL_SCOPE_INDICATOR: &str = "optional_scope";

impl ConventionBuilder {
    pub fn build(example_commit_message: &str) -> ConventionBuilder {
        let indx = example_commit_message
            .find(OPTIONAL_SCOPE_INDICATOR)
            .expect("There must be an optional scope indicator");
        let indicators: (char, char) = (
            example_commit_message.as_bytes()[indx - 1] as char,
            example_commit_message.as_bytes()[OPTIONAL_SCOPE_INDICATOR.len() + indx] as char,
        );

        let regex_pattern = format!(r"^(.*?)(?:\{}(.*?)\{})?: (.*)$", indicators.0, indicators.1);

        ConventionBuilder {
            regex: Regex::new(&regex_pattern).unwrap(),
        }
    }

    pub fn construct_info(&self, message: String) -> Option<ParsedCommitInfo> {
        let captures = self.regex.captures(&message);

        match captures {
            Some(captures) => Some(ParsedCommitInfo {
                type_: captures.get(1).unwrap().as_str().to_string(),
                optional_scope: captures.get(2).map(|m| m.as_str().to_string()),
                description: captures.get(3).map(|m| m.as_str().to_string()),
            }),

            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_fully_described_commit_message() {
        let example_commit_message = String::from("type(optional_scope): description");
        let style_builder = ConventionBuilder::build(example_commit_message.as_str());
        let parsed_info = style_builder
            .construct_info(String::from("ci(frontend): build times"))
            .unwrap();
        assert_eq!(parsed_info.type_, "ci");
        assert_eq!(parsed_info.optional_scope.unwrap(), "frontend");
        assert_eq!(parsed_info.description.unwrap(), "build times");
    }

    #[test]
    fn can_construct_commit_message_without_optional_scope() {
        let example_commit_message = String::from("type(optional_scope): description");
        let style_builder = ConventionBuilder::build(example_commit_message.as_str());
        let parsed_info = style_builder
            .construct_info(String::from("ci: build times"))
            .unwrap();
        assert_eq!(parsed_info.type_, "ci");
        assert_eq!(parsed_info.optional_scope, None);
        assert_eq!(parsed_info.description.unwrap(), "build times");
    }

    #[test]
    fn should_skip_non_conventional_commit() {
        let example_commit_message = String::from("type(optional_scope): description");
        let style_builder = ConventionBuilder::build(example_commit_message.as_str());
        let parsed_info = style_builder.construct_info(String::from("init"));
        assert!(parsed_info.is_none());
    }
}
