use std::fmt::Display;

use egui::Align;
use egui::Color32;
use egui::Direction;
use egui::Layout;
use egui::Response;
use egui::RichText;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::ChipEditOutput;
use crate::chip::Chip;
use crate::chip::ChipKind;
use crate::chip::DEFAULT_CHIP_SIZE;
use crate::state::State;

/// Creates a chip style textbox from mutable reference to texts.
///
/// Unlike [`super::ChipEdit`], this widget does not own the texts.
/// Pressing backspace in empty chip deletes it.
/// Lost focus from empty chip deletes it
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UnownedChipEdit {
    /// The separator string used to split chip texts.
    pub(crate) separator: String,
    /// separator text
    pub(crate) separator_text: Vec<String>,
    /// The units (chips) in the widget.
    pub(crate) units: Vec<Chip>,
    /// The background color of the widget.
    pub(crate) widget_bg: Option<Color32>,
    /// The foreground color of the widget.
    pub(crate) widget_fg: Option<Color32>,
    /// The background color of the chips.
    pub(crate) chip_bg: Option<Color32>,
    /// The foreground color of the chips.
    pub(crate) chip_fg: Option<Color32>,
    /// The index of the focused chip, if any.
    pub(crate) focused: Option<usize>,
    /// Whether the widget should have a frame.
    pub(crate) frame: bool,
    /// The size of the chips.
    pub(crate) chip_size: Option<[f32; 2]>,
    /// texts len during previous iteration
    /// We use this to detect if we need to rebuild the widget
    pub(crate) texts_len: usize,

    /// Leading `icon` char in chip
    // TODO: Fix serde
    #[serde(skip)]
    pub(crate) icon: Option<RichText>,
}

impl Display for UnownedChipEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.separator)
    }
}

impl UnownedChipEdit {
    /// Creates a new `UnownedChipEdit` with the specified separator.
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
                separator: separator.into(),
                separator_text: vec![],
                units: vec![],
                widget_bg: None,
                widget_fg: None,
                chip_bg: None,
                chip_fg: None,
                focused: None,
                frame: true,
                chip_size: Some(DEFAULT_CHIP_SIZE),
                icon: None,
                texts_len: 0,
            };
            ret.rebuild(&mut []);
            Ok(ret)
        }
    }

    pub(crate) fn default_fg_stroke(&self, ui: &Ui) -> impl Into<Stroke> {
        Stroke {
            width: if self.frame {
                ui.visuals().selection.stroke.width
            } else {
                0.
            },
            color: if self.focused.is_some() {
                Self::default_widget_fg_color(ui)
            } else {
                ui.style().visuals.faint_bg_color
            },
        }
    }

    /// Displays the `UnownedChipEdit` widget in the given UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - The UI to display the widget in.
    ///
    /// # Returns
    ///
    /// A `ChipEditOutput` containing the result of the widget interaction.
    pub fn show(&mut self, ui: &mut Ui, texts: &mut Vec<String>) -> ChipEditOutput {
        if self.texts_len != texts.len() {
            self.rebuild(texts);
        }
        let max_index = self.units.len() - 1;
        let mut outputs = Vec::with_capacity(self.units.len());

        let widget_bg = self.widget_bg.unwrap_or(Self::default_widget_bg_color(ui));
        let mut state = State::from(&self.focused);
        let layout = Layout::from_main_dir_and_cross_align(Direction::LeftToRight, Align::TOP)
            .with_main_wrap(true)
            .with_cross_justify(false);
        let mut ret: ChipEditOutput = ui
            .allocate_ui(vec2(ui.available_size_before_wrap().x, 20.), |ui| {
                ui.with_layout(layout, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0., 1.0);
                    egui::Frame::new()
                        .fill(widget_bg)
                        .stroke(self.default_fg_stroke(ui))
                        .show(ui, |ui| {
                            for (index, unit) in &mut self.units.iter_mut().enumerate() {
                                let text = if let ChipKind::Separator = &mut unit.kind {
                                    self.separator_text.get_mut(index / 2).unwrap()
                                } else {
                                    texts.get_mut(index / 2).unwrap()
                                };
                                let output = unit.show(ui, self.focused == Some(index), text);
                                state.update(
                                    max_index,
                                    index,
                                    unit,
                                    &output,
                                    &self.separator,
                                    text,
                                );
                                if state.focus == Some(index) {
                                    output.response.request_focus();
                                }

                                unit.update_position(&output, text);
                                outputs.push(output);
                            }
                        });
                });
            })
            .response
            .into();

        // Retain focus history for the next iteration
        if state.focus_changed {
            println!("focus old:{:?} new:{:?}", self.focused, state.focus);
            self.focused = state.focus;
        }

        if let Some(_index) = state.split {
            self.split(texts);
        }

        if state.merge.is_some() || state.delete.is_some() {
            self.merge(
                texts,
                state.merge.unwrap_or((usize::MAX, usize::MAX)),
                state.delete.unwrap_or(usize::MAX),
            );
        }

        outputs.into_iter().for_each(|o| ret.union(o));
        ret
    }

    /// Rebuilds the `UnownedChipEdit` widget with the given texts.
    ///
    /// # Arguments
    ///
    /// * `texts` - A vector of strings representing the texts for the chips.
    pub(crate) fn rebuild(&mut self, texts: &mut [String]) {
        self.units.clear();
        let len = texts.len();
        self.texts_len = len;

        self.units.push(Chip::new_separator());
        self.separator_text.push("".to_owned());
        for (index, _) in texts.iter_mut().enumerate() {
            let mut chip = Chip::new_chip(self.chip_size, self.icon.clone());
            chip.bg_color = self.chip_bg;
            chip.text_color = self.chip_fg;
            self.units.push(chip);
            if index != len - 1 {
                self.units.push(Chip::new_separator());
                self.separator_text.push("".to_owned());
            }
        }
        if len > 0 {
            self.units.push(Chip::new_separator());
            self.separator_text.push("".to_owned());
        }
    }

    /// Sets the texts for the chips.
    ///
    /// # Arguments
    ///
    /// * `texts` - An iterator of strings representing the texts for the chips.
    pub fn set_text(&mut self, texts: &mut [String]) {
        self.rebuild(texts);
    }

    fn split(&mut self, texts: &mut Vec<String>) {
        let mut temp_texts = vec![];
        for (index, unit) in self.units.iter().enumerate() {
            // skip empty separators. we still care about non empty separators
            if unit.is_separator() && self.separator_text[index / 2].is_empty() {
                continue;
            }

            let (text, clear) = if let ChipKind::Separator = &unit.kind {
                (&mut self.separator_text[index / 2], true)
            } else {
                (&mut texts[index / 2], false)
            };
            let mut v = text.split(&self.separator).map(|s| s.to_owned()).collect();
            temp_texts.append(&mut v);
            if clear {
                text.clear();
            }
        }
        std::mem::swap(texts, &mut temp_texts);
        self.rebuild(texts);
    }

    fn merge(&mut self, texts: &mut Vec<String>, (a, b): (usize, usize), delete: usize) {
        println!("merge: {} {} {}", a, b, delete);
        let unit_min = if a < b { a } else { b };
        let mut text_min = 0;
        let unit_max = if a > b { a } else { b };
        let mut temp_texts = vec![];
        for (index, _) in self
            .units
            .iter()
            .enumerate()
            .filter(|(i, u)| *i != delete && !u.is_separator())
        {
            if index == unit_min {
                text_min = temp_texts.len();
            }
            if index != unit_max {
                temp_texts.push(texts[index / 2].to_owned());
            } else {
                temp_texts
                    .get_mut(text_min)
                    .unwrap()
                    .push_str(&texts[index / 2]);
            }
        }
        std::mem::swap(texts, &mut temp_texts);
        self.rebuild(texts);
    }

    /// Displays the `UnownedChipEdit` widget in the given UI and returns the
    /// response.
    ///
    /// # Arguments
    ///
    /// * `ui` - The UI to display the widget in.
    /// * `texts` - A mutable reference to a vector of strings representing the
    ///   texts for the chips.
    ///
    /// # Returns
    ///
    /// A `Response` object representing the result of the widget interaction.
    pub fn ui(&mut self, ui: &mut Ui, texts: &mut Vec<String>) -> Response {
        self.show(ui, texts).response
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
