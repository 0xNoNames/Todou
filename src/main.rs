mod models;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, styles=utils::get_styles())]
// #[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List tasks
    List {
        #[arg(short, long)]
        /// TUI to set tasks' status
        interactive: bool,

        #[arg(short = 's', long)]
        /// Display tasks' ID
        show_id: bool,

        #[arg(short = 'd', long)]
        ///  Display task set as done
        show_done: bool,
    },
    /// Add a task
    Add {
        /// Name of the task
        name: Option<String>,

        #[arg(short, long, value_enum)]
        /// Set the status
        status: Option<models::Status>,

        #[arg(short, long)]
        /// Set the category
        category: Option<String>,

        #[arg(short, long)]
        /// Set the parent task
        parent: Option<u8>,
    },

    /// Set a task as done
    Done {
        /// Specify a task's id
        #[arg(short, long)]
        id: Option<u8>,
    },

    /// Remove a task
    Remove {
        /// Specify a task's id
        #[arg(short, long)]
        id: Option<u8>,

        /// Force the removal
        #[arg(short, long)]
        force: bool,
    },

    /// Edit a task, use TUI if no options
    Edit {
        /// Specify a task's id
        id: Option<u8>,

        /// Modify the status
        #[arg(short, long)]
        status: Option<models::Status>,

        /// Modify the creation date
        #[arg(short, long)]
        creation_date: Option<String>,

        /// Modify the done date
        #[arg(short, long)]
        done_date: Option<String>,

        /// Modify the category
        #[arg(short, long)]
        category: Option<String>,

        /// Modify the parent task
        #[arg(short, long)]
        parent: Option<u8>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List {
            interactive,
            show_id,
            show_done,
        }) => {
            println!(
                "Displaying tasks..., {}, {}, {}",
                *interactive, *show_id, *show_done
            );
        }
        Some(Commands::Add {
            name,
            status,
            category,
            parent,
        }) => {
            println!("Adding task...");
        }
        Some(Commands::Done { id }) => {
            println!("Setting task as done...");
        }
        Some(Commands::Remove { id, force }) => {
            println!("Removing task...");
        }
        Some(Commands::Edit {
            id,
            status,
            creation_date,
            done_date,
            category,
            parent,
        }) => {
            println!("Editing task...");
        }
        None => {
            println!("Displaying tasks...");
        }
    }
}
