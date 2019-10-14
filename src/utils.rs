use std::{
    fs::File,
    io::Read,
    process::exit,
};

pub fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn print_help() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!(r#"usage: twilight-commander [--key1=value1 --key2=value2 ...]"#);
    exit(0);
}
