use std::fs::File;
use std::io::Read;
use std::process::exit;

pub fn read_file(file_name: &str) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn print_help() {
    println!(
        r#"twilight-commander 0.5.0
usage: twilight-commander [--key1=value1 --key2=value2 ...]"#
    );
    exit(0);
}
