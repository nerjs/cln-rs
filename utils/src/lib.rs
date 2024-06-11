use std::collections::HashMap;

pub fn normalize_list_impl<T>(list: T) -> String where T: Into<String> {
    let list:String = list.into();
    let mut string_list: Vec<&str> = Vec::new();
    let mut cache_list_indexes: HashMap<&str, usize> = HashMap::new();

    for str_part in list.split(" ").map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Some(cached_index) = cache_list_indexes.get(str_part) {
            string_list[*cached_index] = "";
        }
        cache_list_indexes.insert(str_part, string_list.len());
        string_list.push(str_part);
    }

    string_list
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}


#[macro_export]
macro_rules! zzz {
    ($a:tt) => {
        $crate::normalize_list_impl($a)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_transit() {
        let line = normalize_list_impl("some string".to_string());

        assert_eq!(line, "some string");
    }

    #[test]
    fn normalize_string() {
        let line = normalize_list_impl("first second    second third first  fourth".to_string());
        assert_eq!(line, "second third first fourth")
    }
}
