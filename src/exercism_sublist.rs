#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
        use Comparison::*;
        if _first_list.len() == 0 && _second_list.len() == 0 {
            return Equal
        }
        else if _first_list.len() == 0 {
           return Sublist
        }
        else if _first_list.len() > 0  {
            return Superlist
        }
        Unequal
}
