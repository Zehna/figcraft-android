//! FigCraft application library and Android entry point.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

pub mod args;
pub mod control_server;
pub mod gamepad;
pub mod graphics;
pub mod recovery;
pub mod services;
pub mod startup;

#[cfg(target_os = "macos")]
pub mod apple_events;

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    std::thread::spawn(|| {
        figcraft_text::scan_system_fonts();
    });

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "FigCraft",
        options,
        Box::new(|cc| {
            let session = figcraft_engine::Session::new();
            let services = services::Desktop::load_from(None, recovery::Shared::default());
            let mut fig = figcraft_ui_egui::FigApp::new(session, Box::new(services));
            fig.services.set_egui_context(&cc.egui_ctx);
            Ok(Box::new(AndroidShell { fig }))
        }),
    );
}

#[cfg(target_os = "android")]
struct AndroidShell {
    fig: figcraft_ui_egui::FigApp,
}

#[cfg(target_os = "android")]
impl eframe::App for AndroidShell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        figcraft_ui_egui::windows::guard(&mut self.fig, ctx, |fig| fig.background_tick(ctx));
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        let events = figcraft_ui_egui::windows::take_input(&mut self.fig, raw_input.viewport_id);
        raw_input.events.extend(events);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.fig.ui(ui);
    }
}
