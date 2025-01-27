# TODOU

## About

A blazingly fast CLI tool helping with to-do lists.

## Installation

### Using cargo

### From releases

## Usage

### Options

- `--project, -p [PROJECT_PATH]`: Search for `TODO.md` in the current Git directory or in the specified project path
- `--filepath, -f NAME`: Path of the to-do list markdown file
- `--help, -h`: Show help

### Commands

- If no command, list the tasks

- `list`: List tasks

  - `--interactive, -i`: TUI to set tasks' status as todo, doing or done (space, q, e)
  - `--show-id`: Display tasks' ID
  - `--show-done`: Display task marked as done

- `add [TASK_NAME]`: Add a task, use TUI if no task name

  - `--status, -s [TODO, DOING, DONE]`: Set the status
  - `--category, -c`: Set the category
  - `--parent, -p`: Set the parent task

- `done`: Mark a task as done using FZF if no ID

  - `--id, -i`: Specify a task ID

- `remove`: Remove a task using FZF if no ID

  - `--id, -i`: Specify a task ID

- `edit`: Update a task metadata using TUI if no options, using FZF if no ID

  - `--id, -i`: Specify a task ID
  - `--status, -s [TODO, DOING, DONE]`: Modify the status
  - `--creation-date, -cd`: Modify the creation date
  - `--done-date, -fd`: Modify the done date
  - `--category, -c`: Modify the category
  - `--parent, -p`: Modify the parent task
