use lists::second::List;

fn main() {
    println!("Hello Monde !");
    let mut liste = List::new();
    liste.push(35);
    liste.push(24);
    liste.push(14);
    println!("{}", liste.pop().unwrap());
}
