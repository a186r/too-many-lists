use std::env;
use too_many_lists::TCP_Echo::{server, client};


fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someuser1"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user1
    };

    let mut args = env::args();
    args.next();
    let action = args.next().unwrap();
    if action == "s".to_string() {
        server(&args.next().unwrap()).unwrap();
    } else {
        client(&args.next().unwrap()).unwrap();
    }

    let rect1 = Rectangle { width: 30, height: 50 };
    rect1.area();
    //调用struct Rectangle的相关函数
    let rect2 = Rectangle::square(12);
    let rect3 = Rectangle::square2(22);

    let six = IpAddrKind::V6;
    let four = IpAddrKind::V4;

    //在使用时，可以直接使用Some或者None，前面不用加Option::,当使用None时，必须要指定T的具体类型。
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        //使用占位符_表示剩余所有的情况
        _ => (),
    }

    let some_u8_value2 = Some(0u8);
    match some_u8_value2 {
        Some(3) => println!("three"),
        _ => ()
    }

//    对于上面这种只关心一个条件的match来讲，还有一种更加简洁的办法，那就是if let
    if let Some(3) = some_u8_value2 {
        println!("three");
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //相关函数，仍然是函数，而不是方法，并且直接和struct相关，类似于java中的静态方法，调用时直接使用::
    //String::from("Hi")就是String的相关函数
    fn square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}

//rust支持为一个Struct定义多个实现代码块，但是我们并不推荐这么用。
impl Rectangle {
    fn square2(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
}

enum IpAddrKind{
    V4,
    V6,
}

enum Message{
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

//match流程控制
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cent(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

//match还支持模式中绑定值
enum UsState{
    Alabama,
    Alaska,
}

enum Coin2{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin2) -> u8{
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from{:?}!", state);
            25
        },
    }
}

//从Option<T>中提取T的值
//rust要求match必须列出所有可能的条件，
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
