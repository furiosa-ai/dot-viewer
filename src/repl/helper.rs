use std::borrow::Cow::{ self, Borrowed };
use rustyline::highlight::Highlighter;
use rustyline_derive::{ Completer, Helper, Hinter, Validator };

#[derive(Helper, Completer, Hinter, Validator)]
pub struct ReplHelper {
    pub colored_prompt: String,
}

impl Highlighter for ReplHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }
}
