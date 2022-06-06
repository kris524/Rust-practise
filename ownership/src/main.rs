#[derive(Debug)]
struct DropMe;

impl Drop for DropMe {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    println!("Begin outer scope");

    {
        println!("Begin inner scope");
        let x = DropMe;

        println!("x: {:?}", x);

    }

    println!("End of outer sope")
}