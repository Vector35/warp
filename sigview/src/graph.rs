use eframe::egui;
use eframe::egui::Ui;
use egui_graphs::{Graph, GraphView, SettingsInteraction, SettingsNavigation, SettingsStyle};
use petgraph::prelude::StableGraph;
use signaturebuild::prelude::{Function, FunctionGUID};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FunctionNode {
    pub name: String,
    pub guid: FunctionGUID,
}

impl From<Function> for FunctionNode {
    fn from(value: Function) -> Self {
        Self::from(&value)
    }
}

impl From<&Function> for FunctionNode {
    fn from(value: &Function) -> Self {
        Self {
            name: value.symbol.name.to_owned(),
            guid: value.guid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionGraph {
    graph: Graph<FunctionNode, ()>,
}

impl FunctionGraph {
    pub fn from_function(func: &Function) -> Self {
        let mut graph = StableGraph::new();
        let func_node = FunctionNode::from(func);
        let func_node_idx = graph.add_node(func_node);

        for caller_func in &func.constraints.call_sites {
            let name = caller_func
                .symbol
                .as_ref()
                .map(|sym| sym.name.to_owned())
                .unwrap_or_default();
            if let Some(guid) = caller_func.guid {
                let caller_func_node = FunctionNode { name, guid };
                let caller_func_node_idx = graph.add_node(caller_func_node);
                graph.add_edge(caller_func_node_idx, func_node_idx, ());
            }
        }

        FunctionGraph {
            graph: Graph::from(&graph),
        }
    }

    pub fn add(&mut self, ui: &mut Ui) {
        let interaction_settings = &SettingsInteraction::new();
        let style_settings = &SettingsStyle::new().with_labels_always(true);
        let nav_settings = &SettingsNavigation::new()
            .with_fit_to_screen_enabled(false)
            .with_zoom_and_pan_enabled(true);
        ui.add(
            &mut GraphView::new(&mut self.graph)
                .with_styles(style_settings)
                .with_interactions(interaction_settings)
                .with_navigations(nav_settings),
        );
    }

    pub fn add_viewport(&mut self, ctx: &egui::Context) -> bool {
        let mut requested_close = false;
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("function_graph_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Function Graph")
                .with_inner_size([400.0, 400.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.add(ui);
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent viewport that we should not show next frame:
                    requested_close = true;
                }
            },
        );
        requested_close
    }
}
