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
    struct User{
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User{
        email: String::from("some@example.com"),
        username: String::from("someuser1"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User{
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user1
    };

    let mut args = env::args();
    args.next();
    let action = args.next().unwrap();
    if action == "s".to_string() {
        server(&args.next().unwrap()).unwrap();
    }else{
        client(&args.next().unwrap()).unwrap();
    }

    let rect1 = Rectangle {width: 30, height: 50};
    rect1.area();
    //调用struct Rectangle的相关函数
    let rect2 = Rectangle::square(12);
    let rect3 = Rectangle::square2(22);
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