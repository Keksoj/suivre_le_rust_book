// La boucle loop, sans surprise, est conçue pour répéter du code.
// On l'arrête avec break

// Un exemple concret avec une variable qu'on incrémente à chaque tour.
fn main() {

    // définissons une variable modifiable
    let mut counter = 0;

    // boucle
    loop {

        // incrémentation
        counter += 1;
        
        // afficher la variable
        println!("{}", counter);

        // arrêter la boucle une fois arrivé à 10
        if counter == 10 {
            break
        }
    }
}
