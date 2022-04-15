use std::borrow::Cow;
use regex::Regex;

struct Rule {
    pattern: Regex,
    replace: String,
}

pub struct Rules(Vec<Rule>);

impl Rules {
    pub fn parse(s: &str) -> Result<Rules, regex::Error> {
        let mut rules = Vec::new();
        for line in s.lines() {
            let mut linesplit = line.splitn(2, "::=");
            if let Some(patternstr) = linesplit.next() {
                if let Some(replacestr) = linesplit.next() {
                    rules.push(
                        Rule {
                            pattern: Regex::new(patternstr)?,
                            replace: replacestr.to_string(),
                        }
                    );
                }
            }
        }
        Ok(Rules(rules))
    }

    pub fn replace(&self, text: &mut String) -> bool {
        let mut replaced = false;
        for rule in self.0.iter() {
            match rule.pattern.replace(&text, &rule.replace) {
                Cow::Borrowed(_) => {
                }
                Cow::Owned(owned) => {
                    *text = owned;
                    replaced = true;
                }
            }
        }
        replaced
    }

    pub fn run(&self, text: &str) -> String {
        let mut text = text.to_string();
        while self.replace(&mut text) {
        }
        text
    }
}
