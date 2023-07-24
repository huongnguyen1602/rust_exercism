// the first way
// use crate::Comparison::{Equal, Sublist, Superlist, Unequal};
// use std::cmp::Ordering;
// use std::fmt::Debug;
// use std::mem::swap;
// #[derive(Debug, PartialEq, Eq)]
// pub enum Comparison {
//     Equal,
//     Sublist,
//     Superlist,
//     Unequal,
// }
// pub fn is_sublist<T: PartialEq + Clone + Debug>(
//     mut _first_list: Vec<T>,
//     mut _second_list: Vec<T>,
// ) -> bool {
//     if _first_list.len() > _second_list.len() {
//         swap(&mut _first_list, &mut _second_list);
//     }
//     let mut first_list = _first_list
//         .into_iter()
//         .map(|x| Some(x))
//         .collect::<Vec<Option<T>>>();
//     let mut second_list = _second_list
//         .into_iter()
//         .map(|x| Some(x))
//         .collect::<Vec<Option<T>>>();
//     // First list len before
//     let k = first_list.len();
//     first_list.push(None);
//     first_list.append(&mut second_list);
//     let n = first_list.len();
//     let mut pr = vec![0; n];
//     for (i, elem) in first_list.clone().into_iter().enumerate().skip(1) {
//         let mut j = pr[i - 1];
//         while j > 0 && first_list[j] != elem {
//             j = pr[j - 1];
//         }
//         if first_list[j] == elem {
//             pr[i] = j + 1;
//         }
//     }
//     pr.clone().into_iter().for_each(|x| println!("{}", x));
//     pr.into_iter().find(|x| x == &k).is_some()
// }
// pub fn sublist<T: PartialEq + Clone + Debug>(_first_list: &[T], _second_list: &[T]) -> Comparison {
//     let len_1 = _first_list.len();
//     let len_2 = _second_list.len();
//     match (
//         len_1.cmp(&len_2),
//         is_sublist(
//             Vec::from(_first_list.clone()),
//             Vec::from(_second_list.clone()),
//         ),
//     ) {
//         (Ordering::Equal, true) => Equal,
//         (Ordering::Less, true) => Sublist,
//         (Ordering::Greater, true) => Superlist,
//         (_, _) => Unequal,
//     }
// }



// the second way
// #[derive(Debug, PartialEq, Eq)]
// pub enum Comparison{
//     Equal,
//     Sublist,
//     Superlist,
//     Unequal,
// }

// pub fn sublist<T: PartialEq> (_first_list: &[T], _second_list: &[T]) -> Comparison{
//     let (l1, l2) = (_first_list.len(), _second_list.len());
//     if l1 == l2 && _first_list == _second_list {return Comparison::Equal;}
//     if l1 == 0 {return Comparison::Sublist;}
//     if l2 == 0 {return Comparison::Superlist;}
//     if l1<l2 {
//         for (i, n) in _second_list.iter().enumerate() {
//             if *n == _first_list[0] && i+l1 <= l2 && _first_list == &_second_list[i..i+l1] {
//                 return Comparison::Sublist;
//             }
//         }
//     }

//     if l2 < l1 {
//         for (i,n) in _first_list.iter().enumerate(){
//             if *n == _second_list[0] && i+l2<l1 && _second_list == &_first_list[i..i+l2] {
//                 return Comparison::Superlist;
//             }
//         }
//     }
//     Comparison::Unequal
// }


// the third way
// #[derive(Debug, PartialEq, Eq)]
// pub enum Comparison{
//     Equal,
//     Sublist,
//     Superlist,
//     Unequal,
// }

// fn contain_sublist<T: PartialEq> (large: &[T], small: &[T]) -> bool {
//     if small.is_empty() {
//         return true;
//     }
//     large.windows(small.len()).any(|windows| windows == small)
// }

// pub fn sublist<T:PartialEq> (first_list: &[T], second_list: &[T]) -> Comparison{
//     if first_list.len() > second_list.len(){
//         if contain_sublist(first_list, second_list){
//             Comparison::Superlist
//         }else {
//             Comparison::Unequal
//         }
//     } else if first_list.len() < second_list.len() {
//         if contain_sublist(second_list, first_list){
//             Comparison::Sublist
//         }else{
//             Comparison::Unequal
//         }
//     } else if first_list == second_list {
//         Comparison::Equal
//     } else {
//         Comparison::Unequal
//     }
// }


//the fourth way
// #[derive(Debug, PartialEq, Eq)]
// pub enum Comparison{
//     Equal,
//     Sublist,
//     Superlist,
//     Unequal,
// }

// pub fn sublist<T: PartialEq> (a: &[T], b: &[T]) -> Comparison {
//     use Comparison::*;
//     match (a.len(),b.len()) {
//         (0,0) => Equal,
//         (0,_) => Sublist,
//         (_,0) => Superlist,
//         (m,n) if m<n => if b.windows(m).any(|v| v==a) { Sublist } else {Unequal},
//         (m,n) if m>n => if a.windows(n).any(|v| v==b) {Superlist} else {Unequal},
//         (_,_) => if a==b {Equal} else {Unequal}
//     }
// }






//the fifth way 
use std::fmt::Debug;
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison{
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_helper<T: PartialEq + Debug> (pattern: &[T], text: &[T]) -> bool {
    if pattern.is_empty() {return true}
    return text.windows(pattern.len()).any(|win| win == pattern);
} 


pub fn sublist<T: PartialEq + Debug> (first_list: &[T], second_list: &[T]) -> Comparison {
    let a_in_b = sublist_helper(first_list, second_list);
    let b_in_a = sublist_helper(second_list, first_list);
    match (a_in_b, b_in_a) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
        (false, true) => Comparison::Superlist,
    }
}





pub fn test() {
    fn test_compare_larger_equal_lists() {
        use std::iter::repeat;
        let v: Vec<char> = repeat('x').take(1000).collect();
        assert_eq!(Comparison::Equal, sublist(&v, &v));
    }

    fn test_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 2, 3], &[1, 2, 3, 4, 5]));
    }

    fn sublist_in_middle() {
        assert_eq!(Comparison::Sublist, sublist(&[4, 3, 2], &[5, 4, 3, 2, 1]));
    }

    fn sublist_at_end() {
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &[1, 2, 3, 4, 5]));
    }

    fn partially_matching_sublist_at_start() {
        assert_eq!(Comparison::Sublist, sublist(&[1, 1, 2], &[1, 1, 1, 2]));
    }

    fn sublist_early_in_huge_list() {
        let huge: Vec<u32> = (1..1_000_000).collect();
        assert_eq!(Comparison::Sublist, sublist(&[3, 4, 5], &huge));
    }

    fn huge_sublist_not_in_huge_list() {
        let v1: Vec<u64> = (10..1_000_001).collect();
        let v2: Vec<u64> = (1..1_000_000).collect();
        assert_eq!(Comparison::Unequal, sublist(&v1, &v2));
    }

    fn superlist_at_start() {
        assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
    }
    
    fn superlist_in_middle() {
        assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
    }

    fn second_list_missing_element_from_first_list() {
        assert_eq!(Comparison::Unequal, sublist(&[1, 2, 3], &[1, 3]));
    }

    fn recurring_values_sublist() {
        assert_eq!(
            Comparison::Sublist,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 1, 2, 3, 2, 1])
        );
    }
    
    fn recurring_values_unequal() {
        assert_eq!(
            Comparison::Unequal,
            sublist(&[1, 2, 1, 2, 3], &[1, 2, 3, 1, 2, 3, 2, 3, 2, 1])
        );
    }
    println!("it's ok")
}