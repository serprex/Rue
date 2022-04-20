use std::borrow::Cow;
use rand::thread_rng;
use rand::seq::SliceRandom;
use regex::{Regex, RegexSet};

struct Rule {
	pattern: Regex,
	replace: String,
}

pub struct Rules(RegexSet, Vec<Rule>);

impl Rules {
	pub fn parse(s: &str) -> Result<Rules, regex::Error> {
		let mut rules = Vec::new();
		let mut prevpattern: Option<Regex> = None;
		for line in s.lines() {
			if let Some(pattern) = prevpattern.take() {
				rules.push(
					Rule {
						pattern: pattern,
						replace: line.to_string(),
					}
				)
			} else if line.starts_with('=') {
				prevpattern = Some(Regex::new(&line[1..])?);
			} else {
				let mut linesplit = line.splitn(2, "::=");
				if let Some(patternstr) = linesplit.next() {
					if patternstr.is_empty() {
						continue
					}
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
		}
		let set = RegexSet::new(rules.iter().map(|rule| rule.pattern.as_str()))?;
		Ok(Rules(set, rules))
	}

	pub fn replace(&self, text: &mut String) -> bool {
		let matches = self.0.matches(&text);
		if let Some(&idx) = matches.iter().collect::<Vec<_>>().choose(&mut thread_rng()) {
			let rule = &self.1[idx];
			if let Cow::Owned(replacement) = rule.pattern.replace_all(&text, &rule.replace) {
				*text = replacement;
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn run(&self, text: &str) -> String {
		let mut text = text.to_string();
		while self.replace(&mut text) {
		}
		text
	}
}
