/// Sigi, a tool for organizing.
///
/// TODO: Add guidance on using sigi as a library.
///
mod actions;
mod cli;
mod data;
mod items;

pub fn run() {
    let action = cli::get_action();
    action.act()
}
