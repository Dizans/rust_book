use std::io;

fn fib(n: i32) -> u64{
    if n==1 {
        1
    }else if n==2{
        2
    }else{
        fib(n-1) + fib(n-2)
    }
}

mod util;
fn main(){
    let x = fib(40);
    println!("{}",x);
    let mut s = String::from("Hello");
    s.push_str(", world");
    let hello = util::functions::hello("Qingyu".to_string());
    println!("{}", hello);

}

//1. 读写文件
fn read_file(file_name: &str){
}


fn ownership(){
    // y是x的拷贝
    let x = 5;
    let y = x;

    let s1 = String::from("Hello");
    let s2 = s1;
    // move  s1所有权移动到s2了

    //Rust 永远也不会自动创建数据的 “深拷贝”

    let s1 = String::from("hello");
    let s2 = s1.clone();
    // 堆上的数据被拷贝了

    //如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用
    //不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
    //任何简单标量值的组合可以是 Copy 的 (整数类型,布尔类型,浮点数类型,char, 元组，当且仅当其包含的类型也都是 Copy 的时候)

    //函数的参数和返回值都会转移所有权

    // 1. 引用
    //& 符号就是 引用，它们允许你使用值但不获取其所有权
    // 我们将获取引用作为函数参数称为 借用（borrowing）
    // &mut s  可变引用 在特定作用域中的特定数据有且只有一个可变引用(可变引用和可变引用、不可变引用都不兼容)
    // 要么 只能有一个可变引用，要么 只能有多个不可变引用。

    //2. slice
    //slice 也不获取所有权
    //let s = String::from("hello world");
    //
    //let hello = &s[0..5];
    //let world = &s[6..=10];
    // 字符串字面值就是 slice
    // let a = [1, 2, 3, 4, 5];
    //
    // let slice = &a[1..3];
    // slice的类型是 &[i32]

}

// struct

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn struct_exp(){
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // 结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };


    // 元组结构体（tuple structs）
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, //简写法
        username,
        active: true,
        sign_in_count: 1,
    }

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32{  //还可以传&mut self
        self.width * self.height
    }
    fn can_hold(&self, other:& Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        Rectangle{ width: size, height: size}
    }
    // let sq = Rectangle::square(10);
}

//enum
enum IpAddrKind{
    V4,
    V6,
}

enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum_exp(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

enum Message{
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){

    }
}

//match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn match_exp(x:Option<i32>){
    // 匹配Option
    let z = match x {
        None => None,
        Some(i) => Some(i+1),
    };

    // 通配符_
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let
    // 处理只匹配一个模式的值而忽略其他模式的情况

    let some_u8_value = Some(0u8);
    let v = match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };

    // 这样会失去 match 强制要求的穷尽性检查
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // if let else
    // else 相当于match _

//    if let Coin::Quarter(state) = coin {
//        println!("State quarter from {:?}!", state);
//    } else {
//        count += 1;
//    }
}

// module system


// package
// module
// path


// 运行 cargo new 时是在创建一个包
// Cargo 的约定是如果在代表包的 Cargo.toml 的同级目录下包含 src 目录且其中包含 main.rs 文件的话，
// Cargo 就知道这个包带有一个与包同名的二进制 crate，且 src/main.rs 就是 crate 根。
// 另一个约定如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，
// 且 src/lib.rs 是 crate 根。crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

// 包可以带有多个二进制 crate，需将其文件置于 src/bin 目录；每个文件将是一个单独的二进制 crate。

// rust通过模块系统控制私有性

// 模块嵌套
mod sound {
    pub mod instrument {
            pub fn clarinet() {
                // 函数体
                println!("My abspath is crate::sound::instrument::clarinet()");
                super::breathe_in(); //使用super获得相对路径的好处
            }
    }

    mod voice {

    }

    fn breathe_in(){

    }
}

/*
crate
└── sound
    ├── instrument
    │   └── woodwind
    └── voice
*/

// 如果想要调用函数，需要知道其 路径。

//路径 可以有两种形式：
//
//绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
//相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。

fn use_mod(){
    // 绝对路径
    crate::sound::instrument::clarinet(); // 推荐使用
    // 相对路径
    sound::instrument::clarinet();

    // 使用use关键字来避免冗长和重复
    use crate::sound::instrument;
    instrument::clarinet();
}

// 保护规则
// 1. 不允许使用定义于当前模块的子模块中的私有代码。
// 2. 允许使用任何定义于父模块或当前模块中的代码。

// sound 模块不是公有的，不过因为 main 函数与 sound 定义于同一模块中，可以从 main 中引用 sound

mod plant {
    pub struct Vegetable { //结构体共有，但具体的字段还是要具体决定
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

// 如果有一个公有枚举，其所有成员都是公有。只需在 enum 关键词前加上 pub
mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// 对于结构体、枚举和其它项，通过 use 指定项的全路径是习惯用法。
//如: use std::collections::HashMap;
// 例外: 如果 use 语句会将两个同名的项引入作用域时，这是不允许的。
// 可以通过as重命名来解决这个问题

// 通过 pub use 重导出名称

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

/*
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet(); 重导出之后可以这样直接调用clarinet函数
*/


// 嵌套路径消除大量use语句
// use std::io::{self, Write};

// 通过 glob 运算符将所有的公有定义引入作用域
// use std::collections::*;

// 将模块分割进不同文件
// 如果我们将模块放入不同文件中,如: 将sound模块移动到sound.rs中
// 然后可以直接声明
// mod sound
// 然后就可以相当于sound模块定义在本文件中一样使用了

// 可以 在 src/sound.rs中 写pub mod instrument;
// 然后在src/sound/instrument.rs 中 pub fn clarinet() {
//    // 函数体
//}