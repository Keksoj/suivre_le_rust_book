// En déplaçant (move) une variable, 
// on laisse son heap intact mais on l'alloue à une autre variable.
// Comment faire pour avoir deux fois le même texte dans le heap ? On clone.
// clone est une MÉTHODE

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("Voici s1 : {}, \net voici s2 : {}", s1, s2);
}
