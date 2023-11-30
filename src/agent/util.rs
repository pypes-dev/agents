use crate::agent::agent;
use pickledb::PickleDb;
pub fn add_agent(name: &String, db: &mut PickleDb) {
    if !agent_exists(&db, name) {
        let agent_instance = agent::Agent {
            name: name.clone(),
            inputs: Vec::new(),
            actions: Vec::new(),
        };
        println!("Added {}", name);
        db.set(name, &agent_instance).unwrap();
    } else {
        println!("Agent with name {} already exists!", name);
    }
}

pub fn rm_agent(name: &String, db: &mut PickleDb) {
    if !agent_exists(db, name) {
        println!("Agent {} does not exist", name);
        return;
    } else {
        db.rem(name).unwrap();
    }
}

pub fn get_agent(name: &String, db: &mut PickleDb) -> Option<agent::Agent> {
    let keys = db.get_all();
    for key in keys.iter() {
        let curr_agent = db.get::<agent::Agent>(key).unwrap();
        if curr_agent.name == *name {
            return Some(curr_agent);
        }
    }
    None
}

pub fn ls_agents(db: &PickleDb) {
    let keys = db.get_all();
    for key in keys {
        let curr_agent = db.get::<agent::Agent>(&key).unwrap();
        println!("Agent {}", curr_agent.name);
    }
}

pub fn agent_exists(db: &PickleDb, name: &String) -> bool {
    return db.exists(name);
}
