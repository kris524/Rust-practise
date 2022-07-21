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
    println!("{}", digitize(4));
}

fn digitize(n: u64) {
    let n_str = n.to_string();
    let n_array = n_str.chars().collect();
    return n_array;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
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
