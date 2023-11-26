// What is an agent? - Perceptive, autonomous process
// Agent struct has an array of inputs and an array of actions
//
//
// How do you make it perceptive?
// Continously streaming inputs of course!
// Each input should correspond with a url endpoint to post/stream data to;
//
// How do you make it autonomous
// Continously choosing an action over an action space of course!
// Each action should be a function

// Actions should be subscribed to the event lifecycle of an input

// if I have an action A that subscribes to creation of input B, everytime B receives a post request it should run action A
// if I have an action A that subscribes to input B(x), everytime B receives X it should run action A

use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
#[derive(PartialEq, Deserialize, Serialize)]
pub struct Agent {
    pub name: String,
    pub inputs: Vec<String>,
    pub actions: Vec<String>,
}

impl Agent {
    pub fn add_input(&self, json_str: &String) -> Option<Value> {
        let parsed_value = serde_json::from_str(&json_str);
        match parsed_value {
            Ok(val) => return val,
            Err(e) => {
                println!("{}\nUnable to parse json string {}", e, json_str);
                None
            }
        }
    }
}
