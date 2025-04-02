use clap::Args;

#[derive(Debug, Args)]
pub struct Reminder {
    pub item: String,
    pub time: String,
}
