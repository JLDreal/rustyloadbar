use std::{thread, time::Duration};

fn normal(len: u8, symbl: char){
    let mut load : String = String::new();
    for i in 1..len {
    // load += &String::from(symbl);

    print!("{}",symbl);
    thread::sleep_ms(400);
    }
}
fn main() {
    normal(10, '#')
}
