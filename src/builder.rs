use egui::Color32;
use egui::RichText;

use crate::ChipEdit;
use crate::UnownedChipEdit;

/// A builder for creating a `ChipEdit` widget with various customization
/// options.
///
/// # Examples
///
/// ```
/// use egui::Color32;
/// use egui_chip::ChipEditBuilder;
///
/// let chip_edit = ChipEditBuilder::new(",")
///     .unwrap()
///     .texts(vec!["Chip1", "Chip2", "Chip3"])
///     .chip_colors(Color32::from_rgb(255, 0, 0), Color32::from_rgb(0, 255, 0))
///     .widget_colors(Color32::from_rgb(0, 0, 255), Color32::from_rgb(255, 255, 0))
///     .frame(true)
///     .chip_size(Some([100.0, 50.0]))
///     .build();
/// ```
pub struct ChipEditBuilder {
    chip_edit: ChipEdit,
    texts: Vec<String>,
}

impl ChipEditBuilder {
    /// Creates a new `ChipEditBuilder` with the specified separator.
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
            let ret = Self {
                chip_edit: ChipEdit {
                    texts: vec![],
                    unowned: crate::UnownedChipEdit::new(separator)?,
                },
                texts: vec![],
            };
            Ok(ret)
        }
    }

    /// Sets the initial texts for the chips.
    ///
    /// # Arguments
    ///
    /// * `texts` - An iterator of strings representing the initial texts for
    ///   the chips.
    pub fn texts(mut self, texts: impl IntoIterator<Item = impl ToString>) -> Self {
        self.texts = texts.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Sets the background and text colors for the chips within `ChipEdit`.
    ///
    /// # Arguments
    ///
    /// * `bg_color` - The background color for the chips.
    /// * `text_color` - The text color for the chips.
    pub fn chip_colors(mut self, bg_color: Color32, text_color: Color32) -> Self {
        self.chip_edit.unowned.chip_bg = Some(bg_color);
        self.chip_edit.unowned.chip_fg = Some(text_color);
        self
    }

    /// Sets the background and foreground colors for the widget.
    ///
    /// # Arguments
    ///
    /// * `bg_color` - The background color for the widget.
    /// * `fg_color` - The foreground color for the widget.
    pub fn widget_colors(mut self, bg_color: Color32, fg_color: Color32) -> Self {
        self.chip_edit.unowned.widget_bg = Some(bg_color);
        self.chip_edit.unowned.widget_fg = Some(fg_color);
        self
    }

    /// Sets whether the widget should have a frame.
    ///
    /// # Arguments
    ///
    /// * `frame` - A boolean indicating whether the widget should have a frame.
    pub fn frame(mut self, frame: bool) -> Self {
        self.chip_edit.unowned.frame = frame;
        self
    }

    /// Sets the size of the chips.
    ///
    /// # Arguments
    ///
    /// * `chip_size` - An optional array representing the width and height of
    ///   the chips.
    pub fn chip_size(mut self, chip_size: Option<[f32; 2]>) -> Self {
        self.chip_edit.unowned.chip_size = chip_size;
        self
    }

    /// Sets leading icon for the chips
    ///
    /// # Arguments
    ///
    /// * `char` - A single char text
    pub fn chip_icon(mut self, icon: Option<RichText>) -> Result<Self, String> {
        if matches!(&icon, Some(t) if t.text().chars().count() != 1) {
            Err(format!(
                "icon text needs to be single char but found {}",
                icon.unwrap().text().len()
            ))
        } else {
            self.chip_edit.unowned.icon = icon;
            Ok(self)
        }
    }

    /// Builds the `ChipEdit` widget.
    ///
    /// # Returns
    ///
    /// The constructed `ChipEdit` widget.
    pub fn build(self) -> ChipEdit {
        let Self {
            mut chip_edit,
            texts,
        } = self;
        chip_edit.set_text(texts);
        chip_edit.rebuild();
        chip_edit
    }

    /// Builds the `UnownedChipEdit` widget without texts ownership.
    ///
    /// # Returns
    ///
    /// The constructed `UnownedChipEdit` widget.
    pub fn build_unowned(self) -> UnownedChipEdit {
        let Self {
            mut chip_edit,
            texts,
        } = self;
        chip_edit.set_text(texts);
        chip_edit.rebuild();
        chip_edit.unowned
    }
}
