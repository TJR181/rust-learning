use trait_summary::{notify, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Yuankan_bits"),
        content: String::from("I just found a confirmed bug on immunefi"),
        reply: true,
        retweet: true
    };
    println!("New tweet: {}", tweet.summarize());
    notify(tweet);
}
