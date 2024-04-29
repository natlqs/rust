fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Tweeting from Rust!"),
        reply: false,
        retweet: false,
    };


    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("The Associated Press"),
        content: String::from("The Pittsburgh Penguins defeated the New York Islanders 4-0 to become the Stanley Cup Championship."),
    };

    println!("1 news article: {}", news.summarize());
}

pub trait Summary{
    fn summarize(&self) -> String{
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


