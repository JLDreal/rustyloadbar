use std::{thread, time::Duration};
use std::io::Write;


fn right(mut len: usize, symbl: char){
    
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



fn left(mut len: usize, symbl: char){
    
    let mut load : String = String::new();
    len-= 5 + len.to_string().len();
    for i in 1..len {
    load += &String::from(symbl);
    
    print!("[");
    for _t in 1..( len.to_string().len() + 2 - i.to_string().len()){
        print!(" ");
    }
    
    print!("{}",i);
    
    
    print!(" ]");
    
    print!("{}\r", load);
    

    
    
    std::io::stdout().flush();
    thread::sleep_ms(10 );
    }
}
fn mid(mut len: usize, symbl: char){
    
    let mut load : String = String::new();
    len-= 5 + len.to_string().len();
    for i in 1..len {
    load += &String::from(symbl);
    print!("\r{}", load);
    
    
    print!("[");
    for _t in 1..( len.to_string().len() + 2 - i.to_string().len()){
        print!(" ");
    }
    
    
    print!("{}",i);
    
    
    print!(" ]");
    

    for _t in i..len{
        print!(" ");
    }
    std::io::stdout().flush();
    thread::sleep_ms(10 );
    }
}
fn main() {
    if let Some((w, h)) = term_size::dimensions() {
        println!("Width: {}\nHeight: {}", w, h);
        right(w, '#');
        
        mid(w,'%');
        left(w,'=');
        } else {
        println!("Unable to get term size :(")
    }
    
}
