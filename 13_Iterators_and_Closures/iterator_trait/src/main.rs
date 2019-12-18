// 2019-04-08

// On peut créer des itérateur pour toutes les collections, même les hash maps
// Pour définir un trait pour itérateur, il faut nécessairement définir la
// méthode next(). Les autres méthodes sont ensuites directement utilisables
// du moment qu'elles ont des implémentations par défaut.

fn main() {
    println!("Hello, world!");

    let mut truc = Counter::new();
    println!("Voici ce que renvoie la méthode next() : {:?}", truc.next());
    println!("Appliquons next() encore une fois, et déballons : {}", truc.next().unwrap());

    let mut autre = Counter::new().zip(Counter::new());
    println!("\nVoici ce qui se produit si l'on zippe notre itérateur avec lui-même en faisant
    \nlet mut autre = Counter::new().zip(Counter::new());
    \n{:#?}
    \nIl est de type
    std::iter::Zip<Counter, Counter>", autre);

    println!("\nLa même chose incrémentée avec next() :
    {:?}", autre.next());

    println!("\nEncore un tour de next() :
    {:?}", autre.next());

    println!("\nAffichons sans appliquer next() :
    {:?}", autre);

    // Un peu de magie noire. Si j'ai bien compris, on fait
    // 1 + 1
    // 2 * 2
    // 3 * 3
    // 4 * 4
    // 5 * 5
    // et la somme du tout
    let neuf = Counter::new().zip(Counter::new());
    let machin: u32 = neuf.map(|(a, b)| a * b).sum();
    println!("\nAppliquons map(|(a, b)| a * b).sum() sur deux itérateurs zipés:
    {:#?}", machin);
}

#[derive(Debug)]
struct Counter {
    count: u32,  // ce champs est privé, c'est l'implémentation qui y touche,
                 // pas le main()
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

// Implémentons le trait Iteraror pour le type Counter
// rustc exige qu'on implément le type Item et la méthode next()
impl Iterator for Counter {
    type Item = u32;   // ce qui veut dire : l'itératuer renvoie des valeurs u32

    fn next(&mut self) -> Option<Self::Item> {

        // incrémentation
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }

    // Une fois next() définie, on peut utiliser les méthodes par défaut

}

// Testons la méthode next()
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// Un deuxième test où l'on s'amuse bien
#[test]
fn using_other_iterator_trait_methods() {

    // On fabrique une instance de Counter avec new()
    // On le zippe (pas d'autre mot) avec un autre itérateur tout neuf DONT ON
    // AURA SAUTÉ LE PREMIER ÉLÉMENT avec skip(1)
    // on multiplie les élements de chaque paire
    // on ne garde que les résultats divisibles par 3
    // on additionne toutes ces valeurs ensemble
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
