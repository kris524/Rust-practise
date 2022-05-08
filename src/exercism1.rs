use std::str;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}


pub fn sublist_checker( _a: &[u8], _b: &[u8]) -> Comparison{

    let s = match str::from_utf8(_a) {
        Ok(v) => v,
        Err(e) => panic!("Something")
    };
    let s2 = match str::from_utf8(_b) {
        Ok(v) => v,
        Err(e) => panic!("Something")
    };
    println!("{}", s);
    println!("{}", s2);
    if s2.contains(s) {
        println!("Sublist");
        return Comparison::Sublist;
    }
    else {
        println!("idk");
        return Comparison::Unequal;
    }

    
    // if _a == _b {
    //     println!("Equal");
    //     return Comparison::Equal

    // else {
    //     panic!("lol")
    // }
    

}


// Seen as files
// -------------

// main.rs     --> examplary_module.rs   --> a.rs
//                                       --> b.rs
                              
// Seen as modules
// ---------------

// crate_root  --> mod exemplary_module  -->       mod a {}
//                                       --> pub   mod b {}