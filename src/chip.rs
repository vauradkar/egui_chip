use egui::Color32;
use egui::Label;
use egui::Layout;
use egui::Margin;
use egui::RichText;
use egui::Sense;
use egui::TextEdit;
use egui::Ui;

use crate::output::ChipEditOutput;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) enum ChipKind {
    Separator,
    Text,
}

fn default_chip_text_color(_ui: &Ui) -> Color32 {
    Color32::WHITE
}

fn default_chip_bg_color(_ui: &Ui) -> Color32 {
    Color32::BLUE
}

pub(crate) static DEFAULT_CHIP_SIZE: [f32; 2] = [40., 20.];

fn default_inner_margin() -> Margin {
    let mut r: Margin = 0.0.into();
    r.right = 3;
    r.left = 3;
    r
}

fn default_outer_margin() -> Margin {
    let mut r: Margin = 0.0.into();
    r.right = 0;
    r.left = 0;
    r
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Chip {
    pub(crate) at_start: bool,
    pub(crate) at_end: bool,
    pub(crate) kind: ChipKind,
    pub(crate) bg_color: Option<Color32>,
    pub(crate) text_color: Option<Color32>,
    pub(crate) size: Option<[f32; 2]>,
    #[serde(skip)]
    pub(crate) icon: Option<RichText>,
}

impl Chip {
    pub fn new_separator() -> Self {
        Chip {
            at_start: true,
            at_end: true,
            kind: ChipKind::Separator,
            bg_color: None,
            text_color: None,
            size: Some(DEFAULT_CHIP_SIZE),
            icon: None,
        }
    }

    pub fn new_chip(size: Option<[f32; 2]>, icon: Option<RichText>) -> Self {
        Chip {
            at_start: false,
            at_end: false,
            kind: ChipKind::Text,
            bg_color: None,
            text_color: None,
            size,
            icon,
        }
    }

    pub fn show_separator(&mut self, ui: &mut Ui, text: &mut String) -> ChipEditOutput {
        TextEdit::singleline(text)
            .clip_text(true)
            .desired_width(0.0)
            .frame(false)
            .margin(Margin::symmetric(4, 2))
            .show(ui)
            .into()
    }

    pub(crate) fn bg_color(&self, ui: &Ui) -> Color32 {
        self.bg_color.unwrap_or(default_chip_bg_color(ui))
    }

    pub(crate) fn text_color(&self, ui: &Ui) -> Color32 {
        self.text_color.unwrap_or(default_chip_text_color(ui))
    }

    pub(crate) fn draw_text(
        &mut self,
        ui: &mut Ui,
        focused: bool,
        text: &mut String,
    ) -> ChipEditOutput {
        let text_color = self.text_color(ui);
        let mut r = None;
        if let Some(icon) = &self.icon {
            r = Some(
                ui.add(
                    Label::new(icon.clone().color(text_color))
                        .halign(egui::Align::Center)
                        .sense(Sense::click())
                        .truncate(),
                ),
            );
        }
        let mut ret: ChipEditOutput = if focused {
            TextEdit::singleline(text)
                .text_color(text_color)
                .clip_text(true)
                .frame(false)
                .horizontal_align(egui::Align::LEFT)
                .vertical_align(egui::Align::TOP)
                .show(ui)
                .into()
        } else {
            ui.add_sized(
                self.size.unwrap_or([0., 0.]),
                Label::new(RichText::new(text.as_str()).color(text_color))
                    .sense(Sense::click())
                    .truncate(),
            )
            .into()
        };
        if let Some(r) = r {
            ret.response = ret.response.union(r);
        }

        ret
    }

    pub fn show_chip(&mut self, ui: &mut Ui, focused: bool, text: &mut String) -> ChipEditOutput {
        egui::Frame::new()
            .corner_radius(8)
            .fill(self.bg_color(ui))
            .inner_margin(default_inner_margin())
            .outer_margin(default_outer_margin())
            .show(ui, |ui| {
                let layout = Layout::left_to_right(egui::Align::Center);
                if let Some(size) = self.size {
                    ui.allocate_ui_with_layout(size.into(), layout, |ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(0., 1.0);
                        self.draw_text(ui, focused, text)
                    })
                    .inner
                } else {
                    ui.with_layout(layout, |ui| self.draw_text(ui, focused, text))
                        .inner
                }
            })
            .inner
    }

    pub fn show(&mut self, ui: &mut Ui, focused: bool, text: &mut String) -> ChipEditOutput {
        if self.is_separator() {
            self.show_separator(ui, text)
        } else {
            self.show_chip(ui, focused, text)
        }
    }

    pub(crate) fn update_position(&mut self, output: &ChipEditOutput, text: &str) {
        self.at_start = output.cursor_at_start();
        self.at_end = output.cursor_at_end(text);
    }

    pub fn at_start(&self) -> bool {
        self.at_start
    }

    pub fn at_end(&self) -> bool {
        self.at_end
    }

    pub fn is_separator(&self) -> bool {
        matches!(self.kind, ChipKind::Separator)
    }
}
