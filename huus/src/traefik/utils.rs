use std::sync::LazyLock;

use regex::Regex;

static DOMAIN_REGEX: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r"(?:[|&]|^|\s)Host\(`(.+?)`\)").unwrap());

// parse Host(`abc`) || Host(`domain`)&&Host(`ccc`) && !Host(`ignore`)
pub fn parse_rule_to_domains(rule: &str) -> Vec<String> {
	DOMAIN_REGEX
		.captures_iter(rule)
		.map(|cap| cap[1].to_string())
		.collect()
}
