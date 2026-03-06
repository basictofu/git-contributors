use serde::Serialize;

#[derive(Clone)]
pub struct RawAuthor {
    pub name: String,
    pub dates: Vec<u32>,
    pub range: (u32, u32),
}

pub struct Collected {
    pub data_range: (u32, u32),
    pub authors: Vec<RawAuthor>,
}

pub struct RawData {
    pub repo_name: String,
    pub data_range: (u32, u32),
    pub authors: Vec<RawAuthor>,
}

#[derive(Serialize)]
pub struct RenderableAuthor {
    pub name: String,

    pub first_commit: u32,
    pub last_commit: u32,
    pub total_commits: usize,

    pub bins: Vec<usize>,
}

#[derive(Serialize)]
pub struct RenderableData {
    pub repo_name: String,
    pub max_count: usize,
    pub authors: Vec<RenderableAuthor>,
}
