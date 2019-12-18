// rust a des types statiques (statically typed)
// chaque variable doit avoir un type pour compiler

// 4 types scalaires : entiers, nombres flottants, booléens, caractères
    // les entiers, sans signe (u) et avec signe (i) et leur nombre de bit (i32)
    // les nombres à virgule flottante, f64 et f32
    // booléen, true et false (ça prend un octet de mémoire)
    // les caractères, introduit par des guillemets simples ' '

// Types composés (compound types), de deux sortes
    // n-uplets (tuples). Leur longueur est déclarée, et ne bouge plus
    // Les 'arrays', ou tableaux

// Intéressant : le compilateur va signaler des erreurs (d'indexation par exemple)
// plus rigoureusement que dans d'autres langages.

fn main() {
   
    // nombres à virgule flottante (floating-point), préfixe f et nombre de bits
    // f64 par défaut pour x
    let x = 2.0; 
    // précisons qu'on veut du f32
    let y: f32 = 3.0; 
    println!("x = {} et y = {}", x, y);

    // quelques opérations
    let somme = 5 + 10;
    let difference = 95.5 - 4.3;
    let produit = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reste = 43 % 5;
    println!("somme = {}, différence = {}, produit = {}, quotient = {}, reste = {}",
            somme, difference, produit, quotient, reste);
    
    // on déclare les variables boléennes simplement
    let t = true;
    // ou bien on précise le type
    let f: bool = false;
    println!("Vrai c'est {}, faux c'est {}", t, f);
    // les booléennes ne sont concrètement utilisable que dans les conditions (if)

    // les caractères. L'UTF-8 passe, mais c'est quand même pas idéal
    let c = 'z';
    let o = "Œ";
    let heart_eyed_cat = '😻'; // je savais pas que c'était de l'unicode !
    println!("On peut utiliser des caractères comme des nombres : \
            {} {} {}", c, o, heart_eyed_cat);


// les variables composées

//n-uplets ou tuples
    // créons un n-uplets de trois termes, avec un type par position
    let nuplet: (i32, f64, u8) = (500, 6.4, 1);
    println!("Afficher un n-uplet (tuple) n'est pas facile.");

    // on peut déstructurer le nuplet, y mettre trois variables
    let (_l, _m, _n) = nuplet;
    println! ("Mais on peut afficher ses éléments séparément. \
                Le deuxième élément de notre n-uplet est _m : {}", _m);

    // on peut accéder à un élément du n-uplet avec un point et un numéro
    // attention ! On compte à partir de zéro !
    let v: (i32, f64, u8) = (-400, 45.12, 25);
    println!("Imprimons le deuxième terme de v avec 'v.1' : {}", v.1);

// Arrays ou tableaux
    // Dans un array, tous les éléments sont de même type
    // La longueur d'un array est fixe. 
    // Mieux vaudra utiliser un vecteur pour le faire croître.
    let _ar = [1, 2, 3, 4, 5];
   
    // comme on n'a pas plus de douze mois dans l'année, on peut utiliser un array
    let mois =["Janvier", "Février", "Mars", "Avril", "Mai", "Juin", 
                "Juillet", "Août", "Septembre", "Octobre", "Novembre", "Décembre"];
    // pour imprimer un élément, utiliser l'indexation des array avec des []
    println!("Pas facile d'afficher un array. Mais on peut imprimer un élément. \
         Troisième mois de l'année : {}", mois[2]);
            
    // pour préciser le type d'un array, on fait [type; nombre d'éléments]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

} 


