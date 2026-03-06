use crate::{
    data::{RawAuthor, RawData, RenderableAuthor, RenderableData},
    histogram::hist,
};

pub fn generate_renderable_data(
    raw_data: &RawData,
    render_range: (u32, u32),
    numbins: usize,
) -> RenderableData {
    let mut out = RenderableData {
        repo_name: raw_data.repo_name.clone(),
        max_count: 0,
        authors: vec![],
    };

    // Generate histogram for each author
    for RawAuthor {
        name: author,
        dates,
        range: author_range,
    } in &raw_data.authors
    {
        let bins = hist(&dates, render_range, numbins);
        for &bin in &bins {
            if bin > out.max_count {
                out.max_count = bin;
            }
        }
        let renderable_author = RenderableAuthor {
            name: author.to_string(),
            first_commit: author_range.0,
            last_commit: author_range.1,
            total_commits: dates.len(),
            bins,
        };
        out.authors.push(renderable_author)
    }

    out
}
