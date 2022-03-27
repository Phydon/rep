use std::process::Command;
use std::env;

fn main() {
    let mut env_args = Vec::new();
    // TODO write env_args/command to file -> replace this and then read from file
    let mut cmd_container = Vec::new();

    for arg in env::args().skip(1) {
        if arg == String::from("rep") {
            env_args.push(arg);
        } else {
            let arg_cont = arg.clone();
            cmd_container.push(arg_cont);
            env_args.push(arg);
        }
    }
    
    if env_args.is_empty() {
        eprintln!("Nothing given");
        std::process::exit(1);
    }

    println!("Container: {:?}", cmd_container);

    let cmd = &env_args[1];
    let params = &env_args[2..];
    
    println!("cmd: {:?}", cmd);
    println!("params: {:?}", params);

    execute_command(&cmd, &params);
}

fn execute_command(cmd: &String, parameters: &[String]) {
    if cfg!(target_os = "windows") {
        todo!()
        // Command::new("ls").arg("-a").arg("-l").status().unwrap();
    } else {
        Command::new(cmd).args(parameters).status().unwrap();
    }
}
