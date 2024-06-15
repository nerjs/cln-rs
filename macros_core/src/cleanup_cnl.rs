fn find_in(input: &Vec<&str>, str_chunk: &str) -> Option<usize> {
    input.iter().position(|s| *s == str_chunk)
}

pub fn cleanup_cnl<T>(input: T) -> String
where
    T: Into<String>,
{
    let input: String = input.into();
    if input.is_empty() {
        return "".to_string();
    }

    let mut strings_list: Vec<&str> = Vec::new();
    let array = input.split(" ").map(|s| s.trim()).filter(|s| !s.is_empty());

    for str_chunk in array {
        if let Some(founded_index) = find_in(&strings_list, str_chunk) {
            strings_list.remove(founded_index);
        }
        strings_list.push(str_chunk);
    }

    if strings_list.len() == 0 {
        return "".to_string();
    }

    strings_list.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_transit() {
        let line = cleanup_cnl("some string".to_string());

        assert_eq!(line, "some string");
    }

    #[test]
    fn cleanup_and_deduplicate() {
        let line = cleanup_cnl("first second    second third first  fourth".to_string());
        assert_eq!(line, "second third first fourth")
    }
}
