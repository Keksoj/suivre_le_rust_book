// 2019-01-27

// la macro panic! affiche un message d'erreur, rembobine et nettoie le tas
// puis quitte le programme.

fn main() {

    // syntaxe de base : 
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    // Ceci amène à paniquer :
    v[99];
}
