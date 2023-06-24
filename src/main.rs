use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use std::thread;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Please specify a task name");
        return;
    }

    // read json file
    let mut file = File::open(".vscode/tasks.json").expect("Unable to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    // parse json
    let v: Value = serde_json::from_str(&remove_comments(&contents)).expect("Unable to parse json");

    // get tasks
    let tasks = v["tasks"].as_array().expect("Unable to get tasks");

    for task in tasks {
        // get task type, command, label
        let task_type = task["type"].as_str().expect("Unable to get task type");
        let task_command = task["command"].as_str().expect("Unable to get command");
        let task_label = task["label"].as_str().expect("Unable to get label");

        // execute command
        if task_type == "shell" && task_label == args[1] {
            let mut child = Command::new("sh")
                .arg("-c")
                .arg(task_command)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute command");

            let child_stdout = child.stdout.take().expect("Failed to open stdout");
            let reader = BufReader::new(child_stdout);

            let handle = thread::spawn(move || {
                for line in reader.lines() {
                    println!("{}", line.unwrap());
                }
            });

            // Wait for the stdout reading thread to finish
            handle.join().unwrap();

            child.wait().expect("Command wasn't running");
            return;
        }
    }
    println!("found not task name \"{}\"", args[1]);
}

fn remove_comments(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut parts = line.splitn(2, "//");
            parts.next().unwrap().trim().to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_comments() {
        let input = r#"
        {
            // task list
            "tasks": [
                {
                    "type": "shell", // task type
                    // execute command
                    "command": "echo Hello"
                }
            ]
        }
        "#;

        let expected = r#"
{

"tasks": [
{
"type": "shell",

"command": "echo Hello"
}
]
}
"#;
        let output = remove_comments(input);
        print!("{}", output);
        assert_eq!(remove_comments(input), expected);
    }
}
