#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub trait Summarizable {
    fn summary(&self) -> String;
    fn author_summary(&self) -> String;

    // default impl
    // can call other methods
    // can't call dafault impl from overriding impl
    fn def_summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

// impl Summarizable for NewsArticle {} for directly use default impl
impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

// --- trait bound on generic
pub fn notify<T: Summarizable>(item: &T) {
    println!("Breaking news! {}", item.summary());
}

/* too long...
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
}
*/

/* with where
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
}
*/
