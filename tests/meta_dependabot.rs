#![cfg(not(miri))]

#[path = "constants_.rs"]
mod constants;
use constants::*;

#[test]
fn user_appears() {
	version_sync::assert_contains_regex!(
		".github/dependabot.yml",
		&format!(r#"- "{user}"$"#, user = USER)
	);
}
