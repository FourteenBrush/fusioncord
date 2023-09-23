use std::sync::mpsc::Receiver;

use eframe::{CreationContext, Frame};
use egui::Context;
use fusioncord_core::message::RenderMessage;

use crate::renderer::Renderer;

pub struct Application {
    renderer: Renderer,
    #[allow(unused)] // TODO
    rx: Receiver<RenderMessage>,
}

impl Application {
    pub fn new(cc: &CreationContext<'_>, rx: Receiver<RenderMessage>) -> Self {
        Self {
            renderer: Renderer::new(cc.egui_ctx.clone()),
            rx,
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, _ctx: &Context, _frame: &mut Frame) {
        self.renderer.render_server_list(3);
        self.renderer.render_channels(5);
        self.renderer.render_messages(1000);
    }
}
