
use rand::Rng;
// mod exercism1;

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        println!("a User with name {}, age {}, and weight {}", name, age, weight)
    }
}


fn main() {
    let mut rng = rand::thread_rng();

    let arr = [50,62,13,43];
    println!("{:?}", arr);

    let vector: Vec<i32> = (3..5).collect();

    println!("{:?}", vector);

    let macro_vector = vec![5,3,6,3,55,2];
    println!("{:?}", macro_vector)
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    println!("{:?}", bob);


}


