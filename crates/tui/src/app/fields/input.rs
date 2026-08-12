#[derive(Debug, Default, Clone)]
pub(crate) struct TextInput {
    value: String,
    constraint: InputConstraint,
    max_len: Option<usize>,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum InputConstraint {
    #[default]
    Any,
    Ascii,
    Numeric,
}

impl TextInput {
    pub(crate) fn new(constraint: InputConstraint) -> Self {
        Self {
            value: String::new(),
            constraint,
            max_len: None,
        }
    }

    pub(crate) fn push(&mut self, c: char) {
        if self.accepts(c) {
            self.value.push(c);
        }
    }

    fn accepts(&self, c: char) -> bool {
        match self.constraint {
            InputConstraint::Any => true,
            InputConstraint::Ascii => c.is_ascii(),
            InputConstraint::Numeric => c.is_ascii_digit(),
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

impl std::fmt::Display for TextInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}