use std::{thread, time::Duration};
use std::io::Write;

fn normal(mut len: usize, symbl: char){
    let mut load : String = String::new();
    len-= 5 + len.to_string().len();
    for i in 1..len {
    load += &String::from(symbl);
    print!("\r{}", load);
    
    for _t in i..len{
        print!(" ");
    }
    print!("[");
    for _t in 1..( len.to_string().len() + 2 - i.to_string().len()){
        print!(" ");
    }
    
    print!("{}",i);
    
    
    print!(" ]");
    std::io::stdout().flush();
    thread::sleep_ms(10 );
    }
}
fn main() {
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        normal(w, '#');
        } else {
        println!("Unable to get term size :(")
    }
    
}
