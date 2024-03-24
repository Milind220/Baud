mod cli;
mod commands;

fn main() {
    let matches = cli::build_cli().get_matches();
    commands::handle_matches(matches);
}
