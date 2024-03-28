
pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

pub trait Summary {
  // fn summarize(&self) -> String; // without default implementation

  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
}

// traits as parameters
pub fn notify(item: &impl Summary) {  // any type that implements Summary
  println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify2<T: Summary>(item: &T) {  // any type that implements Summary
  println!("Breaking news! {}", item.summarize());
}

// benefit of trait bound syntax - multiple parameters with same trait but same type as well - cannot do using impl
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}    // this will work with trait bound syntax

// multiple traits
// pub fn notify3(item: &(impl Summary + Display)) {}
// pub fn notify4<T: Summary + Display>(item: &T) {}

// Cleaner trait bound syntax with where clause
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {}

// returning types that implement traits
fn returns_summarizable() -> impl Summary {
  Tweet {
      username: String::from("horse_ebooks"),
      content: String::from(
          "of course, as you probably already know, people",
      ),
      reply: false,
      retweet: false,
  }
}

// this wont work - isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler - more in chapter 17
// fn returns_summarizable(switch: bool) -> impl Summary {
//   if switch {
//       NewsArticle {// data here}
//   } else {
//       Tweet {// data here }
//   }
// }

// Using trait bounds to conditionally implement methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// only available if T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// implement trait on any type that implements Display
// impl<T: Display> ToString for T {
  // --snip--
// }

pub fn traits() {
  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
      hockey team in the NHL.",
    ),
  };

  let article_summary = article.summarize();
  println!("{}", article_summary);

  let tweet = Tweet {
    username:
    String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
  .summarize();
  println!("{}", tweet);

  notify(&article);
  notify2(&article);

  println!("{}", returns_summarizable().summarize());
}