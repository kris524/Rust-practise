

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub number: i32,
    pub done: bool,
    pub desc: Option<String>
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

    

}