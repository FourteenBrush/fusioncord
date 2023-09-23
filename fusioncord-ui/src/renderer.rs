use egui::{pos2, Align, CentralPanel, Color32, Layout, ScrollArea, SidePanel, Stroke};

pub struct Renderer {
    ctx: egui::Context,
}

impl Renderer {
    pub fn new(ctx: egui::Context) -> Self {
        Self { ctx }
    }

    pub fn render_server_list(&mut self, servercount: u32) {
        const CIRCLE_RADIUS: f32 = 23.;
        const CIRCLE_DIAMETER: f32 = CIRCLE_RADIUS * 2.;
        const CIRCLE_MARGIN: f32 = 10.;

        SidePanel::left("servers_panel")
            .exact_width(CIRCLE_DIAMETER + 2. * CIRCLE_MARGIN)
            .show(&self.ctx, |ui| {
                for i in 0..=servercount {
                    let x = CIRCLE_MARGIN + CIRCLE_RADIUS;
                    let y = (i as f32) * (CIRCLE_DIAMETER + CIRCLE_MARGIN)
                        + CIRCLE_MARGIN
                        + CIRCLE_RADIUS;

                    ui.painter().circle(
                        pos2(x, y),
                        CIRCLE_RADIUS,
                        Color32::LIGHT_GRAY,
                        Stroke::default(),
                    );
                }
            });
    }

    pub fn render_channels(&mut self, channelcount: u32) {
        SidePanel::left("side_panel").show(&self.ctx, |ui| {
            ui.heading("Channels");

            for i in 0..=channelcount {
                ui.label(format!("channel {i}"));
            }

            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
    }

    pub fn render_messages(&mut self, messagecount: u32) {
        CentralPanel::default().show(&self.ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.heading("eframe template");
                ui.hyperlink("https://github.com/emilk/eframe_template");
                ui.add(egui::github_link_file!(
                    "https://github.com/emilk/eframe_template/blob/master/",
                    "Source code."
                ));

                for i in 0..=messagecount {
                    ui.label(format!("message {i}"));
                }
            });
            egui::warn_if_debug_build(ui);
        });
    }
}
