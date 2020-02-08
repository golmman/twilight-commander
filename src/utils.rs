use log::info;
use std::fs::File;
use std::io::Read;
use std::panic::set_hook;
use std::process::exit;

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

pub fn setup_logger() -> Result<(), fern::InitError> {
    let log_file_path = format!("{}/{}", get_config_dir()?, "tc.log");

    fern::Dispatch::new()
        .format(|out, message, record| {
            let target = record.target();
            let target_split_at = 0.max(target.len() as i32 - 20) as usize;
            let target_short = target.split_at(target_split_at);

            out.finish(format_args!(
                "[{}][{:05}][{:>20}] {}",
                chrono::Local::now()
                    .to_rfc3339_opts(::chrono::SecondsFormat::Millis, true),
                record.level(),
                target_short.1,
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file(log_file_path)?)
        .apply()?;

    set_hook(Box::new(|panic_info| {
        if let Some(p) = panic_info.payload().downcast_ref::<String>() {
            info!("{:?}, \npayload: {}", panic_info, p,);
        } else if let Some(p) = panic_info.payload().downcast_ref::<&str>() {
            info!("{:?}, \npayload: {}", panic_info, p,);
        } else {
            info!("{:?}", panic_info);
        }
    }));

    info!(
        r#"starting...
                                                 
_|_    o|o _ |__|_  _ _ ._ _ ._ _  _.._  _| _ ._ 
 |_\/\/|||(_|| ||_ (_(_)| | || | |(_|| |(_|(/_|  
           _|                                    
                                                 "#
    );

    info!("logger initialized");

    Ok(())
}

pub fn get_config_dir() -> std::io::Result<String> {
    if let Ok(xdg_config_home) = std::env::var("XDG_CONFIG_HOME") {
        Ok(xdg_config_home)
    } else if let Ok(home) = std::env::var("HOME") {
        Ok(format!("{}/.config/twilight-commander", home))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no HOME or XDG_CONFIG_HOME variable is defined",
        ))
    }
}
