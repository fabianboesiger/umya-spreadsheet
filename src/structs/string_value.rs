#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StringValue {
    value: Option<String>,
}
impl StringValue {
    pub(crate) fn get_value(&self) -> &str {
        self.value.as_deref().unwrap_or("")
    }

    pub(crate) fn get_value_str(&self) -> &str {
        self.get_value()
    }

    pub(crate) fn set_value<S: Into<String>>(&mut self, value: S) -> &mut StringValue {
        self.value = Some(value.into());
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut StringValue {
        self.set_value(value.into())
    }

    pub(crate) fn remove_value(&mut self) -> &mut Self {
        self.value = None;
        self
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_str();
        }
        "empty!!"
    }
}
