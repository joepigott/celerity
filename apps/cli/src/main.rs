use clap::Parser;
use cli::{Cli, Command};
use color::term::Red;
use celerity::{NaiveTask, UpdateTask};

mod cli;
mod config;
mod request;
mod util;

fn main() {
    match dispatch() {
        Ok(result) => println!("{}", result.trim()),
        Err(e) => eprintln!("{}: {}", Red("Error"), e),
    }
}

fn dispatch() -> Result<String, String> {
    let config = config::config()?;
    let cli = Cli::parse();

    match cli.command {
        Command::List {
            completed,
            before,
            after,
            shorter,
            longer,
            higher,
            lower,
        } => {
            let info = util::ListInfo {
                completed,
                before,
                after,
                shorter,
                longer,
                higher,
                lower,
            };
            request::list(
                config.server.host,
                config.server.port,
                info,
                config.client.date_format,
            )
        }
        Command::Add {
            title,
            deadline,
            duration,
            priority,
        } => {
            let new_task = NaiveTask::new(title, deadline, duration, priority);
            request::add(config.server.host, config.server.port, new_task)
        }
        Command::Update {
            id,
            title,
            deadline,
            duration,
            priority,
        } => {
            let update_task = UpdateTask::new(id)
                .with_title(title)
                .with_deadline(deadline)
                .with_duration(duration)
                .with_priority(priority);
            request::update(config.server.host, config.server.port, update_task)
        }
        Command::Delete { id, completed } => {
            request::delete(config.server.host, config.server.port, id, completed)
        }
        Command::Complete { id } => request::complete(config.server.host, config.server.port, id),
        Command::Enable => request::enable(config.server.host, config.server.port, true),
        Command::Disable => request::enable(config.server.host, config.server.port, false),
        Command::Active => request::active(
            config.server.host,
            config.server.port,
            config.client.date_format,
        ),
        Command::Status => request::status(config.server.host, config.server.port),
        Command::Priority { command } => match command {
            cli::PriorityCommand::Set { priority } => {
                request::set_priority(config.server.host, config.server.port, priority)
            }
            cli::PriorityCommand::Show => {
                request::get_priority(config.server.host, config.server.port)
            }
        },
    }
}
