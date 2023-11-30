// Agents v1
// What should it do? it should be able to orchestrate agents working together. "Agents can be loosely defined as perceptive autonomous programs"
// Perceptive in the sense they should be able to be always running and having inputs "streaming" into their "process"
// Autonomous in the sense they should maintain a space of possible actions and weigh decisions on which action to take
mod agent;
use clap::{Args, Parser, Subcommand, ValueEnum};
use pickledb::PickleDb;
mod daemon;
mod db;
mod server;
use server::server::{start_server, status};

#[derive(Parser)]
#[command(
    name = "pypes",
    about = "A CLI and Server to develop and interact with autonomous AI Agents\n",
    version = "0.1"
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Starts the agents server")]
    Start(StartArgs),
    #[clap(about = "Stops the agents server")]
    Stop,
    #[clap(about = "Get the running status of the server\n")]
    Status,
    #[clap(about = "Adds an agent with a name")]
    Add(AddArgs),
    #[clap(about = "Removes an agent with a name or removes the db")]
    Rm(RmArgs),
    #[clap(about = "Lists agents\n")]
    Ls,
    #[clap(about = "Subcommand to interact with agents")]
    Agent(AgentCommandArgs),
}

#[derive(Parser)]
struct AgentCommandArgs {
    #[arg(help = "Specifies the name of the agent to interact with.")]
    agent_name: String,

    #[command(subcommand)]
    command: AgentCommands,
}

#[derive(Subcommand)]
enum AgentCommands {
    #[clap(about = "Adds an input or an action to the specified agent.")]
    Add {
        #[arg(help = "Specifies the input or action to be added to the agent.")]
        input: String,
    },
}

#[derive(Args)]
struct StartArgs {
    #[arg(
        short,
        long,
        default_value_t = String::from("7979"),
        help = "Sets the port number for the server to listen on. Defaults to 7979.",
    )]
    port: String,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Determines whether to attach the server to the current terminal session."
    )]
    attatch: bool,
}

#[derive(Args)]
struct AddArgs {
    #[arg(help = "Specifies the name of the agent to be added.")]
    name: String,
}

#[derive(Args)]
struct RmArgs {
    #[arg(
        value_enum,
        help = "Specifies whether to remove an agent or the entire database."
    )]
    entity: RmEntity,

    #[arg(help = "The name of the agent to remove, if the target is an agent.")]
    name: Option<String>,
}

#[derive(Clone, ValueEnum)]
enum RmEntity {
    Agent,
    Db,
}
fn main() {
    let cli = Cli::parse();
    let mut db = db::initialize_db().unwrap();
    match &cli.command {
        Commands::Start(start_args) => {
            start_server(&start_args.port, &start_args.attatch, db);
        }
        Commands::Stop => daemon::kill_daemon(),
        Commands::Status => status(&mut db.config_db),
        Commands::Add(add_args) => {
            agent::util::add_agent(&add_args.name, &mut db.agents_db);
        }
        Commands::Rm(rm_args) => match rm_args.entity {
            RmEntity::Agent => {
                let name = rm_args
                    .name
                    .clone()
                    .expect("Name is required for agent rm, Try agents rm bob");
                agent::util::rm_agent(&name, &mut db.agents_db);
            }
            RmEntity::Db => {
                db::remove_db(db.agents_db);
            }
        },
        Commands::Ls => agent::util::ls_agents(&db.agents_db),
        Commands::Agent(agent_args) => handle_agent_command(agent_args, &mut db.agents_db),
    }
}

fn handle_agent_command(agent_args: &AgentCommandArgs, db: &mut PickleDb) {
    match &agent_args.command {
        AgentCommands::Add { input } => {
            let mut agent = match agent::util::get_agent(&agent_args.agent_name, db) {
                Some(agent) => agent,
                None => {
                    println!("Agent not found: {}", agent_args.agent_name);
                    return;
                }
            };
            match agent.add_input(input) {
                Some(_value) => agent.write_agent_update(db),
                None => return,
            }
        }
    }
}
