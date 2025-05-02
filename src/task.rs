//Written By Noah Robertson
use utc_dt::UTCDatetime;

pub struct Task {
	name: String,
	creation_date: UTCDatetime,
	target_date: Option<UTCDatetime>,
	description: String,
	task_type: Vec<TaskType>,
	completed: bool,
	id: Option<i32>
}

pub enum TaskType {
	Personal,
	Work,
	Hobby,
	Programming,
	Exercise,
	Other,
}

impl Task {
	pub fn new(name: String, creation_date: UTCDatetime,
		       target_date: Option<UTCDatetime>, description: String,
			   task_type: Vec<TaskType>) 
			   -> Task {
		Task {
			name,
			creation_date,
			target_date,
			description,
			task_type,
			completed: false,
			id: None,
		}
	}

	pub fn set_id(&mut self, id: i32) {
		self.id = Some(id);
	}

	pub fn complete(&mut self) {
		self.completed = true;
	}
}