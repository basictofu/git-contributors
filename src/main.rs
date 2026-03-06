mod cli;
mod collect;
mod data;
mod dates;
mod formatting;
mod git;
mod histogram;
mod interactive;
mod parsing;
mod process;
mod shell;
mod sparklines;

use serde_json;

fn main() -> std::io::Result<()> {
    let args = cli::Args::run_parse();

    // Collect "raw" data from git (minimal processing => should be fast)
    let repo_name = git::get_current_repo_name();
    let collected = git::collect_git_log_data();

    let raw_data = data::RawData {
        repo_name,
        data_range: collected.data_range,
        authors: collected.authors,
    };

    // Determine range to display
    let resolved_range = dates::resolve_date_range(raw_data.data_range, &args);

    if args.json {
        let numbins: usize = args.numbins.expect("--numbins is required for json output");
        let renderable_data = process::generate_renderable_data(&raw_data, resolved_range, numbins);
        let serialized = serde_json::to_string(&renderable_data).unwrap();
        println!("{}", serialized);
        return Ok(());
    } else if args.print {
        cli::print_to_terminal(&raw_data, resolved_range, args.numbins);
        return Ok(());
    } else {
        return interactive::run_interactive(raw_data, resolved_range);
    }
}
