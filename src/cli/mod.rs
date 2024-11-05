use clap::{Command, arg};


pub fn run() -> Command {
    Command::new("termsfx")
        .version("0.1.0")
        .author("Corsin Conzett <corsinconzett@gmail.com>")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("daemon")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("start")
                        .about("Start the daemon")
                        .arg_required_else_help(false)
                )
                .subcommand(
                    Command::new("stop")
                        .about("Stop the daemon")
                        .arg_required_else_help(false)
                )
                .subcommand(
                    Command::new("restart")
                        .about("Restart the daemon")
                        .arg_required_else_help(false)
                )
                .subcommand(
                    Command::new("status")
                        .about("Get the status of the daemon")
                        .arg_required_else_help(false)
                )
        )
        .subcommand(
            Command::new("play")
                .about("Play a sound for a given command")
                .arg(arg!(<COMMAND> "The command to play the sound for"))
        )
        .subcommand(
            Command::new("config")
                .about("Config related commands")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("print")
                        .about("Print the current configuration")
                        .arg_required_else_help(false)
                )
        )
}
