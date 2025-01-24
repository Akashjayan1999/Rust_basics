use std::fmt::Display;

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,

}

pub struct Tweet {
    pub user: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait NewsItem {
    fn author(&self) -> &str;
    fn headline(&self) -> &str;
    fn content(&self) -> &str;
}

pub trait Summary{
    fn summarize(&self) -> String {
        String::from("Unimplemented")
    }
    fn summarize2(&self) -> String;

    fn summarize3(&self) -> String {
       format!("{}", self.summarize2())
    }


    fn summarize4() -> String {  // can be called without an instance not work
       format!("{}", "Static method")
    }
} 
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
    fn summarize2(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
   
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.user, self.content)
    }
    fn summarize2(&self) -> String {
        format!("{}: {}", self.user, self.content)
    }
}

pub fn main(){
    let tweet = Tweet {
        user: "@horse_ebooks".to_string(),
        content: "My cat is cute!".to_string(),
        reply: false,
        retweet: false,
    };
    let news_article = NewsArticle {
        headline: "Rust is awesome!".to_string(),
        author: "Jane Smith".to_string(),
        content: "Rust is such an awesome language!".to_string(),
    };
    println!("{}", tweet.summarize());
    println!("{}", news_article.summarize());
    println!("{}", Summary::summarize(&tweet));
    println!("{}", news_article.summarize2());
    println!("{}", news_article.summarize3());
    println!("{}", Summary::summarize3(&tweet));
    // println!("{}", Summaryarize4()); //not possible

    pub fn notify(item : &impl Summary) {
        println!("{}", item.summarize());
    }
      notify(&tweet);
    // pub fn notify<T:Summary>(item : &T) { both are same
    //     println!("{}", item.summarize());
    // }

    pub fn notify3(item : &(impl Summary + NewsItem)) {
        println!("{}", item.summarize());
    }

    pub fn notify6<T:Summary + NewsItem>(item : &T) {
        println!("{}", item.summarize());
       
    }
     

    pub fn notify4(item:&impl Summary , item2: &impl Summary) {
        println!("{}", item.summarize());
        
    }
    pub fn notify2<T:Summary>(item : &T, item2: &T) { //both item and item2 are exact same type
        println!("{}", item.summarize());
    }
    
    fn notify5<T,U>(item:&T , item2: &U)->String 
    where T:Summary, U:Summary+NewsItem {
        format!("{} {}", item.summarize(), item2.summarize())

    }

    fn return_tweet()-> Tweet {
        Tweet {
            user: "@horse_ebooks".to_string(),
            content: "My cat is cute!".to_string(),
            reply: false,
            retweet: false,
        }
    }

    fn return_summary()-> impl Summary {
        Tweet {
            user: "@horse_ebooks".to_string(),
            content: "My cat is cute!".to_string(),
            reply: false,
            retweet: false,
        }
    }

    print!("{}", return_tweet().summarize());
     print!("return_summary {}", return_summary().summarize());

     struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        
    }

    impl<T: Display + PartialOrd> Pair<T>{
        fn cmp(&self) {
            if self.x > self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }


    // impl <T: Display> ToString for T {

    // }

    // impl<T:Display> ToString for Pair<T> {
    //    // fn to_string(&self) -> String {
    //    //     format!("Pair({}, {})", self.x, self.y)
    //    // }
    //     fn to_string(&self) -> String {
    //         format!("Pair({}, {})", self.x, self.y)
    //     }
    // }


}