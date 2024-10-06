pub fn get_path_parts<'a>(s: &'a str) -> Vec<&'a str> {
    s.split("/")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
}
