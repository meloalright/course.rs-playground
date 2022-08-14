use std::fmt::Display;
use std::convert::TryInto;

pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

pub struct Weibo {
    pub username: String,
    pub content: String
}


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("apple"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_with_clone<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in 0..list.len() {
        if list[i] > *largest {
            largest = &list[i];
        }
    }

    largest
}


fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());

    notify(&post);
    notify(&weibo);


    let s = 3.to_string();
    let m1 = returns_summarizable();
    notify(&m1);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);



    let string_list = vec!["Apple".to_string(), "Google".to_string(), "Facebook".to_string(), "MicroSoft".to_string()];

    let result = largest_with_clone(&string_list);
    println!("The largest string is {}", result);


    //

    let a: i32 = 10;
    let b: u16 = 100;
  
    let b_ = b.try_into()
              .unwrap();
  
    if a < b_ {
      println!("Ten is less than one hundred.");
    }

}
