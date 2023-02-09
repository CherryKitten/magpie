pub mod scanner;

pub fn vectorize_tags<'a>(tags: impl Iterator<Item = &'a str> + Sized) -> Vec<String> {
    let mut temp_vec = vec![];
    for tag in tags {
        temp_vec.push(tag.to_string());
    }
    temp_vec
}
