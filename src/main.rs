fn main() {
    let mut todo_list = TodoList::new();

    let task1 = todo_list.add_task("Ders çalış");
    let _task2 = todo_list.add_task("Sinemaya git");

    println!("Projeler bitmeden önce");
    todo_list.list_tasks();

    todo_list.complete_task(task1.id);

    println!("Projeler bittikten sonra");
    todo_list.list_tasks();
}
#[derive(Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: &str) -> Task {
        let id = self.tasks.len() + 1;
        let new_task = Task {
            id,
            description: String::from(description),
            completed: false,
        };
        self.tasks.push(new_task.clone());
        new_task
    }

    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Some(task)
        } else {
            None
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!("ID: {}, Açıklama: {}, Yapıldı mı: {}", task.id, task.description, task.completed);
        }
    }
}