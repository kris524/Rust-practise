fn main() {
    let string1 = String::from("afdasd");
    let string2 = String::from("dsadasaaa");

    let result = longest( string1.as_str(), string2.as_str());
    println!("{}", result);

    let novel = String::from("Call me adsad. Some years ago...");
    let first_sen = novel.split(".").next().expect("Couldnt figure this out");
    println!("{}", first_sen);
     
}

struct ImportExcerpt<'a> {
    part: &'a str
}
impl <'a> ImportExcerpt<'a> {
    fn return_part(&self, announcememts: &str) -> &str {
        println!("Attnetion please {}", announcememts);
        self.part
    }
}

fn longest <'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { 
        x
    }
    else{
        y
    }
}
