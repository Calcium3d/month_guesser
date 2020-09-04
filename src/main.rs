use rand::Rng;

fn main() {
    let month = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let number: usize = rand::thread_rng().gen_range(1, 13);
    let month_number = month[number];
    println!("The month selected is: {} ", month_number);
}
