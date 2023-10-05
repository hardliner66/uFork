use egui_memory_editor::MemoryEditor;
use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;
use ufork::any::Any;
use ufork::host::Host;

#[derive(AppState)]
struct State {
    memory: Vec<u8>,
    host: Host,
    mem_editor: MemoryEditor,
}

impl State {
    fn new() -> Self {
        // Create a memory editor with a variety of ranges, need at least one, but can be as many as you want.
        let mut mem_editor = MemoryEditor::new()
            .with_address_range("All", 0..0xFF)
            .with_window_title("Hello Editor!");
        Self {
            host: Host::new(),
            memory: vec![0; 0x10000],
            mem_editor,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .set_vsync(true)
        .set_lazy_loop(true)
        .set_high_dpi(true);

    notan::init_with(State::new)
        .add_config(win)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut output = plugins.egui(|ctx| {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Egui Plugin Example");
            if ui.button("Fullscreen").clicked() {
                let is_fullscreen = app.window().is_fullscreen();
                app.window().set_fullscreen(!is_fullscreen);
            }

            // In your egui rendering simply include the following.
            // The write function is optional, if you don't set it the UI will be in read-only mode.
            let mut is_open = true;
            state.mem_editor.window_ui_read_only(
                ctx,
                &mut is_open,
                &mut state.host.core(),
                |mem, address| Some(mem.ram(Any::ram(address)).t().raw().try_into().unwrap()),
            );
        });
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}
