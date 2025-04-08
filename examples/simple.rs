//! Simple demo app for egui_chip

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::CreationContext;
use eframe::egui;
use egui::Color32;
use egui::RichText;
use egui::TextEdit;
use egui::color_picker::Alpha;
use egui::color_picker::color_edit_button_srgba;
use egui_chip::ChipEdit;
use egui_chip::ChipEditBuilder;
use egui_chip::UnownedChipEdit;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640., 480.]),
        ..Default::default()
    };
    eframe::run_native(
        "Simple egui_chip demo",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_nerdfonts::add_to_fonts(&mut fonts, egui_nerdfonts::Variant::Regular);

            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::<MyApp>::new(MyApp::new(cc)))
        }),
    )
}

#[derive(Debug, PartialEq, Default, Clone)]
struct Configs {
    widget_bg: Color32,
    widget_fg: Color32,
    chip_bg: Color32,
    chip_fg: Color32,
    frame: bool,
    chip_size: [f32; 2],
    separator: String,
    icon: String,
}

impl Configs {
    fn new(cc: &CreationContext) -> Self {
        Self {
            frame: true,
            chip_size: [60., 20.],
            separator: ";".to_string(),
            icon: egui_nerdfonts::regular::SMILE.to_owned(),
            widget_bg: cc.egui_ctx.style().visuals.extreme_bg_color,
            widget_fg: cc.egui_ctx.style().visuals.selection.stroke.color,
            chip_bg: Color32::BLUE,
            chip_fg: Color32::WHITE,
        }
    }

    fn build(&self, texts: Vec<String>) -> ChipEdit {
        ChipEditBuilder::new(&self.separator)
            .unwrap()
            .frame(self.frame)
            .widget_colors(self.widget_bg, self.widget_fg)
            .chip_colors(self.chip_bg, self.chip_fg)
            .chip_size(Some(self.chip_size))
            .texts(texts)
            .chip_icon(if self.icon.is_empty() {
                None
            } else {
                Some(RichText::new(&self.icon).weak())
            })
            .unwrap()
            .build()
    }
}

struct MyApp {
    chip: ChipEdit,
    configs: Configs,
    uchips: UnownedChipEdit,
    texts: Vec<String>,
}

impl MyApp {
    fn new(cc: &CreationContext) -> Self {
        let configs = Configs::new(cc);
        Self {
            chip: configs.build(
                [
                    "Place",
                    "cursor",
                    "in a chip",
                    "and edit",
                    ",",
                    "delete,",
                    "or backspace",
                    ".",
                    "Chip",
                    "gets",
                    "deleted",
                    "if you",
                    "delete",
                    "when the",
                    "cursor is",
                    "outside",
                ]
                .map(|s| s.to_owned())
                .into(),
            ),
            configs: Configs::new(cc),
            uchips: UnownedChipEdit::new(&configs.separator).unwrap(),
            texts: [
                "Place",
                "cursor",
                "in a chip",
                "and edit",
                ",",
                "delete,",
                "or backspace",
                ".",
                "Chip",
                "gets",
                "deleted",
                "if you",
                "delete",
                "when the",
                "cursor is",
            ]
            .map(|s| s.to_owned())
            .into(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| ui.heading("egui chips"));
            ui.add_space(30.);
            ui.add_sized([600., 200.], &mut self.chip);
            ui.add_space(10.);
            ui.separator();
            self.uchips.show(ui, &mut self.texts);
            ui.add_space(10.);
            ui.label(format!("current values '{}'", self.chip.values().join(" ")));
            ui.add_space(10.);
            ui.separator();
            ui.add_space(10.);
            let old = self.configs.clone();

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Widget bg color");
                    color_edit_button_srgba(ui, &mut self.configs.widget_bg, Alpha::Opaque);
                    ui.end_row();
                    ui.label("Widget fg color");
                    color_edit_button_srgba(ui, &mut self.configs.widget_fg, Alpha::Opaque);
                    ui.end_row();
                    ui.label("Chip bg color");
                    ui.color_edit_button_srgba(&mut self.configs.chip_bg);
                    ui.end_row();
                    ui.label("Chip fg color");
                    ui.color_edit_button_srgba(&mut self.configs.chip_fg);
                    ui.end_row();
                    ui.label("Draw frame");
                    ui.checkbox(&mut self.configs.frame, "");
                    ui.end_row();
                    ui.label("Width");
                    ui.add(
                        egui::DragValue::new(&mut self.configs.chip_size[0])
                            .speed(0.1)
                            .range(20.0..=100.0),
                    );
                    ui.end_row();
                    ui.label("Height");
                    ui.add(
                        egui::DragValue::new(&mut self.configs.chip_size[1])
                            .speed(0.1)
                            .range(20.0..=40.0),
                    );
                    ui.end_row();
                    ui.label("Single char icon");
                    ui.add(
                        TextEdit::singleline(&mut self.configs.icon)
                            .char_limit(1)
                            .desired_width(10.),
                    );
                    ui.end_row();
                });

            if old != self.configs {
                let texts = self.chip.values();
                self.chip = self.configs.build(texts);
            }
        });
    }
}
