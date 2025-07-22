/// 6.1.4 The String Type
/// https://262.ecma-international.org/16.0/#sec-ecmascript-language-types-string-type
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub(crate) struct JSString(pub(crate) String);

impl JSString {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn utf16_len(&self) -> usize {
        self.0.chars().count()
    }
}

impl JSString {
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for JSString {
    fn from(value: String) -> Self {
        JSString(value)
    }
}

impl From<&String> for JSString {
    fn from(value: &String) -> Self {
        JSString(value.clone())
    }
}

impl From<&str> for JSString {
    fn from(value: &str) -> Self {
        JSString(value.to_string())
    }
}
