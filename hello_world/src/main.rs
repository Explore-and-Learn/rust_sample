fn main() {
    println!("{}", get_fibonnaci_value_by_index(10));
}
//This is a comment
fn get_fibonnaci_value_by_index(i : i32) -> i32 {
    if i == 0 {
        return 1;
    }   
    if i == 1 {
        return 1;
    }
    return get_fibonnaci_value_by_index(i - 1) + get_fibonnaci_value_by_index(i - 2);
}
