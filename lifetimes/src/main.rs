fn main() {
    let string1 = String::from("afdasd");
    let string2 = String::from("dsadasaaa");

    let result = longest( string1.as_str(), string2.as_str());
    println!("{}", result);
     
}

struct ImportExcerpt<'a> {
    part: &'a str
}

fn longest <'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { 
        x
    }
    else{
        y
    }
}
