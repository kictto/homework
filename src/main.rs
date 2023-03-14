use std::cmp::Ordering;
use rand::Rng;
/// i32的冒泡排序
fn i32_bubble_sort(array: &mut Vec<i32>) {
    let mut i = 0usize;
    while i < array.len() {
        let mut j = i + 1;
        while j < array.len() {
            if array[i] > array[j] {
                (array[i], array[j]) = (array[j], array[i]);
            }
            j += 1;
        }
        i += 1;
    }
}
/// i32 冒泡排序示例
fn i32_sort_example() {
    let mut a = Vec::new();
    for _ in 0..20 {
        a.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("Before Sort :{:?}", a);
    i32_bubble_sort(&mut a);
    println!("After  Sort :{:?}", a);
}
/// 基于 PartialOrd 的通用冒泡排序
fn normal_bubble_sort<T: PartialOrd>(array: &mut Vec<T>) {
    let mut i = 0usize;
    while i < array.len() {
        let mut j = i + 1;
        while j < array.len() {
            if array[i].partial_cmp(&array[j]) == Some(Ordering::Greater) {
                array.swap(i, j);
            }
            j += 1;
        }
        i += 1;
    }
}
/// 基于 PartialOrd 的通用冒泡排序 示例
fn normal_bubble_sort_example() {
    let mut a = Vec::new();
    for _ in 0..20 {
        a.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("Before Sort :{:?}", a);
    normal_bubble_sort(&mut a);
    println!("After  Sort :{:?}", a);
}

fn main() {
    println!("Hello, world!");
    println!("i32Sort:");
    i32_sort_example();
    println!("============= i32Sort end.");

    println!("Normal Sort:");
    normal_bubble_sort_example();
    println!("============= Normal Sort end.");
}
