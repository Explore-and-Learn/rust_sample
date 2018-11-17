use std::mem;

fn main() {
	let t:f64 = get_fibonnaci_value_by_index(10);
    println!(" Fibonacci value is {}, size = {}", t, mem::size_of_val(&t));
    println!("{}", get_is_even(101));
    println!("{}", is_palindrone("racecar"));
    println!("{}", is_palindrone("notApalidrone"));
    println!("ididi is a palidrone? {}", is_palindrone("ididi"));
	println!("{}^pi = {}", t, f64::powf(t, std::f64::consts::PI));

	let c = 1 | 2;
	println!("1|2 is {}", c);
	let two_to_11  = 1 << 11;
	println!("1 << 11 = {}", two_to_11);
}
//This is a comment
fn get_fibonnaci_value_by_index(i : i32) -> f64 {
    if i == 0 || i == 1 {
        //This is a test comment
        return 1.0;
    } 
    return get_fibonnaci_value_by_index(i - 1) + get_fibonnaci_value_by_index(i - 2);
}

fn get_is_even(i : i32) -> bool {
    return 2%i == 0;
}

fn is_palindrone(s : &str) -> bool {
    return s == s.chars().rev().collect::<String>()
}
