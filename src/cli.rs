use crate::data::{RawAuthor, RawData, RenderableAuthor};
use crate::formatting::generate_axis;
use crate::process::generate_renderable_data;
use crate::sparklines::spark;
use clap::Parser;
use crossterm::terminal;
use std::vec::Vec;

/// Arguments to main CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Start date, in git's default format, e.g. --start="2023-10-27 14:00:00"
    /// Dates default to local time. For UTC, use the "Z" suffix, e.g. "2023-10-27 14:00:00Z"
    #[arg(long)]
    pub start: Option<String>,

    /// End date. Same format as "--start"
    #[arg(long)]
    pub end: Option<String>,

    /// Limit number of authors to display, e.g. "--limit 5"
    #[arg(long)]
    pub limit: Option<usize>,

    /// Constrain the number of histogram bins (for use with scripts)
    #[arg(long)]
    pub numbins: Option<usize>,

    /// Only print sparklines (do not open interactive view)
    #[arg(short, long, default_value = "false")]
    pub print: bool,

    /// Only print json output (for use with scripts). See source code for shape.
    #[arg(long, default_value = "false")]
    pub json: bool,
}

impl Args {
    pub fn run_parse() -> Args {
        Args::parse()
    }
}

pub fn print_to_terminal(
    raw_data: &RawData,
    resolved_range: (u32, u32),
    user_specified_bins: Option<usize>,
) {
    let mut max_name_width = 0;
    for RawAuthor {
        name,
        dates: _,
        range: _,
    } in &raw_data.authors
    {
        if name.len() > max_name_width {
            max_name_width = name.len();
        }
    }
    let name_width = if max_name_width < 30 {
        max_name_width
    } else {
        30
    };

    let gap = " ";

    let numbins: usize = match user_specified_bins {
        Some(val) => val,
        None => {
            // Limit histogram width to terminal
            let (term_width, _) =
                terminal::size().expect("Could not determine terminal size for output.");
            (term_width as usize) - name_width - gap.len()
        }
    };
    if numbins <= 0 {
        panic!("Cannot print to terminal: terminal is too narrow.")
    }

    let renderable_data = generate_renderable_data(raw_data, resolved_range, numbins);

    // Scale each bin down, then draw sparklines
    let mut lines: Vec<String> = vec![];
    for RenderableAuthor {
        name: author,
        bins,
        first_commit: _,
        last_commit: _,
        total_commits: _,
    } in &renderable_data.authors
    {
        let scale = 8.0 / (renderable_data.max_count as f64);
        let heights = bins
            .iter()
            .map(|c| (scale * (*c as f64)).floor() as u8)
            .collect();
        let sparkline = spark(heights);
        let author_padded = if author.len() > name_width {
            author[..name_width].to_string()
        } else {
            author.to_string() + &" ".repeat(name_width - author.len())
        };
        let line = format!("{}{}{}", author_padded, gap, sparkline);
        lines.push(line);
    }

    let axis = &generate_axis(numbins as u64, resolved_range, None);
    let axis_padded = " ".repeat(name_width) + gap + axis;
    let text = axis_padded.to_string() + "\n" + &lines.join("\n");

    // NOTE: this has to be printed as one entire string, not line-by
    // line, because otherwise it panics when piped to `head -10`, for
    // example
    println!("{}", text);
}
