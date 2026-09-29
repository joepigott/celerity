use crate::util::ListInfo;
use chrono::Duration;
use reqwest::blocking::Client;
use reqwest::{Url, StatusCode};
use celerity::priority::Priority;
use celerity::{NaiveTask, Task, TaskQueue, UpdateTask};

/// Sends a `GET` request to fetch the task queue, and lists the tasks.
/// `ListInfo` contains arguments defined by the user for filtering. The
/// `completed` flag switches the array of tasks from those in the active queue
/// to those in the completed list. The other arguments define bounds on which
/// to filter the tasks.
pub fn list(
    host: String,
    port: Option<u16>,
    info: ListInfo,
    date_format: String,
) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .map_err(|e| e.to_string())?;

    match response.status() {
        StatusCode::OK => {
            let text = response.text().map_err(|e| e.to_string())?;
            let queue: TaskQueue = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            let mut tasks: Vec<Task> = if info.completed {
                queue.iter_completed().map(|t| t.to_owned()).collect()
            } else {
                queue.iter().map(|t| t.to_owned()).collect()
            };

            // do some filtering based on cli options

            if let Some(before) = info.before {
                tasks.retain(|t| t.deadline < before);
            }
            if let Some(after) = info.after {
                tasks.retain(|t| t.deadline > after);
            }
            if let Some(shorter) = info.shorter {
                tasks.retain(|t| t.duration < shorter);
            }
            if let Some(longer) = info.longer {
                tasks.retain(|t| t.duration > longer);
            }
            if let Some(lower) = info.lower {
                tasks.retain(|t| t.priority < lower);
            }
            if let Some(higher) = info.higher {
                tasks.retain(|t| t.priority > higher);
            }

            // collect retained tasks into a string representation

            if tasks.is_empty() {
                Ok("No tasks match the specified bounds".to_string())
            } else {
                Ok(tasks.iter().map(|t| t.display(&date_format)).collect())
            }
        },
        StatusCode::NOT_FOUND => {
            Ok("There are no tasks in the queue".to_string())
        }
        _ => {
            Err("Unable to retrieve tasks".to_string())
        }
    }
}

/// Sends a `POST` request with a `NaiveTask` as the body. The server will give
/// this task an ID and add it to the queue.
pub fn add(host: String, port: Option<u16>, task: NaiveTask) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let client = Client::new();

    client
        .post(url)
        .body(serde_json::to_string(&task).map_err(|e| e.to_string())?)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

/// Sends a `PUT` request with an `UpdateTask` as the body. The server will
/// parse the `Some` fields and update the corresponding task with the new
/// information.
pub fn update(host: String, port: Option<u16>, task: UpdateTask) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks");

    let client = Client::new();

    client
        .put(url)
        .body(serde_json::to_string(&task).map_err(|e| e.to_string())?)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

/// Sends a `DELETE` request with an ID as the body. The server will delete the
/// corresponding task---it will be removed entirely and **not** be added to
/// the completed queue.
pub fn delete(host: String, port: Option<u16>, ids: Vec<usize>, completed: bool) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path(if completed {
        "api/tasks/complete"
    } else {
        "api/tasks"
    });

    let client = Client::new();

    let mut result = String::new();
    for id in ids {
        result = client
            .delete(format!("{url}/{id}"))
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;
    }

    Ok(result)
}

/// Sends a `PUT` request with an ID as the body. The server will remove the
/// task from the queue and add it to the completed list.
pub fn complete(host: String, port: Option<u16>, ids: Vec<usize>) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks/complete");

    let client = Client::new();

    let mut result = String::new();
    for id in ids {
        result = client
            .put(format!("{url}/{id}"))
            .body(serde_json::to_string(&id).map_err(|e| e.to_string())?)
            .send()
            .map_err(|e| e.to_string())?
            .text()
            .map_err(|e| e.to_string())?;
    }

    Ok(result)
}

/// Sends a `POST` request to enable or disable the scheduler, depending on the
/// value of `enable`.
pub fn enable(host: String, port: Option<u16>, enable: bool) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path(if enable {
        "api/tasks/enable"
    } else {
        "api/tasks/disable"
    });

    let client = Client::new();

    client
        .post(url)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

/// Sends a `GET` request, which will return the active task according to the
/// current scheduler priority. The task will be returned whether the scheduler
/// is enabled or not.
pub fn active(host: String, port: Option<u16>, date_format: String) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks/active");

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .map_err(|e| e.to_string())?;

    match response.status() {
        StatusCode::OK => {
            let text = response.text().map_err(|e| e.to_string())?;
            let task: Task = serde_json::from_str(&text).map_err(|e| e.to_string())?;
            Ok(task.display(&date_format))
        },
        StatusCode::NOT_FOUND => {
            Ok("There are no tasks in the queue".to_string())
        }
        _ => {
            Err("Unable to retrieve active task".to_string())
        }
    }
}

/// Sends a `GET` request, which will return the status of the scheduler
/// (`enabled` or `disabled`).
pub fn status(host: String, port: Option<u16>) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks/status");

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?
        .parse::<bool>()
        .map_err(|e| e.to_string())?;

    Ok(if response {
        "enabled".to_string()
    } else {
        "disabled".to_string()
    })
}

/// Sends a `PUT` request with a `Priority` trait object as the body. The
/// server will apply the priority to the scheduler.
pub fn set_priority(
    host: String,
    port: Option<u16>,
    priority: Box<dyn Priority>,
) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks/priority");

    let client = Client::new();

    client
        .put(url)
        .body(serde_json::to_string(&priority).map_err(|e| e.to_string())?)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())
}

/// Sends a `GET` request, which returns a string representation of the current
/// scheduler priority.
pub fn get_priority(host: String, port: Option<u16>) -> Result<String, String> {
    let mut url = convert_url(host, port)?;
    url.set_path("api/tasks/priority");

    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;
    let priority: Box<dyn Priority> = serde_json::from_str(&response).map_err(|e| e.to_string())?;

    Ok(priority.string())
}

/// Converts a user-defined `host` and `port` into a URL usable by `reqwest`.
/// `reqwest` URL building is not very good, so a base URL of
/// `http://example.com` is defined and the user options are applied to it.
fn convert_url(host: String, port: Option<u16>) -> Result<Url, String> {
    let mut url = Url::parse("https://example.com").map_err(|e| e.to_string())?;
    url.set_host(Some(&host))
        .map_err(|_| "Unable to set URL host")?;
    if let Some(port) = port {
        url.set_port(Some(port))
            .map_err(|_| "Unable to set URL port")?;
    }
    url.set_scheme("https")
        .map_err(|_| "Unable to set URL scheme")?;

    Ok(url)
}

trait DisplayTask {
    fn display(&self, date_format: &str) -> String;
}

impl DisplayTask for Task {
    fn display(&self, date_format: &str) -> String {
        format!(
            "{} - {}\n\tDeadline: {}\n\tTime Remaining: {}\n\tPriority: {}\n",
            self.id(),
            self.title,
            self.deadline.format(date_format),
            self.duration.display(),
            self.priority,
        )
    }
}

trait DisplayDuration {
    fn display(&self) -> String;
}

impl DisplayDuration for Duration {
    fn display(&self) -> String {
        let hours = self.num_hours();
        let minutes = self.num_minutes() % 60;
        let seconds = self.num_seconds() % 60;

        let mut result = String::new();

        if hours != 0 {
            result.push_str(&format!("{hours}h "));
        }
        if minutes != 0 {
            result.push_str(&format!("{minutes}m "));
        }
        if seconds != 0 {
            result.push_str(&format!("{seconds}s"));
        }

        result
    }
}
