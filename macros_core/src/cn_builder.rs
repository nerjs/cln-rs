use crate::cleanup_cnl::cleanup_cnl;

pub struct CnPart(String);


impl From<&str> for CnPart {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for CnPart {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<T: Into<String>> From<Option<T>> for CnPart {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => Self(value.into()),
            None => Self(String::new()),
        }
    }
}

impl From<(bool, &str, &str)> for CnPart {
    fn from(value: (bool, &str, &str)) -> Self {
        match value.0 {
            true => Self(value.1.to_string()),
            false => Self(value.2.to_string()),
        }
    }
}

impl From<(bool, &str)> for CnPart {
    fn from(value: (bool, &str)) -> Self {
        Self::from((value.0, value.1, ""))
    }
}

pub struct CnBuilder(String);

impl CnBuilder {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn add<T>(mut self, item: T) -> Self
    where
        T: Into<CnPart>,
    {
        let item: CnPart = item.into();
        self.0.push_str(&format!(" {}", item.0));

        self
    }

    pub fn to_classlist(&self) -> String {
        cleanup_cnl(&self.0)
    }
}
