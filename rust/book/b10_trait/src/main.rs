extern crate b10_trait;

use b10_trait::Tweet;
use b10_trait::Summarizable;
use b10_trait::notify;

fn main() {
    let tweet  = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summary());
    println!("{}", tweet.def_summary());
    notify(&tweet);
}

// import the trait and impl by the client
// can't impl external trait for external type
// orphan rule???
struct WeatherForcast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64
}

impl Summarizable for WeatherForcast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }

    fn author_summary(&self) -> String {
        String::from("@Weather_forcaster")
    }
}
