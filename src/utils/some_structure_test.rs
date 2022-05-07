
#[cfg(test)]

mod some_structure_test{
    use super::super::{*};
    #[test]
    fn double_test() {
        let sut = SomeStructure { value: 2 };
        assert_eq!(sut.double_value(), 4);
    }

    #[test]
    fn module_private_test() {
        module_private_function();
        assert!(true)
    }
}

