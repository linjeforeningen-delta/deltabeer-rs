#[derive(Debug, Default, Clone)]
pub(crate) struct NumericInput {
    value: String,
}

impl NumericInput {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, c: char) {
        if c.is_ascii_digit() {
            self.value.push(c);
        }
    }

    pub(crate) fn backspace(&mut self) {
        self.value.pop();
    }

    pub(crate) fn clear(&mut self) {
        self.value.clear();
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.value
    }

    pub(crate) fn value(&self) -> Option<u32> {
        self.value.parse().ok()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl std::fmt::Display for NumericInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}