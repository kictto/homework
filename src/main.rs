fn i32_bubble_sort(array: &mut Vec<i32>) {
    let mut i = 0usize;
    while i < array.len() {
        let mut j = i + 1;
        while j < array.len() {
            if array[i] > array[j] {
                (array[i], array[j]) = (array[j], array[i]);
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

fn i32_sort() {
    let mut a = vec![1, 2, 3, 4, 5, 6];
    let mut b = vec![6, 5, 4, 3, 2, 1];
    let mut c = vec![1, 6, 2, 5, 5, 3, 4, 6];
    println!("Before Sort a:{:?}", a);
    i32_bubble_sort(&mut a);
    println!("After  Sort a:{:?}", a);

    println!("Before Sort b:{:?}", b);
    i32_bubble_sort(&mut b);
    println!("After  Sort b:{:?}", b);

    println!("Before Sort c:{:?}", c);
    i32_bubble_sort(&mut c);
    println!("After  Sort c:{:?}", c);
}

fn main() {
    println!("Hello, world!");
    println!("i32Sort:");
    i32_sort();
    println!("============= i32Sort end.")
}
