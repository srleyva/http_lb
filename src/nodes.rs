use rocket_contrib::json::Json;
use serde_derive::{Serialize, Deserialize};
use rocket::State;
use std::sync::{Mutex, Arc};
use std::fmt;

use consistent_hashing;

use rocket::response::{Responder};


#[derive(Serialize, Deserialize, Clone)]
pub struct Node {
    hostname: String,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hostname: {}", self.hostname)
    }}

impl consistent_hashing::Evict for Node {
    fn evict(self) -> Self {
        return self;
    }

    fn merge(&mut self, _item: Node) -> () {
        // TODO Move keys
        ()
    }
}

 pub struct NodesCollection {
    pub nodes: Arc<Mutex<consistent_hashing::ConsistentHash<String, Node>>>
 }

 impl NodesCollection {
     pub fn is_empty(&self) -> bool {
         return self.nodes.lock().unwrap().user_keys.is_empty();
     }
 }
 
 #[derive(Responder)]
 pub enum NodesResponder {
     #[response(status = 200)]
     Found(Json<Vec<String>>),
 }

#[get("/nodes")]
pub fn get_nodes(redis_nodes: State<NodesCollection>) -> NodesResponder {
    let redis_nodes = Arc::clone(&redis_nodes.nodes);
    let redis_nodes = &*redis_nodes.lock().unwrap();
    NodesResponder::Found(Json(redis_nodes.user_keys.clone()))
}

#[derive(Responder)]
pub enum NodeResponder {
    #[response(status = 404)]
    NotFound(String),

    #[response(status = 200)]
    Found(Json<Node>)
}

#[get("/nodes/<key>")]
pub fn get_node(key: String, redis_nodes: State<NodesCollection>) -> NodeResponder {

    if redis_nodes.is_empty() {
        return NodeResponder::NotFound(key);
    }

    let redis_nodes = Arc::clone(&redis_nodes.nodes);
    let redis_nodes = redis_nodes.lock().unwrap();

    NodeResponder::Found(Json(redis_nodes.get_node(&key).unwrap().clone()))
}

#[post("/nodes", data = "<node>")]
pub fn post_node(node: Json<Node>, redis_nodes: State<'_, NodesCollection>) {
    let redis_nodes = Arc::clone(&redis_nodes.nodes);
    let mut redis_nodes = redis_nodes.lock().unwrap();
    match redis_nodes.add_node(node.hostname.to_string(), node.clone()) {
        Err(_) => println!("node exists"),
        Ok(_) => (),
    }
}

