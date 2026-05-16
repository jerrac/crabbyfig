use std::process::exit;

fn main() {
    if let Err(e) = run() {
        log_message(&format!("Error: {}", e), "ERROR");
        exit(1);
    }
}

fn run() -> Result<(), String> {
    let prefix = std::env::var("CRABBYFIX")
        .map_err(|_| "CRABBYFIX environment variable must be set".to_string())?;
    let targets = load_targets();
    if !targets.is_ok() {
        return Err("Failed to load targets".to_string());
    }
    for (key, value) in std::env::vars() {
        if key.starts_with(prefix.as_str()) && value.len() > 0 {
            let processed = process_var(key.as_str(), value.as_str());
            let (use_key, use_value) = match processed {
                Ok((use_key, use_value)) => (use_key, use_value),
                Err(e) => return Err(e),
            };
            for item in targets.clone().unwrap().iter() {
                process_file(item.clone(), use_key.clone(), use_value.clone())
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}
fn targets_from_file(file_path: &str) -> Result<String, String> {
    // Parse the contents of the file into the same format as if
    // the user was using a csv string in the normal environment variable.
    let mut file_contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    if file_contents.contains("\r\n") {
        file_contents = file_contents.replace("\r\n", ",");
    }
    if file_contents.contains("\n") {
        file_contents = file_contents.replace("\n", ",");
    }
    if file_contents.contains("\r") {
        file_contents = file_contents.replace("\r", ",");
    }
    Ok(file_contents)
}

fn load_targets() -> Result<Vec<String>, String> {
    let targets_in_file = std::env::var("CRABBYGETS_FILE");
    let mut targets_string = String::new();
    if targets_in_file.is_ok() {
        let targets_file_path = targets_in_file.unwrap();
        let load_targets_string = targets_from_file(targets_file_path.clone().as_str());
        if load_targets_string.is_err() {
            return Err(format!(
                "Failed to read targets from file: {}",
                load_targets_string.err().unwrap()
            ));
        }
        targets_string = load_targets_string.unwrap();
    }
    let load_targets_in_env = std::env::var("CRABBYGETS");
    if load_targets_in_env.is_ok() {
        targets_string = format!(
            "{},{}",
            targets_string,
            load_targets_in_env.unwrap().as_str()
        );
    }
    let targets: Vec<String> = targets_string.split(',').map(|s| s.to_string()).collect();
    Ok(targets)
}

fn process_file(
    file_path_string: String,
    use_key: String,
    use_value: String,
) -> Result<(), String> {
    if !file_path_string.is_empty() {
        let file_path = std::path::Path::new(file_path_string.as_str());
        if !file_path.exists() {
            return Err(format!(
                "Target file does not exist: {}",
                file_path.display()
            ));
        } else {
            let content = std::fs::read_to_string(file_path).expect("Failed to read file");
            let replaced = content.replace(use_key.as_str(), use_value.as_str());
            std::fs::write(file_path, replaced).expect("Failed to write file");
            log_message(
                &format!("Replaced {} in {}", use_key, file_path.display()),
                "INFO",
            );
            Ok(())
        }
    } else {
        // Just ignore empty file paths.
        Ok(())
    }
}
#[test]
fn test_process_file() {
    let file_path = std::path::Path::new("test.txt");
    let content = "Hello, REPLACE_QWERTY_WORLD!";
    std::fs::write(file_path, content).expect("Failed to write file");
    let replace_key = "REPLACE_QWERTY_WORLD".to_string();
    let replace_value = "Bob".to_string();
    let replaced = process_file("test.txt".to_string(), replace_key, replace_value)
        .expect("Failed to process file");
    assert_eq!(replaced, ());
    let file_contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    assert_eq!(file_contents, "Hello, Bob!");
    log_message(
        &format!("File contents after replacement: {}", file_contents),
        "DEBUG",
    );
    std::fs::remove_file(file_path).expect("Failed to remove file");
}

fn process_var(env_key: &str, env_value: &str) -> Result<(String, String), String> {
    let mut use_key = env_key.to_string();
    let mut use_value = env_value.to_string();
    if env_key.ends_with("_FILE") {
        use_key = env_key.replace("_FILE", "");
        let file_path = std::path::Path::new(env_value);
        if !file_path.exists() {
            let wait_for_key = "CRABBYWAIT";
            let wait_for = std::env::var(wait_for_key).unwrap_or_else(|_| "0".to_string());
            let wait_for_count_key = "CRABBYWAITCOUNT";
            let wait_for_count =
                std::env::var(wait_for_count_key).unwrap_or_else(|_| "100".to_string());
            if is_numeric(wait_for.as_str())
                && wait_for != "0"
                && is_numeric(wait_for_count.as_str())
            {
                let interval = std::time::Duration::from_secs(wait_for.parse::<u64>().unwrap());
                let loop_count = wait_for_count.parse::<u64>().unwrap();
                for _ in 0..loop_count {
                    log_message(
                        &format!(
                            "File not found: {}. Waiting for it to be populated. Interval {}, loops: {}",
                            file_path.display(),
                            interval.as_secs(),
                            loop_count
                        ),
                        "INFO",
                    );
                    std::thread::sleep(interval);
                    if file_path.exists() {
                        log_message(
                            &format!(
                                "File found: {}. Waited {} loops for it to be populated.",
                                file_path.display(),
                                loop_count
                            ),
                            "INFO",
                        );
                        break;
                    }
                }
            } else {
                log_message(&format!("File not found: {}", file_path.display()), "ERROR");
                return Err("Error: File not found".to_string());
            }
        }
        if !file_path.exists() {
            log_message(&format!("File not found: {}", file_path.display()), "ERROR");
            return Err("Error: File not found".to_string());
        }
        log_message(&format!("File found: {}", file_path.display()), "DEBUG");
        let file_contents = std::fs::read_to_string(file_path).expect("Failed to read file");
        use_value = file_contents;
    }
    let replace_key = format!("REPLACE_{}", use_key);
    Ok((replace_key, use_value))
}

#[test]
fn test_process_var_file_valid() {
    // Test a _FILE var.
    let box_file_contents = "Alice";
    let file_path = std::path::Path::new("box.txt");
    let content = "Alice";
    std::fs::write(file_path, content)
        .expect("Failed to write file for testing process_var on _FILE vars.");
    let box_env_key = "QWERTY_BOX_FILE";
    let box_env_value = std::env::var(box_env_key);
    let box_processed = process_var(box_env_key, box_env_value.unwrap().as_str());
    let (box_replace_key, box_replace_value) = box_processed.unwrap();
    assert_eq!(box_replace_key, "REPLACE_QWERTY_BOX");
    assert_eq!(box_replace_value, box_file_contents);
    std::fs::remove_file(file_path)
        .expect("Failed to remove file for testing process_var on _FILE vars.");
}
#[test]
fn test_process_var_file_invalid() {
    // Test a missing file var
    let missing_file_env_key = "QWERTY_MISSING_FILE";
    let missing_file_env_value = std::env::var(missing_file_env_key);
    let processed_missing = process_var(
        missing_file_env_key,
        missing_file_env_value.unwrap().as_str(),
    );
    assert!(processed_missing.is_err());
}

fn log_message(message: &str, level: &str) {
    let date_string = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let parsed = format!("{} [CRABBYFIG][{}] {}", date_string, level, message);
    println!("{}", parsed);
}
#[test]
fn test_log_message() {
    log_message("Test message", "INFO");
}
fn is_numeric(s: &str) -> bool {
    s.trim().parse::<u64>().is_ok()
}
#[test]
fn test_is_numeric() {
    assert!(is_numeric("123"));
    assert!(!is_numeric("abc"));
    assert!(!is_numeric("123.45"));
}
