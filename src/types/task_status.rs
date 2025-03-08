#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed
}

impl TaskStatus {
    // Convert from database string to enum
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(TaskStatus::Pending),
            "in_progress" => Some(TaskStatus::InProgress),
            "completed" => Some(TaskStatus::Completed),
            _ => None
        }
    }

    // Convert enum back to database string
    pub fn to_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed"
        }
    }

    // Convert enum back to database string
    pub fn as_string(&self) -> Option<String> {
            match self {
                TaskStatus::Pending => Some(String::from("pending")),
                TaskStatus::InProgress => Some(String::from("in_progress")),
                TaskStatus::Completed => Some(String::from("completed"))
            }
    }

    // Optional: Add business logic for status transitions
    pub fn can_transition_to(&self, new_status: &TaskStatus) -> bool {
        match (self, new_status) {
            (TaskStatus::Pending, TaskStatus::InProgress) => true,
            (TaskStatus::InProgress, TaskStatus::Completed) => true,
            _ => false
        }
    }
}

