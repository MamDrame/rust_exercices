//pub use crate::OfficeWorker::*;

#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
    pub name: String,
    pub age: u32,
    pub role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(',').collect();
        OfficeWorker {
            name: parts[0].to_string(),
            age: parts[1].parse::<u32>().unwrap(),
            role: WorkerRole::from(parts[2]),
        }
    }
}

impl From<&str> for WorkerRole {
    fn from(value: &str) -> Self {
        match value {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => panic!("Invalid role"),
        }
    }
}
