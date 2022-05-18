
use rand::Rng;
// mod exercism1;
#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        println!("a User with name {}, age {}, and weight {}", name, age, weight)
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }
    pub fn age(&self) -> u32 {
        return self.age;
    }
    pub fn weight(&self) -> f32 {
        return self.weight;
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
        
    }
}


fn main() {
    let mut rng = rand::thread_rng();

    let arr = [50,62,13,43];
    println!("{:?}", arr);

    let vector: Vec<i32> = (3..5).collect();

    println!("{:?}", vector);

    let macro_vector = vec![5,3,6,3,55,2];
    println!("{:?}", macro_vector);
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    println!("{:?}", bob);


}


