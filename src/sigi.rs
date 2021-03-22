/// Sigi, a tool for organizing.
///
/// TODO: Add guidance on using sigi as a library.
///
pub mod actions;
mod cli;
pub mod data;
pub mod items;

pub fn run() {
    let action = cli::get_action();
    action.act()
}
