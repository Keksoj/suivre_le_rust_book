// 2019-07-07
// Le trait Drop permet de customise ce qui se passe quand une valeur sort du
// scope. La plupart du temps, on implémente Drop en cas de Smart Pointer.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};
    println!("CustomSmartpointers created.");
} // au moment de droper c et d, la méthode drop() est appelée ici.
