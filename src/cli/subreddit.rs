use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub enum Subreddit {
    /// Add subreddit(s) to subscribe
    Add(AddSubreddit),
    /// Remove subreddit(s) from subscription
    Remove(RemoveSubreddit),
}

impl Subreddit {
    pub fn handle(&self, profile: &str) {
        match &self {
            Self::Add(add) => Self::add_subreddit(add, profile),
            Self::Remove(rem) => Self::remove_subreddit(rem, profile),
        }
    }

    fn add_subreddit(add: &AddSubreddit, profile: &str) {}

    fn remove_subreddit(remove: &RemoveSubreddit, profile: &str) {}
}

#[derive(Debug, StructOpt, Clone)]
pub struct AddSubreddit {
    input: Vec<String>,
    #[structopt(short, long)]
    no_nsfw: bool,
}

#[derive(Debug, StructOpt, Clone)]
pub struct RemoveSubreddit {
    input: Vec<String>,
}
