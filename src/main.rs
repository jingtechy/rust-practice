fn main() {
    let mut my_int_array: [i32; 6] = [64, 34, 25, 12, 22, 11];
    bubble_sort(&mut my_int_array);
    println!("My int array is sorted: {:?}", my_int_array);

	let mut my_str_array: [&str; 5]= ["happy","hello","to","birthday","you"];
	bubble_sort(&mut my_str_array);
	println!("My str array is sorted: {:?}", my_str_array);
}


fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let size = arr.len();

    for i in 0..size {
        for j in 0..size - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
