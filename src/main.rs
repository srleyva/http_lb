#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, World"
}

mod nodes;

use std::sync::{Arc, Mutex};

use consistent_hashing;

fn main() {
    rocket::ignite()
        .manage(nodes::NodesCollection { nodes: Arc::new(Mutex::new(consistent_hashing::ConsistentHash::new(1000).unwrap()))})
        .mount("/v1", routes![index, nodes::get_nodes, nodes::post_node, nodes::get_node]).launch();
}