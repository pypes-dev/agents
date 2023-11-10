// Agents v1
// What should it do? it should be able to orchestrate agents working together. "Agents can be loosely defined as perceptive autonomous programs"
// Perceptive in the sense they should be able to be always running and having inputs "streaming" into their "process"
// Autonomous in the sense they should maintain a space of possible actions and weigh decisions on which action to take
mod agent;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
mod daemon;
mod db;
mod server;

#[derive(Parser)]
#[command(
    name = "agents",
    about = "A CLI and Server to develop and interact with autonomous AI Agents\n",
    version = "0.1"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    optional_target_agent: Option<Vec<String>>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "\nStarts the server with no target  \nStarts an agent if a target is passed")]
    Start {
        #[arg(short, long, default_value_t = String::from("7979"))]
        port: String,
    },
    #[clap(about = "\nStops the server with no target  \nStops an agent if a target is passed")]
    Stop,
    #[clap(
        about = "\nAdds an agent with a name  \nAdd inputs or action to an agent if a target is passed"
    )]
    Add(AddArgs),
    #[clap(
        about = "\nRemoves an agent with a name  \nRemoves inputs or action to an agent if a target is passed"
    )]
    Rm(RmArgs),
    #[clap(about = "\nLists agents  \nLists an agents inputs and actions if a target is passed")]
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
    let mut app = Cli::command();
    let cli = Cli::parse();
    if let Some(target) = &cli.optional_target_agent {
        println!("CLI.OTHER");
        if target.len() == 1 {
            app.print_long_help().unwrap();
            return;
        }
        for o in target.iter() {
            println!("o {} ", o);
        }

        return;
    }
    let mut db = db::initialize_db().unwrap();
    match &cli.command {
        Some(Commands::Start { port }) => {
            let address = format!("{}:{}", "localhost", port);
            server::start_server(address);
        }
        Some(Commands::Stop) => daemon::kill_daemon(),
        Some(Commands::Add(add_args)) => {
            agent::util::add_agent(&add_args.name, &mut db);
        }
        Some(Commands::Rm(rm_args)) => match rm_args.entity {
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
        Some(Commands::Ls) => agent::util::ls_agents(&db),
        None => {
            app.print_help().expect("Failed to print help");
            println!(); // Print a newline after the help message   },
        }
    }
}
