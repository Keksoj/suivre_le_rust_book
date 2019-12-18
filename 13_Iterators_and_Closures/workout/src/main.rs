// 2019-04-05

// Les Closures permettent de... j'ai pas encore compris
// Nous sommes dans une salle de sport et l'utilisateur demande à l'algorithme de
// lui créer un programme d'entraînement. L'algorithme a deux arguments :
    //  l'intensité des efforts à fournir
    // un nombre aléatoire pour amener de la variété dans le programme de sport

use std::thread;
use std::time::Duration;

fn main() {
    // On skipe l'invite utilisateur et la génération du nombre aléatoire
    // On les hardcode, arbitrairement
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    )
}


// Cette fonction simule un temps de calcul de deux secondes
// Elle prends un nombre et renvoie exactement le même.
// Pourquoi intensity ? C'est un nombre rentré par l'utilisateur pour choisir
// la quantité d'efforts qu'il veut fournir à la salle de sport.
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("On y passe du temps...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// Voici l'algorithme un peu bidon qui génère un programme de sport
// Il prend les deux arguments : l'intensité donnée par l'utilisateur, et
// un nombre aléatoire (les deux sont hardcoded de toute façon)
// On y appelle beaucoup la fonction qui simule un temps de calcul.
fn generate_workout(intensity: u32, random_number:u32) {

    // Voici une closure !
    // Son paramètre est rédigé entre des pipes verticales (|), comme dans Ruby
    // Ensuite, des accolades pour le contenu (pas obligatoire si une seule
    // déclaration). Comme dans une fonction, la dernière ligne renvoie une
    // valeur.
    // ATTENTION ! cet 'expensive_closure' ne continue pas la valeur retour,
    // mais contient la DÉFINITION de cette fonction anonyme.
    // La closure permet de stocker du code et de s'en servir ailleurs ;-)

    // let expensive_closure = |num| {
    //     println!("On y passe du temps...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // On POURRAIT indiquer les type de donnée ainsi :
    // let expensive_closure = |num: u32| -> u32 {}
    // mais ça serait redondant, le compilateur ne l'exige pas. Il infère.
    // Mais on ne pourra pas utiliser une même closure avec des types différents

    // Créons une instance du struct Cacher avec sa méthode new(), qui prend en
    // argument une closure que nous définissons ici et qui a pour but principal
    // de mettre le thread sur pause
    let mut expensive_result = Cacher::new(|num| {
        println!("On y passe du temps...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Aujourd'hui, faites {} pompes !",

            // Ceci appelle la fonction value() qui prend intensity comme argument
            // Cette fonction est logée dans le struct Cacher, dont expensive_result
            // est une instance
            expensive_result.value(intensity)
        );
        println!(
            "Ensuite, faites {} tractions !",

            // On appelle cette fonction une deuxième fois, et comme la valeur
            // existe déjà, on n'a pas besoin d'attendre les deux secondes
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Faites une pause aujourd'hui ! Restez hydraté !");
        } else {
            println!(
                "Aujourd'hui, courrez {} minutes !",
                expensive_result.value(intensity)
            );
        }
    }
}

// We can create a struct that will hold
    // the closure
    // and the the resulting value of calling the closure
// Le struct exécutera la closure (fermeture) seulement si on a besoin de la
// valeur retour, et il la CACHERA au reste du code, qui n'en sera pas
// responsable.
// Ce struct va gérer lui-même ses valeurs, qui peuvent rester privées.
struct Cacher<T>
    // T est une CLOSURE qui doit implémenter le TRAIT Fn(u32) -> u32
    where T: Fn(u32) -> u32
{
    calculation: T, // la closure elle-même
    value: Option<u32>, // la valeur qui sera produite par la closure
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{

    // cette fonction new() crée une instance du struct Cacher
    // Elle prend en argument une closure
    // Et retourne une instance de Cacher qui contient la closure et un None
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation, // cette variable est une closure !
            value: None,
        }
    }

    // Cette fonction value() doit renvoyer une valeur
    fn value(&mut self, arg: u32) -> u32 {

        // Elle vérifie ce que contient le champ 'value'
        match self.value {

            // S'il contient une valeur, on la renvoie
            Some(v) => v,

            // Et en cas de None,
            None => {

                // on exécute la fermeture (closure) 'calculation' contenue
                // dans le struct, avec comme le même argument que la fonction
                let v = (self.calculation)(arg);

                // On attribue cette valeur au champ 'value' du struct
                self.value = Some(v);

                // on retourne cette valeur
                v
            }
        }
    }
}
