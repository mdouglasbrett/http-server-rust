pub fn get_path_parts(s: &str) -> Vec<&str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}
