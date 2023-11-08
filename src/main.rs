// Agents v1
// What should it do? it should be able to orchestrate agents working together. "Agents can be loosely defined as perceptive autonomous programs"
// Perceptive in the sense they should be able to be always running and having inputs "streaming" into their "process"
// Autonomous in the sense they should maintain a space of possible actions and weigh decisions on which action to take
mod agent;
use clap::{Args, Parser, Subcommand, ValueEnum};
mod daemon;
mod db;
mod server;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long, default_value_t = String::from("7979"))]
        port: String,
    },
    Stop,
    Add(AddArgs),
    Rm(RmArgs),
    Ls,
}

#[derive(Args)]
struct AddArgs {
    #[arg()]
    name: String,
}

#[derive(Args)]
struct RmArgs {
    #[arg(value_enum)]
    entity: RmEntity,

    #[arg()]
    name: Option<String>,
}

#[derive(Clone, ValueEnum)]
enum RmEntity {
    Agent,
    Db,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut db = db::initialize_db().unwrap();
    match &cli.command {
        Commands::Start { port } => {
            let address = format!("{}:{}", "localhost", port);
            server::start_server(address);
        }
        Commands::Stop => daemon::kill_daemon(),
        Commands::Add(add_args) => {
            agent::util::add_agent(&add_args.name, &mut db);
        }
        Commands::Rm(rm_args) => match rm_args.entity {
            RmEntity::Agent => {
                let name = rm_args
                    .name
                    .clone()
                    .expect("Name is required for agent rm, Try agents rm bob");
                agent::util::rm_agent(&name, &mut db);
            }
            RmEntity::Db => {
                db::remove_db(db);
            }
        },
        Commands::Ls => agent::util::ls_agents(&db),
    }
}
