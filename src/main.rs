mod cli;

/// Run the CLI
fn main() {
    let action = cli::get_action();
    action.act()
}
