// #![feature(generic_const_exprs)]

use core::slice;
use std::{collections::BinaryHeap, mem::size_of};
#[macro_use]

mod error;
pub mod graph_binary;
mod macros;
mod specs;
mod structure;

#[cfg(feature = "graph_son")]
pub mod graphson;

#[cfg(feature = "serde")]
pub mod de;
#[cfg(feature = "serde")]
pub mod ser;

pub use structure::Binding;
#[cfg(test)]
mod tests {}

// Example usage
// fn main() {
// ClientBuilder::new("ws://localhost:8182/gremlin")
//     .unwrap()
//     .connect_insecure()
//     .unwrap();
// }

fn quick_sort<T: Ord + Clone>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => (),
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1)
            }
        }
        _ => {
            let (pivot, rest) = slice.split_first_mut().unwrap();

            let mut left = 0;
            let mut right = rest.len() as isize - 1;
            while left <= right {
                if rest[left as usize] <= *pivot {
                    left += 1;
                } else {
                    rest.swap(left as usize, right as usize);
                    right -= 1;
                }
            }
            left += 1;
            right += 1;
            slice.swap(0, left as usize - 1);
            // let (left, right) = slice.split_at_mut(left);
            quick_sort(&mut slice[..left as usize - 1]);
            quick_sort(&mut slice[right as usize + 1..]);
        }
    }
}

fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for unsorted in 1..slice.len() {
        let mut cursor = unsorted;
        while cursor > 0 && slice[cursor - 1] > slice[cursor] {
            slice.swap(cursor, cursor - 1);
            cursor -= 1;
        }
    }
}

fn bubble_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        for j in 0..slice.len() {
            if slice[i] <= slice[j] {
                slice.swap(i, j)
            }
        }
    }
}

fn merge_sort<T: Ord + Copy>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
                return;
            }
        }
        _ => {}
    }
    let mid = slice.len() / 2;

    merge_sort(&mut slice[..mid]);
    merge_sort(&mut slice[mid..]);

    let mut buf = slice.to_vec();

    merge(&slice[..mid], &slice[mid..], &mut buf[..]);

    slice.copy_from_slice(&buf)
}
fn merge<T: Ord + Clone>(left: &[T], right: &[T], buf: &mut [T]) {
    let mut l_index = 0;
    let mut r_index = 0;
    let mut buf_index = 0;

    while l_index < left.len() && r_index < right.len() {
        if left[l_index] < right[r_index] {
            buf[buf_index] = left[l_index].clone();
            l_index += 1;
            buf_index += 1;
        } else {
            buf[buf_index] = right[r_index].clone();
            r_index += 1;
            buf_index += 1;
        }
    }
    if r_index < right.len() {
        for i in buf_index..buf.len() {
            buf[i] = right[r_index].clone();
            r_index += 1;
        }
    }
    if l_index < left.len() {
        for i in buf_index..buf.len() {
            buf[i] = left[l_index].clone();
            l_index += 1;
        }
    }
}
fn binary_search<T: PartialOrd + PartialEq>(slice: &[T], element: &T) -> Option<usize> {
    let mut mid = slice.len() / 2;
    let mut left = 0;
    let mut right = slice.len() - 1;
    while left <= right {
        if slice[mid] > *element {
            right = mid;
        } else if slice[mid] < *element {
            left = mid + 1;
        } else {
            return Some(mid);
        }
        mid = left + ((right - left) / 2);
    }
    None
}

#[test]
fn test() {
    let mut slice = [3, 2, 1];
    let mut slice1 = [1, 2, 3];
    let mut slice2 = [2, 1, 3];
    let mut slice4 = [3, 1, 2];
    let mut slice5 = [2, 1, 2];
    quick_sort(&mut slice);
    quick_sort(&mut slice1);
    quick_sort(&mut slice2);
    quick_sort(&mut slice4);
    quick_sort(&mut slice5);

    assert_eq!(slice, [1, 2, 3]);
    assert_eq!(slice1, [1, 2, 3]);
    assert_eq!(slice2, [1, 2, 3]);
    assert_eq!(slice4, [1, 2, 3]);
    assert_eq!(slice5, [1, 2, 2]);
    // assert_eq!(slice1, [1, 2, 3]);
}

#[test]
fn test4() {
    let mut slice = [3, 2, 1];
    let mut slice1 = [1, 2, 3];
    let mut slice2 = [2, 1, 3];
    let mut slice4 = [3, 1, 2];
    let mut slice5 = [2, 1, 2];
    bubble_sort(&mut slice);
    bubble_sort(&mut slice1);
    bubble_sort(&mut slice2);
    bubble_sort(&mut slice4);
    bubble_sort(&mut slice5);

    assert_eq!(slice, [1, 2, 3]);
    assert_eq!(slice1, [1, 2, 3]);
    assert_eq!(slice2, [1, 2, 3]);
    assert_eq!(slice4, [1, 2, 3]);
    assert_eq!(slice5, [1, 2, 2]);
    // assert_eq!(slice1, [1, 2, 3]);
}

#[test]
fn test1() {
    let mut slice = [3, 2, 1];
    let mut slice1 = [1, 2, 3];
    let mut slice2 = [2, 1, 3];
    let mut slice4 = [3, 1, 2];
    let mut slice5 = [2, 1, 2];
    insertion_sort(&mut slice);
    insertion_sort(&mut slice1);
    insertion_sort(&mut slice2);
    insertion_sort(&mut slice4);
    insertion_sort(&mut slice5);
    // slice.sort();
    assert_eq!(slice, [1, 2, 3]);
    assert_eq!(slice1, [1, 2, 3]);
    assert_eq!(slice2, [1, 2, 3]);
    assert_eq!(slice4, [1, 2, 3]);
    assert_eq!(slice5, [1, 2, 2]);
    // assert_eq!(slice1, [1, 2, 3]);
}

#[test]
fn test3() {
    let mut slice = [3, 2, 1];
    let mut slice1 = [1, 2, 3];
    let mut slice2 = [2, 1, 3];
    let mut slice4 = [3, 1, 2];
    let mut slice5 = [
        2, 1, 2, 7, 342, 123, 123, 1092983, 8237, 87126, 348, 128937, 254, 1872,
    ];
    merge_sort(&mut slice);
    merge_sort(&mut slice1);
    merge_sort(&mut slice2);
    merge_sort(&mut slice4);
    merge_sort(&mut slice5);
    // slice.sort();
    assert_eq!(slice, [1, 2, 3]);
    assert_eq!(slice1, [1, 2, 3]);
    assert_eq!(slice2, [1, 2, 3]);
    assert_eq!(slice4, [1, 2, 3]);
    println!("{slice5:?}")
    // assert_eq!(slice1, [1, 2, 3]);
}

#[test]
fn test2() {
    let mut slice = [3, 2, 1, 6, 7, 10, 3, 1, 7, 8, 0];
    slice.sort();
    assert_eq!(Some(0), binary_search(&slice, &0));
    // assert_eq!(slice1, [1, 2, 3]);
}

#[test]
fn make_heap_test() {
    let mut slice = [1, 2, 3, 4, 5, 6];
    make_heap(&mut slice);
    assert_eq!(slice, [6, 5, 3, 4, 2, 1]);

    // assert_eq!(slice1, [1, 2, 3]);
}

fn heapify<T: Ord>(slice: &mut [T], root: usize) {
    let mut largest = root;
    let l = 2 * root + 1;
    let r = 2 * root + 2;

    if l < slice.len() && slice[l] > slice[largest] {
        largest = l;
    }
    if r < slice.len() && slice[r] > slice[largest] {
        largest = r;
    }
    if largest != root {
        slice.swap(largest, root);
        heapify(slice, largest);
    }
}

fn make_heap<T: Ord>(slice: &mut [T]) {
    for i in (0..=slice.len() / 2 - 1).rev() {
        heapify(slice, i);
    }
}

const fn log2(s: u32) -> u32 {
    let mut log = 0;
    let mut s = s;
    while s ^ 1 != 0 {
        log += 1;
        s >>= 1;
    }
    log
}

#[test]
fn test_log2() {
    assert_eq!(log2(1), 0);
    assert_eq!(log2(2), 1);
    assert_eq!(log2(3), 1);
    assert_eq!(log2(4), 2);
    assert_eq!(log2(16), 4);
}

trait Test {
    fn print(&self);
}

struct Animal;

impl Test for Animal {
    fn print(&self) {
        println!("animal");
        println!("{}", size_of::<Animal>());
    }
}

struct Human(i32);

impl Test for Human {
    fn print(&self) {
        println!("human");
        println!("{}", size_of::<Human>());
    }
}
fn testing_p(t: &dyn Test) {
    t.print();
}

#[test]
fn test_dyn() {
    let h = Human(2);
    let a = Animal;

    testing_p(&h);
    testing_p(&a);
}
