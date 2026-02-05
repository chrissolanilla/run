use std::process::Command;
use std::collections::HashMap;
use std::env;

fn read_file(contents:&mut String) -> std::io::Result<()> {
    let mut file = std::fs::File::open("run.txt")?;
    // let mut contents = String::new();
    // std::io::Read::read_to_string(&mut file, &mut contents)?;
    std::io::Read::read_to_string(&mut file, contents)?;
    // println!("file contents: {}", contents);
    Ok(())
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        args.push("default".to_string());
    }
    if args.len() != 2 {
        println!("bad args length, only pass in one argument max, if you have a default then no args needed to run that");
        std::process::exit(1);
    }
    // println!("args are: ");

    // for arg in &args {
    //     println!("{}", arg);
    // }

    let mut contents = String::new();
    // println!("contents here is: {}", contents);

    if let Err(e) = read_file(&mut contents) {
        eprintln!("error: can't find run.txt file, {e}");
        std::process::exit(1);
    }

    // println!("file says : {}", contents);
    let lines: Vec<&str> = contents.lines().collect();
    let mut commands: HashMap<String, String> = HashMap::new();
    for line in lines {
        if line.is_empty() || line.starts_with("//") {
            println!("skipping line: {}", line);
            continue;
        }

        let (key, value) = match line.split_once(':') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => {
                println!("bad line: {}", line);
                continue;
            }
        };

        commands.insert(key.to_string(), value.to_string());
    }

    if &args[1] == "list" {
        println!("list of commands:");
        for (k, v) in commands {
            println!("{k}, : {v}");
        }
        std::process::exit(0);
    }

    let cmd = match commands.get(&args[1]) {
        Some(v) => v.trim(),
        None => {
            eprintln!("unkown key in list, look for availible commands with \"run list\" ");
            std::process::exit(1);
        }
    };

    let out = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        // .output()
        .status()
        .expect("failed to execute command");

    println!("exit: {out}");

    std::process::exit(0);
}

