pub mod string {
    pub fn get_from_vec<'a>(source: &'a Vec<&str>, idx: usize) -> &'a str {
        source.get(idx).copied().unwrap_or("")
    }
}
