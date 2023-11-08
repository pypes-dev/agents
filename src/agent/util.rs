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
        db.ladd("agents", &agent_instance);
    } else {
        println!("Agent with name {} already exists!", name);
    }
}

pub fn rm_agent(name: &String, db: &mut PickleDb) {
    if !agent_exists(db, name) {
        println!("Agent {} does not exist", name);
        return;
    } else {
        if let Some(agent) = get_agent(name, db) {
            db.lrem_value::<agent::Agent>("agents", &agent).unwrap();
        }
    }
}

pub fn get_agent(name: &String, db: &mut PickleDb) -> Option<agent::Agent> {
    for agent_iter in db.liter("agents") {
        let curr_agent = agent_iter.get_item::<agent::Agent>().unwrap();
        if curr_agent.name == *name {
            return Some(curr_agent);
        }
    }
    None
}

pub fn ls_agents(db: &PickleDb) {
    for agent_iter in db.liter("agents") {
        let curr_agent = agent_iter.get_item::<agent::Agent>().unwrap();
        println!("Agent {}", curr_agent.name);
    }
}

pub fn agent_exists(db: &PickleDb, name: &String) -> bool {
    for agent_iter in db.liter("agents") {
        let curr_agent = agent_iter.get_item::<agent::Agent>().unwrap();
        if curr_agent.name == *name {
            return true;
        }
    }
    return false;
}
