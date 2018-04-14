extern crate hyper;
extern crate rustc_serialize;
extern crate bbs;

use std::env;
use std::io::{self, Write};

use bbs::{UserClient, HTML_ADDR};

fn msg_loop(user: String) {

    loop {
        let mut msg = String::new();
        print!("> ");
        io::stdout().flush().expect("Error flushing output...");
        io::stdin().read_line(&mut msg).expect("Error while processing message!");
        if msg.starts_with("quit") || msg.starts_with("Quit") {
            return;
        }
        println!("User: {}, Msg: {}", user, msg);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You need to specify user, ex:");
        println!("client Alex");
    } else {
        msg_loop(args[1..].join(" "));
    }
}
