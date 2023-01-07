#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use]
extern crate lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref moonlight: RwLock<String> = RwLock::new("string".to_string());
}


#[get("/moonlight/<args>")]
fn cmd(args: String) -> String {
    let argsArr:Vec<&str> = args.split(' ').collect();
    std::process::Command::new(moonlight.read().unwrap().to_string()).args(argsArr).spawn().unwrap();
    return format!("{} {}", moonlight.read().unwrap(), args);
}

fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.len() > 1{
        *moonlight.write().unwrap() = args[1].clone();
    }
    else{
        panic!("unknow moonlight path");
    }

    println!("Moonlight Bin At : {}", moonlight.read().unwrap());

    rocket::ignite().mount("/", routes![cmd]).launch();
}