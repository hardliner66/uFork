use egui_quad_editor::QuadEditor;
use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;
use ufork::any::Any;
use ufork::core::Core;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {
    ::core::unreachable!()
}

#[cfg(target_arch = "wasm32")]
#[global_allocator]
//static ALLOCATOR: lol_alloc::LeakingPageAllocator = lol_alloc::LeakingPageAllocator;
static ALLOCATOR: lol_alloc::AssumeSingleThreaded<lol_alloc::FreeListAllocator> =
    unsafe { lol_alloc::AssumeSingleThreaded::new(lol_alloc::FreeListAllocator::new()) };

#[derive(AppState)]
struct State {
    core: Core,
    quad_editor: QuadEditor,
}

impl State {
    fn new() -> Self {
        // Create a memory editor with a variety of ranges, need at least one, but can be as many as you want.
        let quad_editor = QuadEditor::new()
            .with_address_range("All", 0..0xFF)
            .with_window_title("Hello Editor!");
        Self {
            core: Core::default(),
            quad_editor,
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
            state.quad_editor.window_ui_read_only(
                ctx,
                &mut is_open,
                &mut state.core,
                |mem, address| mem.ram(Any::ram(address)).t().raw().try_into().ok(),
            );
        });
    });

    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
}
