use crate::app::TextInput;
use crate::ui::theme::{Palette, Theme};
use crate::ui::traits::Content;
use ratatui::text::{Line, Span};

pub(crate) struct Form<'a> {
    fields: Vec<FormField<'a>>,
    selected: usize,
}

struct FormField<'a> {
    label: &'a str,
    input: &'a TextInput,
    hidden: bool,
    placeholder: Option<&'a str>,
}

impl<'a> Form<'a> {
    pub(crate) fn new(selected: usize) -> Self {
        Self {
            fields: Vec::new(),
            selected,
        }
    }

    pub(crate) fn add_field(
        mut self,
        label: &'a str,
        input: &'a TextInput,
    ) -> Self {
        self.fields.push(FormField {
            label,
            input,
            hidden: false,
            placeholder: None,
        });

        self
    }

    pub(crate) fn add_hidden_field(
        mut self,
        label: &'a str,
        input: &'a TextInput,
    ) -> Self {
        self.fields.push(FormField {
            label,
            input,
            hidden: true,
            placeholder: None,
        });

        self
    }

    pub(crate) fn add_field_with_placeholder(
        mut self,
        label: &'a str,
        input: &'a TextInput,
        placeholder: &'a str,
    ) -> Self {
        self.fields.push(FormField {
            label,
            input,
            hidden: false,
            placeholder: Some(placeholder),
        });

        self
    }

    pub(crate) fn add_hidden_field_with_placeholder(
        mut self,
        label: &'a str,
        input: &'a TextInput,
        placeholder: &'a str,
    ) -> Self {
        self.fields.push(FormField {
            label,
            input,
            hidden: true,
            placeholder: Some(placeholder),
        });

        self
    }
}
impl Content for Form<'_> {
    fn lines(
        &self,
        theme: &Theme,
        palette: Palette,
    ) -> Vec<Line<'_>> {
        let label_width = self
            .fields
            .iter()
            .map(|field| field.label.len())
            .max()
            .unwrap_or(0);

        self.fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let active = i == self.selected;

                let value = if field.hidden {
                    "*".repeat(field.input.len())
                } else {
                    field.input.as_string()
                };

                let marker = if self.fields.len() > 1 {
                    if active { "> " } else { "  " }
                } else {
                    ""
                };

                let style = if active {
                    theme.selected_style(palette)
                } else {
                    palette.muted()
                };

                Line::from(vec![
                    Span::raw(format!(
                        "{:<width$}",
                        field.label,
                        width = label_width + 2
                    )),
                    Span::raw(value),
                ])
                    .style(style)
            })
            .collect()
    }
}