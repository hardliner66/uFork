use egui_quad_editor::QuadEditor;
use notan::draw::*;
use notan::egui::{self, *};
use notan::prelude::*;
use ufork::{any::Any, core::*, device::*};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
//static ALLOCATOR: lol_alloc::LeakingPageAllocator = lol_alloc::LeakingPageAllocator;
static ALLOCATOR: lol_alloc::AssumeSingleThreaded<lol_alloc::FreeListAllocator> =
    unsafe { lol_alloc::AssumeSingleThreaded::new(lol_alloc::FreeListAllocator::new()) };

#[derive(AppState)]
struct State {
    running: bool,
    core: Core,
    // collector: Option<EventCollector>,
    quad_editor: QuadEditor,
    steps_per_frame: u32,
}

impl State {
    fn new() -> Self {
        // Create a memory editor with a variety of ranges, need at least one, but can be as many as you want.
        let quad_editor = QuadEditor::new()
            .with_address_range("All", 0..0xFF)
            .with_window_title("Hello Editor!");
        let mut core = Core::new(
            DebugDevice {
                debug_print: |s| {
                    log::debug!("{:?}", s);
                },
            },
            ClockDevice {
                read_clock: || Any::new(0),
            },
            RandomDevice {
                get_random: |a, b| Any::new(rand::thread_rng().gen_range(a.raw()..b.raw())),
            },
        );
        core.set_trace_event(Box::new(|a, b| {
            tracing::trace!("{:?}", (a, b));
        }));
        {
            let quad_rom: QuadRom =
                serde_json::from_str(include_str!("../core.quad_rom.json")).unwrap();
            core.quad_rom = quad_rom.quad_rom.try_into().unwrap();
            core.rom_top = quad_rom.rom_top.try_into().unwrap();
        }
        {
            let quad_ram: QuadRam =
                serde_json::from_str(include_str!("../core.quad_ram.json")).unwrap();
            core.quad_ram = quad_ram.0.try_into().unwrap();
        }
        {
            let blob_ram: BlobRam =
                serde_json::from_str(include_str!("../core.blob_ram.json")).unwrap();
            core.blob_ram = blob_ram.0.try_into().unwrap();
        }
        {
            let gc_queue: GcQueue =
                serde_json::from_str(include_str!("../core.gc_queue.json")).unwrap();
            core.gc_queue = gc_queue.gc_queue.try_into().unwrap();
            core.gc_state = gc_queue.gc_state.try_into().unwrap();
        }
        Self {
            core,
            // collector: None,
            quad_editor,
            steps_per_frame: 1,
            running: false,
        }
    }
}

#[notan_main]
fn main() -> Result<(), String> {
    egui_logger::init().unwrap();
    // let collector = egui_tracing::EventCollector::default();
    // tracing_subscriber::registry()
    //     .with(collector.clone())
    //     .init();

    let win = WindowConfig::new()
        .set_vsync(false)
        .set_lazy_loop(false)
        .set_high_dpi(true);

    notan::init_with(State::new)
        // .initialize(move |_app: &mut App, state: &mut State| {
        //     state.collector = Some(collector);
        // })
        .add_config(win)
        .add_config(DrawConfig)
        .add_config(EguiConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, plugins: &mut Plugins, state: &mut State) {
    if state.running {
        state.core.run_loop(1);
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);

    let mut output = plugins.egui(|ctx| {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut state.steps_per_frame, 1..=100));
            if ui.button("Fullscreen").clicked() {
                let is_fullscreen = app.window().is_fullscreen();
                app.window().set_fullscreen(!is_fullscreen);
            }

            if ui.button("Pause/Unpause").clicked() {
                state.running = !state.running;
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

            egui_logger::logger_ui(ui);
            // if let Some(collector) = &state.collector {
            //     ui.add(egui_tracing::Logs::new(collector.clone()));
            // };
        });
    });

    output.clear_color(Color::BLACK);

    // if output.needs_repaint() {
    gfx.render(&output);
    // }
}
