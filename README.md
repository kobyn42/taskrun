# taskrun

`taskrun` is a command-line tool written in Rust for running tasks defined in the `.vscode/tasks.json` file.

## Install

```bash
cargo install --path .
```

## Usage

Execute it with the task name as a command-line argument.

```bash
taskrun <task-name>
```

## tasks.json file

Tasks are defined in the `.vscode/tasks.json` file. Each task is defined as an object with three properties: `type`, `command`, and `label`.

```json
{
    "tasks": [
        {
            "type": "shell",
            "command": "echo Hello",
            "label": "greet"
        }
    ]
}
```

In this example, a task named `greet` is defined, and this task executes the command `echo Hello`.

## Note

- Currently, only tasks with `type` of `shell` are supported.
- If the task name specified as a command-line argument does not exist in `tasks.json`, nothing will be executed.
