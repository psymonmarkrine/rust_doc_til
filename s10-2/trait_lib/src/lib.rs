pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*

// notify(Tweet{...}, NewsArticle{...})
// notify(NewsArticle{...}, Tweet{...})
// notify(Tweet{...}, Tweet{...})
// notify(NewsArticle{...}, NewsArticle{...})
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}

// notify(Tweet{...}, Tweet{...})
// notify(NewsArticle{...}, NewsArticle{...})
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item2.summarize());
}

pub fn notify(item: &(impl Summary + Display)) {
    println!("item Display: {}, item.summarize(): {}", item, item.summarize());
}

pub fn notify<T: Summary + Display>(item: &T) {
    println!("item Display: {}, item.summarize(): {}", item, item.summarize());
}

// */
