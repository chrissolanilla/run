use std::process::Command;

fn read_file(contents:&mut String) -> std::io::Result<()> {
    let mut file = std::fs::File::open("run.txt")?;
    // let mut contents = String::new();
    // std::io::Read::read_to_string(&mut file, &mut contents)?;
    std::io::Read::read_to_string(&mut file, contents)?;
    // println!("file contents: {}", contents);
    Ok(())
}

fn main() {
    let mut contents = String::new();
    println!("contents here is: {}", contents);

    if let Err(e) = read_file(&mut contents) {
        eprintln!("error: can't find run.txt file, {e}");
        std::process::exit(1);
    }

    println!("file says : {}", contents);
    let cmd = contents.trim();

    let out = Command::new(cmd)
        .arg("-lha")
        .output()
        .expect("failed to execute command");

    //theoretically we could put in chinese
    match String::from_utf8(out.stdout) {
        Ok(s) => println!("{s}"),
        Err(e) => eprintln!("bad utf8: {e}"),
    }



}

