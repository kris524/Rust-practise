// fn digitize(n: u64) -> Vec<u8> {

//     let mut digits: Vec<u8>= Vec::new();
//     let mut n = n;
//     println!("{}", n);
//     loop {
//         digits.push((n % 10) as u8);
//         n = n/ 10;

//         if n < 1 {
//             break
//         }
//     }

//     digits

// }

fn answer_digitize(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect()
}

fn set_alarm(employed: bool, vacation: bool) -> bool {
    employed && !vacation
}

fn lovefunc(flower1: u16, flower2: u16) -> bool {
    (flower1 + flower2) % 2 != 0
}

fn how_much_i_love_you(mut nb_petals: u16) -> &'static str {
    let vec1 = vec![
        "I love you",
        "a little",
        "a lot",
        "passionately",
        "madly",
        "not at all",
    ];
    // let mut iter = vec1.iter();
    while nb_petals > 6 {
        let nb_petals = nb_petals - 5;
    }
    vec1[(nb_petals - 1) as usize]
}

fn boolean_to_string(b: bool) -> String {
    match b {
        true => String::from("true"),
        false => String::from("false"),
    }
}

fn dna_to_rna(dna: &str) -> String {
    // let mut my_chars: Vec<_> = dna.chars().collect();
    // dna.chars().map( |c| match c {'T' => 'U', k=>k}).collect()

    dna.replace("U", "T")
}

fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}
fn bmi(weight: u32, height: f32) -> &'static str {
    let h2 = height * height;
    let bmi = (weight as f32) / h2;
    match bmi {
        0.0..=18.5 => "Underweight",
        18.6..=25.0 => "Normal",
        25.1..=30.0 => "Overweight",
        _ => "Obese",
    }
}
fn solution(word: &str, ending: &str) -> bool {
    let a = ending.len();
    let b = word.len();
    if (a > b) {
        return false;
    }
    let section = &word[(b - a)..];
    println!("{} {}", section, a);
    if section == ending {
        true
    } else {
        false
    }
}
fn positive_sum(slice: &[i32]) -> i32 {
    //better solution
    slice.iter().filter(|&x| x.is_positive()).sum()

    // your code
    // let mut positive_nums = vec![];

    // for &num in slice {
    //     if num > 0 {
    //         positive_nums.push(num);
    //     }
    //     else {
    //         continue;
    //     }
    // }
    // return positive_nums.iter().sum()
}

fn main() {
    // println!("{:?}", answer_digitize(234324));
    println!("{:?}", dna_to_rna("TREE"));
    println!("{:?}", solution("abc", "c"));
    println!("{:?}", digitize(443243));
}

fn digitize(n: u64) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .rev()
        .collect()
    // let n_str = n.to_string();
    // let n_array = n_str.chars().map(|c| c.to_digit(10).unwrap()).rev().collect()

    //.to_digit(10).unwrap()
    // let b: Vec<u8> = n_array.into_iter().map(|x| x as u8).collect();
}

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x.pow(2)).sum()
}

fn validate_pin(pin: &str) -> bool {
    if pin.len() == 4 || pin.len() == 6 {
        let a = pin.parse::<u32>();
        match a {
            Ok(u32) => return true,
            Err(_) => return false,
        }
        println!("{:?}", a);
        return true;
    } else {
        false
    }
}



fn open_or_senior(data: Vec<(i32, i32)>)-> Vec<String>  {

    let mut output: Vec<String> = Vec::new();
    

    for batch in data.iter() {
        println!("{:?}", batch.0);
        if batch.0 >= 55 && batch.1 > 7 {
            output.push("Senior".to_string())
        }
        else {
            output.push("Open".to_string())
        }
    }
    return output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(open_or_senior(vec![(45, 12), (55,21), (19, -2), (104, 20)]), vec!["Open", "Senior", "Open", "Senior"]);
        assert_eq!(open_or_senior(vec![(3, 12), (55,1), (91, -2), (54, 23)]), vec!["Open", "Open", "Open", "Open"]);
    }

    #[test]
    fn invalid_length_tests() {
        assert_eq!(validate_pin("1"), false);
        assert_eq!(validate_pin("12"), false);
        assert_eq!(validate_pin("123"), false);
        assert_eq!(validate_pin("12345"), false);
        assert_eq!(validate_pin("1234567"), false);
        assert_eq!(validate_pin("-1234"), false);
        assert_eq!(validate_pin("1.234"), false);
        assert_eq!(validate_pin("-1.234"), false);
        assert_eq!(validate_pin("00000000"), false);
    }
    #[test]
    fn non_digit_chars_tests() {
        assert_eq!(validate_pin("a234"), false);
        assert_eq!(validate_pin(".234"), false);
    }

    #[test]
    fn valid_pin_tests() {
        assert_eq!(validate_pin("1234"), true);
        assert_eq!(validate_pin("0000"), true);
        assert_eq!(validate_pin("1111"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("098765"), true);
        assert_eq!(validate_pin("000000"), true);
        assert_eq!(validate_pin("123456"), true);
        assert_eq!(validate_pin("090909"), true);
    }

    #[test]
    fn example_stuff() {
        assert_eq!(
            boolean_to_string(true),
            "true",
            "When we pass in true, we want the string \"true\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
    }

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
    }

    #[test]
    fn basic_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }
}
