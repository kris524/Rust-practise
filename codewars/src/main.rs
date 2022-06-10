
fn digitize(n: u64) -> Vec<u8> {
    
    let mut digits: Vec<u8>= Vec::new();
    let mut n = n;
    println!("{}", n);
    while n > 9 {
        let x = n % 10;
        let mut m = x as u8;
        digits.push(m);
        n = n/ 10
    }
    // digits.push(n);
    let n = n as u8;
    digits.push(n);
    digits
 
}

fn main() {
    println!("{:?}", digitize(35231));
    
}
