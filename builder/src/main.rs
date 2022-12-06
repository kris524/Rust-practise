

#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub number: i32,
    pub done: bool,
    pub desc: Option<String>
}

fn main() -> Result<()> {

    let task = Task {
        title: "Title".to_string(),
        number: 12,
        done: false,
        desc: None,
    };
    println!( "{task:?}");

    Ok(())

}