/// Run the GUI
fn main() {
    let exit_code = sigi::gui::run();
    std::process::exit(exit_code);
}
