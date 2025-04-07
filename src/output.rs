use egui::Key;
use egui::Response;
use egui::text::CursorRange;
use egui::text_edit::TextEditOutput;

/// Represents the output of a `ChipEdit` widget.
pub struct ChipEditOutput {
    /// The response from the widget.
    /// As a single ChipEdit can have multiple TextEdits in it,
    /// the `Response` is the `Response::union` of all the containing
    /// responses.
    pub response: Response,

    /// The range of the cursor within the text.
    pub cursor_range: Option<CursorRange>,

    /// True if the widget gained focus.
    pub gained_focus: bool,
}

impl ChipEditOutput {
    /// Merges another `ChipEditOutput` into this one.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `ChipEditOutput` to merge.
    pub fn union(&mut self, other: Self) {
        let Self {
            response,
            cursor_range,
            gained_focus,
        } = other;
        self.gained_focus |= gained_focus || response.gained_focus();
        self.response = self.response.union(response);
        if self.cursor_range.is_none() {
            self.cursor_range = cursor_range;
        }
    }

    /// Checks if the cursor is at the specified position.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position to check.
    ///
    /// # Returns
    ///
    /// `true` if the cursor is at the specified position, `false` otherwise.
    pub fn cursor_at(&self, pos: usize) -> bool {
        if let Some(cursor) = &self.cursor_range {
            cursor.single().is_some() && (cursor.as_sorted_char_range().end == pos)
        } else {
            false
        }
    }

    /// Checks if the cursor is at the end of the text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to check.
    ///
    /// # Returns
    ///
    /// `true` if the cursor is at the end of the text, `false` otherwise.
    pub(crate) fn cursor_at_end(&self, text: &str) -> bool {
        self.cursor_at(text.len())
    }

    /// Checks if the cursor is at the start of the text.
    ///
    /// # Returns
    ///
    /// `true` if the cursor is at the start of the text, `false` otherwise.
    pub(crate) fn cursor_at_start(&self) -> bool {
        self.cursor_at(0)
    }

    /// Checks if the widget gained focus.
    ///
    /// # Returns
    ///
    /// `true` if the widget gained focus, `false` otherwise.
    pub fn gained_focus(&self) -> bool {
        self.response.gained_focus() || self.response.clicked()
    }

    /// Checks if the widget lost focus.
    ///
    /// # Returns
    ///
    /// `true` if the widget lost focus, `false` otherwise.
    pub fn lost_focus(&self) -> bool {
        self.response.lost_focus()
    }

    /// Checks if the specified key is pressed.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check.
    ///
    /// # Returns
    ///
    /// `true` if the key is pressed, `false` otherwise.
    pub(crate) fn is_key_pressed(&self, key: Key) -> bool {
        self.response.ctx.input(|i| i.key_pressed(key))
    }
}

impl From<TextEditOutput> for ChipEditOutput {
    fn from(value: TextEditOutput) -> Self {
        Self {
            gained_focus: value.response.gained_focus(),
            response: value.response,
            cursor_range: value.cursor_range,
        }
    }
}

impl From<Response> for ChipEditOutput {
    fn from(response: Response) -> Self {
        Self {
            gained_focus: response.gained_focus(),
            response,
            cursor_range: None,
        }
    }
}
