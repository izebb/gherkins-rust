use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keywords {
    // Structure
    Background,
    Feature,
    Scenario,
    ScenarioOutline,
    ScenarioTemplate,
    // Step
    Given,
    When,
    Then,
    And,
    But,
    Repeat,
    // Data
    Examples,
    Example,
    // Docs
    DocString,
    Comment,
    Description,
    Tag,
    // Linting
    Whitespace,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    keyword: Keywords,
    raw: String,
    line: usize,
    start: usize,
    end: usize,
}

pub static TOKEN_PATTERNS: &[(Keywords, &str)] = &[
    (Keywords::Comment, r"^#"),
    (Keywords::Tag, r"^@\w+"),
    (Keywords::DocString, r#"^"""#),
    (Keywords::Feature, r"^Feature:"),
    (Keywords::Background, r"^Background:"),
    (Keywords::ScenarioOutline, r"^Scenario Outline:"),
    (Keywords::ScenarioTemplate, r"^Scenario Template:"),
    (Keywords::Scenario, r"^Scenario:"),
    (Keywords::Examples, r"^Examples:"),
    (Keywords::Example, r"^Example:"),
    (Keywords::Given, r"^Given\s+"),
    (Keywords::When, r"^When\s+"),
    (Keywords::Then, r"^Then\s+"),
    (Keywords::And, r"^And\s+"),
    (Keywords::But, r"^But\s+"),
    (Keywords::Repeat, r"^\*\s+"),
    (Keywords::Whitespace, r"^\s+"),
    (Keywords::Description, r"^.+"),
];

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut lines = input.lines();
    let mut result: Vec<Token> = Vec::new();

    let mut line_count = 1;
    while let Some(line) = lines.next() {
        let mut input_line = line.to_string();
        let mut pos = 0;

        while !input_line.is_empty() {
            for (keyword, pattern) in TOKEN_PATTERNS.iter() {
                let re = Regex::new(pattern).unwrap();

                if let Some(m) = re.find(&input_line) {
                    if m.start() == 0 {
                        let consumed = m.end();

                        result.push(Token {
                            keyword: keyword.clone(),
                            start: pos,
                            end: pos + consumed,
                            raw: m.as_str().to_string(),
                            line: line_count,
                        });

                        input_line.drain(..consumed);
                        pos += consumed;

                        break;
                    }
                }
            }
        }

        line_count += 1;
    }

    result
}

#[cfg(test)]
mod tokenizer_tests;
