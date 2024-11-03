mod cli;

fn main() {
   let matches = cli::run().get_matches();

   match matches.subcommand() {
       Some(("daemon", daemon_matches)) => {
           match daemon_matches.subcommand() {
               Some(("start", _)) => {
                   println!("Starting daemon");
               }
               Some(("stop", _)) => {
                   println!("Stopping daemon");
               }
               Some(("restart", _)) => {
                   println!("Restarting daemon");
               }
               Some(("status", _)) => {
                   println!("Getting daemon status");
               }
               _ => unreachable!(),
           }
       },
       Some(("play", play_matches)) => {
           let command = play_matches.get_one::<String>("COMMAND").expect("required");
           println!("Playing sound for command: {}", command);
       },
       _ => unreachable!(),
   }
}
