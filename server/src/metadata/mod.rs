pub mod scanner;

pub fn vectorize_tags<'a>(tags: impl Iterator<Item = &'a str> + Sized) -> Vec<String> {
    tags.map(|tag| tag.to_string()).collect()
}
