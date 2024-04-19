use std::path::PathBuf;

fn main() {
    // Build the resource directory
    let extra_dir = std::fs::read_dir("extra").unwrap();
    let mut files = vec![];
    for file in extra_dir {
        let file = file.unwrap();
        let path = file.path();
        let path = path.to_str().unwrap();
        files.push(path.to_string());
    }
    if files.is_empty() {
        panic!("No files found in extra directory");
    }

    if !files.contains(&"extra/convert.py".to_string()) {
        panic!("convert.py not found in extra directory");
    }

    if !files.contains(&"extra/lila-public-piece.zip".to_string()) {
        panic!("lila public pieces zip not found in extra directory");
    }

    if PathBuf::from("resource").is_dir() {
    } else {
        let resource_dir = std::fs::create_dir("resource");
        let Ok(_) = resource_dir else {
            panic!("Failed to create resources directory");
        };
    }

    // unzip the lila-public-piece.zip
    let mut command = std::process::Command::new("unzip");
    command.arg("extra/lila-public-piece.zip");
    command.arg("-d");
    command.arg("resource");
    command
        .spawn()
        .expect("Failed to unzip lila-public-piece.zip");

    let mut command = std::process::Command::new("python3");
    command.arg("extra/convert.py");

    command.spawn().expect("Failed to run convert.py");
}
