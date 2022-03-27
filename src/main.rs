use std::process::Command;
use std::env;

fn main() {
    let mut env_args = Vec::new();
    // TODO write env_args/command to file -> replace this and then read from file
    let mut cmd_container = Vec::new();

    for arg in env::args().skip(1) {
            let arg_cont = arg.clone();
            cmd_container.push(arg_cont);
            env_args.push(arg);
    }
    
    if env_args.is_empty() {
        eprintln!("Nothing given");
        std::process::exit(1);
    }

    let cmd = &env_args[0];
    let parameters = &env_args[1..];
    
    execute_command(&cmd, &parameters);
}

fn execute_command(cmd: &String, parameters: &[String]) {
    if cfg!(target_os = "windows") {
        todo!()
        // Command::new("ls").arg("-a").arg("-l").status().unwrap();
    } else {
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
