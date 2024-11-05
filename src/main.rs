mod cli;
mod daemon;
mod audio;
mod config;
mod client;

use daemon::Daemon;

fn main() {
   let matches = cli::run().get_matches();

   match matches.subcommand() {
       Some(("daemon", daemon_matches)) => {
           match daemon_matches.subcommand() {
               Some(("start", _)) => {
                   Daemon::start();
               }
               Some(("stop", _)) => {
                   Daemon::stop();
               }
               Some(("restart", _)) => {
                   Daemon::restart();
               }
               Some(("status", _)) => {
                   Daemon::status();
               }
               _ => unreachable!(),
           }
       },
       Some(("play", play_matches)) => {
           let command = play_matches.get_one::<String>("COMMAND").expect("required");
           client::play(&command);
       },
       Some(("config", config_matches)) => {
           match config_matches.subcommand() {
               Some(("print", _)) => {
                   client::print_config();
               }
               _ => unreachable!(),
           }
       },
       _ => unreachable!(),
   }
}
