use egui::Key;

use crate::chip::Chip;
use crate::chip_edit::ChipEditOutput;

#[derive(Debug)]
pub(crate) struct State {
    // what index to focus on. None implies the focus is elsewhere in the ui
    pub focus: Option<usize>,

    // true if this iteration updated focus change
    pub focus_changed: bool,

    // chips at index (a, b) need to be merged
    pub merge: Option<(usize, usize)>,

    // chip at index needs to be split
    pub split: Option<usize>,

    // chip at index needs to be deleted
    pub delete: Option<usize>,
}

impl From<&Option<usize>> for State {
    fn from(value: &Option<usize>) -> Self {
        let mut ret = Self::new();
        ret.focus = *value;
        ret
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            focus: None,
            focus_changed: false,
            merge: None,
            split: None,
            delete: None,
        }
    }

    pub fn set_focus(&mut self, index: usize) {
        self.focus = Some(index);
        self.focus_changed = true;
    }

    pub fn set_merge(&mut self, a: usize, b: usize) {
        self.merge = Some((a, b));
    }

    pub fn clear_focus(&mut self) {
        self.focus = None;
        self.focus_changed = true;
    }

    pub fn update(
        &mut self,
        max_index: usize,
        index: usize,
        unit: &Chip,
        output: &ChipEditOutput,
        separator: &str,
    ) {
        let resp = &output.response;

        if resp.changed() && unit.is_separator && !unit.text.is_empty() {
            self.split = Some(index);
            self.set_focus(index + 1);
        }

        if self.split.is_none() && output.response.changed() && unit.needs_update(separator) {
            self.split = Some(index);
            self.set_focus(index);
        }

        if output.gained_focus() {
            self.set_focus(index);
        } else if output.lost_focus() && self.focus == Some(index) {
            self.clear_focus();
        } else if self.focus_changed {
            return;
        }

        let act_at_end = unit.at_end && output.cursor_at_end(&unit.text);
        let act_at_start = unit.at_start && output.cursor_at_start();

        if resp.has_focus() {
            if output.is_key_pressed(Key::Delete) && act_at_end && index < max_index {
                self.set_focus(index);
                if unit.is_separator {
                    self.delete = Some(index + 1);
                } else {
                    self.set_merge(index, index + 2);
                }
            } else if output.is_key_pressed(Key::Backspace) && act_at_start && index > 1 {
                self.set_focus(index - 2);
                if unit.is_separator {
                    self.delete = Some(index - 1);
                } else {
                    self.set_merge(index - 2, index);
                }
            } else if output.is_key_pressed(Key::ArrowRight) && act_at_end && index < max_index {
                self.set_focus(index + 1);
            } else if output.is_key_pressed(Key::ArrowLeft) && act_at_start && index > 0 {
                self.set_focus(index - 1);
            }
        }
    }
}
