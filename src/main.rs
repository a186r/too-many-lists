use std::env;
use too_many_lists::TCP_Echo::{server, client};

fn main() {
    let mut args = env::args();
    args.next();
    let action = args.next().unwrap();
    if action == "s".to_string() {
        server(&args.next().unwrap()).unwrap();
    }else{
        client(&args.next().unwrap()).unwrap();
    }
}
