# `celerity-cli`

This is the reference implementation for the 
[`celerity`](https://github.com/joepigott/celerity) CLI client.

## Configuration

`celerity-cli` will look for a configuration file in `<config_dir>/celerity/config.toml`,
where `<config_dir>` is your configuration directory depending on your platform
(e.g. `~/.config` on Linux). An alternate path to a configuration file can be
provided using the `--config` flag.

The configuration file **must** define the following fields:
```toml
[server]
host = "<server hostname/ip>"
port = <server port>

[client]
date_format = "<valid strftime format>"
```

## Usage

For general usage information, run `celerity -h`.

### Adding and Manipulating Tasks

`celerity add` requires four arguments: a title (`-t`), deadline (`-d`), estimated
duration (`-D`), and priority (`-p`). For example, to add an example task that 
should take 1 hour to complete, is due in one week, and is high priority:
```
$ celerity add -t "example task" -d "2/20/2025 12:00 pm" -D 1h -p 0
Item successfully added
```
This task will be assigned a unique ID and added to the queue. `celerity update` 
takes an ID and optionally any of the arguments from `celerity add`; any that are 
defined will be assigned to the task with the corresponding ID. For example, if
`example task` will instead take 3 hours to complete, it can be updated via
```
$ celerity update 1 -D 3h
Item successfully updated
```

### Listing Tasks

The task queue (or completed list if `-c` is supplied) can be listed using
`celerity list`:
```
$ celerity list
1 - example task
        Deadline: 2/20/2025 12:00 PM
        Time Remaining: 3h
        Priority: 0
```
`celerity list` optionally takes arguments that allow filtering of the queue: 
start deadline (`--after`), end deadline (`--before`), shortest duration 
(`--longer`), longest duration (`--shorter`), lowest priority (`--higher`),
and highest priority (`--lower`). All tasks that meet the specified bounds will
be listed.

`celerity active` can be used to list the currently selected task based on the
scheduler priority, regardless of whether the scheduler is enabled or not:
```
$ celerity active
1 - example task
        Deadline: 2/20/2025 12:00 PM
        Time Remaining: 3h
        Priority: 0
```

### Enabling/Disabling the Scheduler

By default, the scheduler is disabled on startup, and will not perform task
selection or update duration. To enable the scheduler, simply use `celerity 
enable`:
```
$ celerity enable
Scheduler successfully enabled
```
At this point, the scheduler will select a task from the queue and begin
removing time from its duration. To stop this, use `celerity disable`:
```
$ celerity disable
Scheduler successfully disabled
```
The status of the scheduler can be fetched using `celerity status`:
```
$ celerity status
disabled
```

### Completing and Deleting Tasks

`celerity complete` takes an ID and marks the corresponding task as complete. It 
will then be removed from the scheduling queue and added to the completed list 
with a new ID, which can be viewed with `celerity list -c`.
```
$ celerity complete 1
Task marked as completed
$ celerity list -c
1 - example task
        Deadline: 2/20/2025 12:00 PM
        Time Remaining: 2h 59m 50s
        Priority: 0
```
Tasks can be deleted from the queue using `celerity delete <ID>`; if `-c` is
supplied, it will instead be deleted from the completed list. For example, if
we add two new tasks:
```
$ celerity add -t "another example task" -d "2/20/2025 1:00 pm" -D 2h -p 2
Item successfully added
$ celerity add -t "another another example task" -d "2/21/2025 2:00 pm" -D 4h -p 5
Item successfully added
$ celerity list
1 - another example task
        Deadline: 2/20/2025 1:00 PM
        Time Remaining: 2h
        Priority: 2
2 - another another example task
        Deadline: 2/21/2025 2:00 PM
        Time Remaining: 4h
        Priority: 5
```
"another example task" can be deleted via
```
$ celerity delete 1
Item successfully deleted
$ celerity list
2 - another another example task
        Deadline: 2/21/2025 2:00 PM
        Time Remaining: 4h
        Priority: 5
```
"example task" (which we marked as completed) can be deleted via
```
$ celerity delete 1 -c
Item successfully deleted
$ celerity list -c
No tasks match the specified bounds
```
Note that **deleted tasks are not recoverable**. Be careful when deleting.

### Getting and Setting Priority

The selection algorithm or "priority" of the scheduler is hot-swappable, and
can be set via `celerity priority set`. For example, to set the priority to 
`ShortestUrgency` (tasks are ranked based on a score, calculated from their
duration and the distance to their deadline),
```
$ celerity priority set shoresturgency
Task queue priority successfully updated
```
The current queue priority can be fetched using
```
$ celerity priority show
Shortest Duration with Urgency
```
The priorities are currenly hardcoded and the system is not fully functional.
This will be fixed in the future.
