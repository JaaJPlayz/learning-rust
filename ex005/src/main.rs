#![allow(unreachable_code)]

fn main() {
    let my_arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut my_arr_index = 0;
    let my_arr_size = my_arr.len();

    while my_arr_index < my_arr_size - 1 {
        print!("{:?} -> ", my_arr[my_arr_index]);
        my_arr_index += 1;
    }
    print!("{:?}", my_arr[my_arr_index]);
}
