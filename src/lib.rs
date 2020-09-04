#[warn(clippy::pedantic)]

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}
