

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub number: i32,
    pub done: bool,
    pub desc: Option<String>
}

//default 
impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Default".into(),
            number: 12,
            done: false,
            desc: None
        }
    }
}

// create the constructor function
impl Task {
    pub fn new(title: impl Into<String> ) -> Task {
        Task {
            title: title.into(),
            number: 1,
            done: false,
            desc: None,
        }
    }
}

fn main() {

    let task = Task::new("XYZ");
    println!( "{task:?}");

    let task2 = Task::default();
    println!("{task2:?}"); 

    let task3: Option<Task> = None;
    let task3 = task3.unwrap_or_default();
    println!("{task3:?}")

}