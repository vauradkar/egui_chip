use std::fmt::Display;

use egui::Color32;
use egui::Response;
use egui::Ui;
use egui::Widget;

use crate::ChipEditOutput;
use crate::UnownedChipEdit;

/// Creates a chip style textbox
///
/// Press backspace in empty chip deletes it.
/// Lost focus from empty chip deletes it.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ChipEdit {
    /// texts
    pub(crate) texts: Vec<String>,
    /// The units (chips) in the widget.
    pub(crate) unowned: UnownedChipEdit,
}

impl Display for ChipEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.values().join(&self.unowned.separator))
    }
}

impl ChipEdit {
    /// Creates a new `ChipEdit` with the specified separator.
    ///
    /// # Arguments
    ///
    /// * `separator` - The separator string used to split chip texts.
    ///
    /// # Errors
    ///
    /// Returns an error if the separator is empty.
    pub fn new(separator: &str) -> Result<Self, String> {
        if separator.is_empty() {
            Err("separator cannot be empty".to_owned())
        } else {
            let mut ret = Self {
                texts: vec![],
                unowned: UnownedChipEdit::new(separator)?,
            };
            ret.rebuild();
            Ok(ret)
        }
    }

    /// Displays the `ChipEdit` widget in the given UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - The UI to display the widget in.
    ///
    /// # Returns
    ///
    /// A `ChipEditOutput` containing the result of the widget interaction.
    pub fn show(&mut self, ui: &mut Ui) -> ChipEditOutput {
        self.unowned.show(ui, &mut self.texts)
    }

    /// Rebuilds the `ChipEdit` widget with the given texts.
    ///
    /// # Arguments
    ///
    /// * `texts` - A vector of strings representing the texts for the chips.
    pub(crate) fn rebuild(&mut self) {
        self.unowned.rebuild(&mut self.texts);
    }

    /// Sets the texts for the chips.
    ///
    /// # Arguments
    ///
    /// * `texts` - An iterator of strings representing the texts for the chips.
    pub fn set_text(&mut self, texts: impl IntoIterator<Item = impl ToString>) {
        self.texts = texts.into_iter().map(|s| s.to_string()).collect();
        self.rebuild();
    }

    /// Returns the current values of the chips.
    ///
    /// # Returns
    ///
    /// A vector of strings representing the current values of the chips.
    pub fn values(&self) -> Vec<String> {
        self.texts.clone()
    }

    /// Returns the default background color for the widget
    pub fn default_widget_bg_color(ui: &Ui) -> Color32 {
        ui.visuals().extreme_bg_color
    }

    /// Returns the default foreground color for the widget
    pub fn default_widget_fg_color(ui: &Ui) -> Color32 {
        ui.visuals().selection.stroke.color
    }
}

impl Widget for &mut ChipEdit {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui).response
    }
}
