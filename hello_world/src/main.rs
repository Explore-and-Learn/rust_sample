fn main() {
    println!("{}", get_fibonnaci_value_by_index(10));
    println!("{}", get_is_even(101));
}
//This is a comment
fn get_fibonnaci_value_by_index(i : i32) -> i32 {
    if i == 0 || i == 1 {
        return 1;
    } 
    return get_fibonnaci_value_by_index(i - 1) + get_fibonnaci_value_by_index(i - 2);
}

fn get_is_even(i : i32) -> bool {
    return 2%i == 0;
}
