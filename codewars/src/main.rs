
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

fn lovefunc(flower1: u16, flower2: u16) -> bool {
    
    (flower1 + flower2) % 2 != 0

}


fn how_much_i_love_you(mut nb_petals: u16) -> &'static str {
    let vec1 = vec!["I love you", "a little", "a lot", "passionately", "madly", "not at all"];
    // let mut iter = vec1.iter();
    while  nb_petals > 6 {
        let nb_petals = nb_petals - 5;
    }
    vec1[(nb_petals-1) as usize]
}

fn boolean_to_string(b: bool) -> String {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(boolean_to_string(true), "true", "When we pass in true, we want the string \"true\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
    }
}




  
fn main() {
   
    // println!("{:?}", answer_digitize(234324));
    println!("{:?}", digitize(35231));

    let a = set_alarm(true, true);
    println!("{}", a);
    
}
