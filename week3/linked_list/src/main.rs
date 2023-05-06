use linked_list::LinkedList;

use crate::linked_list::ComputeNorm;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    for val in &list {
       println!("{}", val);
    }

    let mut float_list: LinkedList<f64> = LinkedList::new();
    for i in 1..6 {
        float_list.push_front(i as f64);
    }
    println!("norm: {:.4}", float_list.compute_norm());
}
