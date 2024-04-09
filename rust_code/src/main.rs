mod display_use;
mod list_display_use;
mod format_use;
mod tuple_use;
mod list_use;

use log::debug;

fn fmt_use() {
    // std::fmt 定义的宏处理打印
    // 实现至少一个可打印的 traits
    println!("Let's print some Lines:");
    println!();
    println!("Hello, world");
    println!("{}, {}", "hello", "world");
    println!("Hello, world!");
    println!("{number:>0width$}", number = 1, width = 6);
    eprintln!("{subject} {verb} {object}",
              object = "the lazy dog",
              subject = "the quick brown fox",
              verb = "jumps over");
}

fn format_use() {
    //format!使用
}

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// debug 通过推导实现
fn debug_use() {
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    println!("{:?}", Structure(12));
    println!("{:?}", Deep(Structure(7)));
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's")
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn beautiful_use() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
fn main() {
    // println!("{:#?}", display_use::Stru(3));
    // list_display_use::list_display_use();
    // format_use::format_use();
    // tuple_use::tuple_use();
    list_use::list_use();
}