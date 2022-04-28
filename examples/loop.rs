use std::io::stdin;

extern crate no_free_alloc;

fn main() {
    for i in 0..1000000 {
        let mut s = String::from("YEAH!!! ");
        s.push_str(&format!("{}", i));
        println!("{}", s);
    }
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
}
