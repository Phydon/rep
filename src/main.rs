use std::process::Command;
use std::{env, fs, io};
use std::io::Write;

fn main() {
    let mut env_args = Vec::new();
    
    for arg in env::args().skip(1) {
        env_args.push(arg);
    }

    if env_args.is_empty() {
        eprintln!("USAGE: rep [COMMAND] <FLAGS>");
        std::process::exit(1);
    }

    log_cmd(&mut env_args).expect("Failed to log command");

    let cmd = &env_args[0];
    let parameters = &env_args[1..];
    
    execute_command(&cmd, &parameters);
}

fn execute_command(cmd: &String, parameters: &[String]) {
    if cfg!(target_os = "windows") {
        todo!()
    } else {
        // TODO
        // may cause problems with selfmade aliases
        // e.g. error: "unknown command: sf"
        // -> defining aliases as ENV variables could solve this
        let output = Command::new(cmd).args(parameters).status();
        match output {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("Unknown command: {:} {:}", cmd, parameters.join(" "));
                std::process::exit(1);
            }
        };
    }
}

fn log_cmd(cmd_list: &mut Vec<String>) -> io::Result<()> {
    let file_path = "/home/phydon/main/rep/cmd_log.txt";

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;

    writeln!(file, "{:}", cmd_list.join(" "))?;
    Ok(())
}
