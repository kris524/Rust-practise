mod Rust_workshop;


use std::collections::HashMap;

// use Rust_workshop::part1 as p1;

fn main() {
    type Slot<K, V> = Option< ((K, V), usize)>;
    struct MyHashMap<T> {
        key: T,
        value: T,
    }

    impl MyHashMap {

        fn new(self ,key: T, value: T) {
            MyHashMap {key, value};
        }

    }

}



#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn it_works(){
        // create a new HashMap
        let mut map = HashMap::new();

        // insert key/value pairs into the HashMap
        assert_eq!(map.insert("foo", "bar"), None);
        // if an item already exists for that value, it should return the old value
        // assert_eq!(map.insert("foo", "lol"), Some("bar"));

        // // get a value based on its key
        // assert_eq!(map.get(&"foo"), Some(&"lol"));
        // // you should be able to do this multiple times
        // assert_eq!(map.get(&"foo"), Some(&"lol"));
        // // if no value exists for a key, return None
        // assert_eq!(map.get(&"qux"), None);

        // // remove a value for a key
        // assert_eq!(map.remove(&"foo"), Some("lol"));
        // // once it no longer exists in the map, it should return None
        // assert_eq!(map.get(&"foo"), None);
    }

}