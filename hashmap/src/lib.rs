use std::collections::HashMap;

// use Rust_workshop::part1 as p1;

use std::hash::{Hasher, Hash};



    // try out the traits and generics in rust (chapter 10)

// Build a HashMap from scratch

const INITIAL_BUCKET_COUNT: usize = 16; //Number of slots/buckets
type Slot<Key, Value> = Option<((Key, Value), usize)>;


#[derive(Debug)]
pub struct MyHashMap<Key: Hash + Eq, Value> {
    slots: Vec<Slot<Key, Value>>,
    slot_count: usize,
    item_count: usize,
}



impl<Key, Value> MyHashMap<Key, Value> 
    where Key: Hash + Eq,
            
{

    pub fn new() -> Self {  
        Self { slots: Self::create_slots(INITIAL_BUCKET_COUNT), slot_count: INITIAL_BUCKET_COUNT, item_count: 0}
    }

    fn create_slots(slot_count: usize) -> Vec<Slot<Key, Value>>  {
        let mut new_slots = Vec::with_capacity(slot_count);
        for _ in 0..slot_count {
            new_slots.push(None);
        }
        new_slots
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