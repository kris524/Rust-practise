
#[cfg(test)]
#[path = "some_structure_tests.rs"]
mod some_structure_tests;

pub struct SomeStructure {
    value: i32
}

impl SomeStructure {
    fn double_value(&self) -> i32 {
        self.value * 2
    }
}

fn module_private_function() {

}