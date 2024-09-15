use eframe::{egui, Error as EframeError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Task {
    pub(crate) id: u32,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) completed: bool,
}

impl Task {
    pub(crate) fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TaskManagerApp {
    tasks: Vec<Task>,
    new_task_title: String,
    new_task_description: String,
    filter_completed: Option<bool>,
}

impl Default for TaskManagerApp {
    fn default() -> Self {
        let tasks = load_tasks().unwrap_or_else(|_| Vec::new());
        Self {
            tasks,
            new_task_title: String::new(),
            new_task_description: String::new(),
            filter_completed: None,
        }
    }
}

impl eframe::App for TaskManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Task Manager");

            ui.horizontal(|ui| {
                ui.label("Title: ");
                ui.text_edit_singleline(&mut self.new_task_title);
            });

            ui.horizontal(|ui| {
                ui.label("Description: ");
                ui.text_edit_singleline(&mut self.new_task_description);
            });

            if ui.button("Add Task").clicked() {
                self.add_task(); // Call the add_task method
                save_tasks(&self.tasks).unwrap();
                self.new_task_title.clear();
                self.new_task_description.clear();
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Filter: ");
                if ui.button("All").clicked() {
                    self.filter_completed = None;
                }
                if ui.button("Completed").clicked() {
                    self.filter_completed = Some(true);
                }
                if ui.button("Pending").clicked() {
                    self.filter_completed = Some(false);
                }
            });

            let filtered_tasks = view_tasks(&self.tasks, self.filter_completed);

            for task in filtered_tasks {
                ui.horizontal(|ui| {
                    ui.label(format!("{}: {}", task.id, task.title));
                    if task.completed {
                        ui.label(" (Completed)");
                    }

                    if ui.button("Complete").clicked() {
                        complete_task(&mut self.tasks, task.id);
                        save_tasks(&self.tasks).unwrap();
                    }

                    if ui.button("Delete").clicked() {
                        delete_task(&mut self.tasks, task.id);
                        save_tasks(&self.tasks).unwrap();
                    }
                });
            }
        });
    }
}

impl TaskManagerApp {
    fn add_task(&mut self) {
        // Create a new task and add it to the list
        let id = self.tasks.len() as u32 + 1;
        let task = Task::new(
            id,
            self.new_task_title.clone(),
            self.new_task_description.clone(),
        );
        self.tasks.push(task);

        // Print task details to the console
        println!("Adding task with title: {}", self.new_task_title);
        println!("Description: {}", self.new_task_description);
    }
}

pub fn view_tasks(tasks: &Vec<Task>, completed: Option<bool>) -> Vec<Task> {
    tasks.iter().filter(|&task| {
        if let Some(is_completed) = completed {
            task.completed == is_completed
        } else {
            true
        }
    }).cloned().collect()
}

pub fn complete_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) {
    tasks.retain(|task| task.id != id);
}

pub fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    let data = serde_json::to_string(&tasks)?;
    std::fs::write("tasks.json", data)?;
    Ok(())
}

fn load_tasks() -> std::io::Result<Vec<Task>> {
    let data = std::fs::read_to_string("tasks.json")?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn main() -> Result<(), EframeError> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Task Manager GUI",  // The window title
        native_options,
        Box::new(|_cc| Ok(Box::new(TaskManagerApp::default()))),
    )
}
