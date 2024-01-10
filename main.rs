use ultimate_tic_tac_toe::App;

fn main() {
    eframe::run_native(
        "Ultimate Tic Tac Toe",
        eframe::NativeOptions {
            renderer: eframe::Renderer::Wgpu,
            ..Default::default()
        },
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}
