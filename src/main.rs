use std::cmp::Ordering;
use rand::Rng;

/// i32的选择排序
fn i32_sort_selection(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        for j in i + 1..array.len() {
            if array[i] > array[j] {
                (array[i], array[j]) = (array[j], array[i]);
            }
        }
    }
}
/// i32的选择排序2
fn i32_sort_selection_2(array: &mut Vec<i32>){
    for i in 0..array.len()-1 {
        let mut min = i;
        for j in i + 1..array.len() {
            if array[min] > array[j] {
                min = j;
            }
        }
        (array[i], array[min]) = (array[min], array[i]);
    }
}
/// i32的冒泡排序
fn i32_bubble_sort(array: &mut Vec<i32>) {
    for i in 0..array.len() - 1 {
        for j in 0..array.len() - 1 - i {
            if array[j] > array[j + 1] {
                (array[j], array[j + 1]) = (array[j + 1], array[j]);
            }
        }
    }
}

/// i32 冒泡排序示例
fn i32_sort_example() {
    let mut a = Vec::new();
    for _ in 0..20 {
        a.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("Before Sort : {:?}", a);
    i32_bubble_sort(&mut a);
    println!("After  Sort : {:?}", a);
}

/// 基于 PartialOrd 的通用冒泡排序
fn normal_bubble_sort<T: PartialOrd>(array: &mut Vec<T>) {
    for i in 0..array.len() {
        for j in i + 1..array.len() {
            if array[i].partial_cmp(&array[j]) == Some(Ordering::Greater) {
                array.swap(i, j);
            }
        }
    }
}

/// 基于 PartialOrd 的通用冒泡排序 示例
fn normal_bubble_sort_example() {
    let mut a = Vec::new();
    for _ in 0..20 {
        a.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("Before Sort : {:?}", a);
    normal_bubble_sort(&mut a);
    println!("After  Sort : {:?}", a);
}

#[derive(Debug, Eq)]
struct User {
    user_name: String,
    sign_up_timestamp: u64,
}

impl PartialEq<Self> for User {
    fn eq(&self, other: &Self) -> bool {
        self.sign_up_timestamp.eq(&other.sign_up_timestamp)
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.sign_up_timestamp.cmp(&other.sign_up_timestamp))
    }
}

fn normal_bubble_sort_example2() {
    let mut a = Vec::new();
    for i in 0..3 {
        a.push(User {
            user_name: format!("User-{}", i),
            sign_up_timestamp: rand::thread_rng().gen_range(1..=100),
        });
    }
    println!("Before Sort : {:#?}", a);
    normal_bubble_sort(&mut a);
    println!("After  Sort : {:#?}", a);
}

fn main() {
    println!("Hello, world!");
    println!("i32Sort:");
    i32_sort_example();
    println!("============= i32Sort end.");

    println!("PartialOrd Sort:");
    normal_bubble_sort_example();
    println!("============= PartialOrd Sort end.");

    println!("PartialOrd Struct Sort :");
    normal_bubble_sort_example2();
    println!("============= PartialOrd Struct Sort end.");
}
