
fn digitize(n: u64) -> Vec<u8> {
    
    let mut digits: Vec<u8>= Vec::new();
    let mut n = n;
    println!("{}", n);
    loop {
        digits.push((n % 10) as u8);
        n = n/ 10;

        if n < 1 {
            break
        }
    }
    
    digits
 
}
 
 
fn answer_digitize(n: u64) -> Vec<u8>{

    n.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8 ).rev().collect()
    
}


fn set_alarm(employed: bool, vacation: bool) -> bool {
        
        employed && !vacation
}
  
fn main() {
    // println!("{:?}", answer_digitize(234324));
    println!("{:?}", digitize(35231));

    let a = set_alarm(true, true);
    println!("{}", a);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digitize() {
        assert_eq!(digitize(35231), [1, 3, 2, 5, 3]);
    }
}