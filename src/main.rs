
use rand::Rng;
// mod exercism1;
mod exercism_clock_exercise;


fn main() {
    let mut rng = rand::thread_rng();

    let arr = [50,62,13,43];
    println!("{:?}", arr);

    let vector: Vec<i32> = (3..5).collect();

    println!("{:?}", vector);

    let macro_vector = vec![5,3,6,3,55,2];
    println!("{:?}", macro_vector);

    


}


