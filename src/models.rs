// #[derive(clap::ValueEnum, Clone, Debug)]
#[derive(clap::ValueEnum, Clone)]
pub enum Status {
    TODO,
    DOING,
    DONE,
}
