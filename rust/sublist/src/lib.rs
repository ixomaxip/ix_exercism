use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn superlist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    _second_list.is_empty() 
    || _first_list
            .windows(_second_list.len())
            .any(|sub| sub == _second_list)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match _first_list.len().cmp(&_second_list.len()) {
        Ordering::Greater
            if superlist(_first_list, _second_list) => Comparison::Superlist,
        Ordering::Less
            if superlist(_second_list, _first_list) => Comparison::Sublist,
        Ordering::Equal
            if _first_list == _second_list => Comparison::Equal,
        _ => Comparison::Unequal
    }
}
