#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use nix::unistd;

#[get("/")]
fn index() -> String {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");
    format!("Hello, world, from {}!", hostname)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
