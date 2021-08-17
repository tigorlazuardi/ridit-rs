use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub enum Subreddit {
    /// Add subreddit(s) to subscribe
    ///
    /// Examples:
    ///
    /// adding subreddit using default settings: `ridit subreddit add wallpaper wallpapers`
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
    /// Prevent nsfw tagged images from being downloaded.
    #[structopt(short, long)]
    no_nsfw: bool,
    /// Images are downloaded first before being checked for size.
    ///
    /// Not all subreddit has metadata for image size. For those kind of subreddits, you have to
    /// download them first before the size can be checked and added to list.
    ///
    /// How to know which subreddits have them? Add to subscribe list and see if any images are downloaded
    /// from them. If there's no images downloaded after making sure the settings are correct and the
    /// subreddit is in fact, an images collection subreddit, then enable this flag when adding
    /// subreddit.
    ///
    /// You can replace existing subreddit settings using the add command. It will update the
    /// settings instead of adding double entry.
    #[structopt(short, long)]
    download_first: bool,
}

#[derive(Debug, StructOpt, Clone)]
pub struct RemoveSubreddit {
    input: Vec<String>,
}
