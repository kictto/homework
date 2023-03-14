use rand::Rng;

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

fn i32_sort_example() {
    let mut a = Vec::new();
    for _ in 0..10 {
        a.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("Before Sort :{:?}", a);
    i32_bubble_sort(&mut a);
    println!("After  Sort :{:?}", a);
}

fn normal_bubble_sort<T: PartialOrd>(array: &mut Vec<T>) {
    let mut i = 0usize;
    while i < array.len() {
        let mut j = i + 1;
        while j < array.len() {
            if array[i] > array[j] {
                array.swap(i,j);
            }
            j += 1;
        }
        i += 1;
    }
}

fn normal_bubble_sort_example(){
    let mut a = Vec::new();
    for _ in 0..10 {
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
