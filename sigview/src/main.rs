mod graph;

use std::path::Path;
// ui design is my passion
use crate::graph::FunctionGraph;
use eframe::egui;
use eframe::egui::{Direction, Layout, ScrollArea, Ui};
use egui_virtual_list::VirtualList;
use warp::signature::Data;
use warp::signature::function::Function;
// TODO: Add some collision viewer and graph viewer
// TODO: Load multiple Data for collision.
// TODO: Type viewer (data.types)

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 540.0]),
        ..Default::default()
    };
    eframe::run_native(
        "sigview",
        options,
        Box::new(|_cc| Ok(Box::<SigView>::default())),
    )
}

#[derive(Default)]
struct SigView {
    data: Option<Data>,
    virtual_list: VirtualList,
    selected_graph: Option<FunctionGraph>,
    selected_function_info: Option<Function>,
    search_query: String,
}

impl SigView {
    #[allow(unused)]
    pub fn from_data(data: Data) -> Self {
        Self {
            data: Some(data),
            selected_graph: None,
            selected_function_info: None,
            virtual_list: VirtualList::new(),
            search_query: "".to_string(),
        }
    }

    pub fn rebuild_from_file(&mut self, path: &Path) {
        if let Ok(buf) = std::fs::read(path) {
            if let Some(data) = Data::from_bytes(&buf) {
                self.data = Some(data);
            } else {
                log::error!("Could not get data from file: {:?}", path);
            }
        } else {
            log::error!("Could not read file: {:?}", path);
        }
    }

    pub fn file_selector_btn(&mut self, ui: &mut Ui) {
        if ui.button("Open File or Drop File").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.rebuild_from_file(&path);
            }
        }
    }

    pub fn file_dropper_handler(&mut self, ui: &mut Ui) {
        // Handle dropped files.
        ui.ctx().input(|i| {
            for file in &i.raw.dropped_files {
                if let Some(path) = &file.path {
                    self.rebuild_from_file(path);
                }
            }
        });
    }
}

impl eframe::App for SigView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(mut graph) = self.selected_graph.clone() {
            if graph.add_viewport(ctx) {
                self.selected_graph = None;
            }
        }

        if let Some(function) = self.selected_function_info.clone() {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("function_info_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Function Info")
                    .with_inner_size([400.0, 400.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label(format!("Name: {}", function.symbol.name));
                            ui.separator();
                            ui.label(format!("GUID: {}", function.guid));
                            ui.separator();
                            ui.label(format!("{:#?}", function.ty));
                            ui.separator();
                            ui.label(format!("{:#?}", function.constraints));
                            ui.separator();
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.selected_function_info = None;
                    }
                },
            );
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.file_dropper_handler(ui);
            if let Some(data) = &self.data {
                ui.label(format!("Data with {} functions...", data.functions.len()));

                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search...")
                    .show(ui);

                ui.separator();

                let filtered_functions = data
                    .functions
                    .iter()
                    .filter(|f| {
                        f.symbol
                            .name
                            .to_lowercase()
                            .contains(&self.search_query.to_lowercase())
                            || f.guid.to_string().contains(&self.search_query)
                    })
                    .collect::<Vec<_>>();

                ScrollArea::vertical().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    self.virtual_list.ui_custom_layout(
                        ui,
                        filtered_functions.len(),
                        |ui, start_index| {
                            let function = filtered_functions[start_index];
                            ui.horizontal(|ui| {
                                ui.label(format!("Function: {}", function.symbol.name));
                                if ui.button("View Graph").clicked() {
                                    self.selected_graph =
                                        Some(FunctionGraph::from_function(function));
                                }
                                if ui.button("View Info").clicked() {
                                    self.selected_function_info = Some(function.clone());
                                }
                            });
                            1
                        },
                    );
                });
            } else {
                ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                    self.file_selector_btn(ui);
                });
            }
        });
    }
}
