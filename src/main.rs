#[macro_use] extern crate rocket;

#[macro_use]
extern crate lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref MOONLIGHT: RwLock<String> = RwLock::new("string".to_string());
}


#[get("/moonlight/<args>")]
fn moonlight(args: String) -> String {
    let args_arr:Vec<&str> = args.split(' ').collect();
    std::process::Command::new(MOONLIGHT.read().unwrap().to_string()).args(args_arr).spawn().unwrap();
    return format!("{} {}", MOONLIGHT.read().unwrap(), args);
}

#[get("/get_moonlight/<args>")]
fn get_moonlight(args: String) -> String {
    let args_arr:Vec<&str> = args.split(' ').collect();
    let output = std::process::Command::new(MOONLIGHT.read().unwrap().to_string()).args(args_arr).output().unwrap();
    return format!("output: \n{} \n error: \n{}", String::from_utf8_lossy(&output.stdout).to_string(), String::from_utf8_lossy(&output.stderr).to_string());
}
#[get("/kill/<args>")]
fn kill(args: String) -> String {
    let args_arr:Vec<&str> = args.split(' ').collect();
    let output = std::process::Command::new("pkill").args(args_arr).output().unwrap();
    return format!("output: \n{} \n error: \n{}", String::from_utf8_lossy(&output.stdout).to_string(), String::from_utf8_lossy(&output.stderr).to_string());
}

#[launch]
fn rocket() -> _ {
    let args:Vec<String> = std::env::args().collect();
    if args.len() > 1{
        *MOONLIGHT.write().unwrap() = args[1].clone();
    }
    else{
        panic!("unknow moonlight path");
    }

    println!("Moonlight Bin At : {}", MOONLIGHT.read().unwrap());

    //rocket::build().mount("/hello", routes![cmd])

    rocket::build().mount("/", routes![moonlight])
    .mount("/", routes![get_moonlight])
    .mount("/", routes![kill])
    .mount("/", rocket::fs::FileServer::from("html"))

}