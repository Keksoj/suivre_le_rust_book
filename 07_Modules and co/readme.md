### Modules et compagnie

Le chapitre 7 du manuel répond à ce genre d'interrogation : comment gérer ce qu'on peut utiliser dans un scope ?
Quelles fonctions ? Quelles variable ?

Ce qu'on utilise :
* **Les packages** créent, testent, et partagent des *crates*
* **Les crates** sont des arborescences de modules qui créent des bibliothèques ou exécutables.
* **Les modules** et le mot-clé **use** contrôlent l'étendue et le caractère privé des chemin. Je sais pas ce que ça veut dire.
* **Un chemin** (*path*) c'est une manière de trouver un objet (un struct, une fonction, un module) dans l'arborescence.

On va voir ce que ça veut dire tout ce charabia.

##### Packages et crates

Taper `cargo new` crée un package, on le voit car le fichier `Cargo.toml` a été créé. Il contient par exemple

```
[package]
name = "mon-projet"
version = "0.1.0"
authors = ["Emmanuel Bosquet <bjokac@gmail.com>"]
edition = "2018"

[dependencies]
```
Ce fichier ne parle pas du fichier source contenu dans `src/`, qui s'appelle `main.rs`.
Mais Cargo saura que `src/main.rs` est le `crate root`. C'est quoi le crate root ?
De même, `src/lib.rs` sera le `crate root` du crate bibliothèque (*library crate*).
Je comprends encore rien, sinon que Cargo enverra ces fichiers crate root à `rustc`, le compilateur.

Deux types de package :
* *library* avec `lib.rs`, il y en a soit un seul, soit aucun.
* *binary* avec `main.rs`, autant qu'on veut.

#### Modules

Les modules sont un moyen d'organiser le code en arborescence.
Les fonctions ont ainsi un **chemin**.
* **Chemin absolu** (*absolute path*), part d'un crate root en utilisant le nom d'un crate ou un crate litéral;
* **Chemin relatif** (*relative path*), part du module actuel et utilise `self`, `super`, ou un identifiant du module actuel.

```rs
    // chemin absolu
    crate::sound::instrument::clarinette();

    // chemin relatif
    sound::instrument::clarinette();
```

Ils sont tous deux suivis par un ou plusieurs identifiants séparés par des `::`

**La vie privée des modules**

Les modules sont privés par nature.
Si l'on définit une fonction ou un struct à l'intérieur, il ou elle devient automatiquement privé.
Les règles de la vie privée :
* Tous les objets sont privés par défauts (fonctions, méthodes, enums...)
* On utilise `pub` pour rendre un objet public
* On ne peux pas utiliser de code privé défini dans un module en aval dans l'arborescence
* On peut utiliser n'importe quel code, privé ou non, dans le module actuel, ou en amont dans l'arborescence.

Un peu comme dans un système de fichier. Si j'ai accès à `/usr/share/` c'est que j'ai forcément accès à `/usr/`, mais je n'ai pas forcément accès à `/usr/share/xml`.

On utilise également `super` pour aller chercher un objet à partir d'un étage au-dessus dans l'arborescence, comme avec `..` sur linux. Par exemple
```rs
mod instrument {
    fn clarinette() {
        super::fonction();
    }
}

fn fonction() {
    // contenu de la fonction
}
```
va chercher `fonction()` un degré au-dessus, donc une indentation à gauche.
C'est plus pratique pour transférer du code d'un endroit à un autre.
La fonction() et son lien bougent ensemble mais leur position relative demeure.


##### Un exemple donné par le maître
Voici ce que Jean-Philippe me donne pour illustrer l'affaire.
```rs
mod reality {
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    impl Person {
        pub fn new(name: &str, age: u8) -> Person {
            if age > 130 {
                panic!();
            }

            Person {
                name: String::from(name),
                age
            }
        }
    }
}

fn main() {
    // This panics.
    let person = reality::Person::new("Toto", 140);

    // This is possible.
    let person = reality::Person {
        name: String::from("toto"),
        age: 140,
    };
}
```
Il dit :
> Si tout est public, n’importe-qui peut manipuler les données.
> Si tu rends public ta structure `Person` mais pas son contenu, on dit que tu l’encapsules : seul le module peut manipuler les données.
> L’idée, c’est de sécuriser l’interface pour que l’utilisateur de ton module ne puisse pas faire n’importe quoi


##### Utiliser `use`

Pour appeler un chemin, on n'est pas obligé de faire

```rs
mod sound
    pub mod instrument {
        pub fn clarinet() {
            // définition de la fonction
        }
    }
}

fn main() {
    crate::sound::instrument::clarinette()
```

On peut utiliser `use` ainsi :

```rs
mod sound
    pub mod instrument {
        pub fn clarinet() {
            // définition de la fonction
        }
    }
}

// utiliser use avec un chemin absolu
use crate::sound::instrument;

// use avec un chemin relatif
use self::sound::instrument;

fn main() {
    instrument::clarinet();
}
```
Parfois c'est plus pratique.

##### Combiner `pub` et `use`

Quand on apporte un nom dans le scope avec `use`, le nom est privé. Utiliser `pub` permettra au code d'appeler ce nom et le code associé. On appelle ça *ré-exporter*.
Il s'agit non seulement d'amener un objet dans le scope, mais encore de le rendre disponible à d'autres pour l'amener dans leurs propre scope.


#### Utiliser un package externe

On édite le fichier `Cargo.toml` comme on l'a fait pour le programme Devine.
```
[dependencies]
rand = "0.5.5"
```
Ce code ajoute `rand` comme dépendance, Cargo télécharge le package `rand` avec ses dépendances, depuis *https//crates.io*. Le code sera dispo pour le projet.

Pour amener `rand` dans le scope du package, on utilise `use` ainsi : 
```rs
use rand::Rng;
```
`Rng`, un trait, est l'objet qu'on veut amener dans le scope. On pourra maintenant utiliser les fonctions offertes par `Rng` avec la syntaxe suivante :
```rs
fn main() {
    let nombre_secret = rand::thread_rng().gen_range(1,101);
```

**La bibliothèque standard**, (*standard library*), `std` de son petit nom, est externe à notre package et doit être appelée avec `use`. Cependant, on n'a pas besoin de modifier `Cargo.toml`. Un chemin absolu ressemble alors à 
```rs
use std::collections::HashMap;
```

#####Syntaxe plus compacte
Au lieu de 
```rs
use std::cmp::Ordering;
use std::io;
// ---etc---
```
On peut écrire
```rs
use sdt::{cmp::Ordering, io};
// ---etc---
```
On appelle ça un *nested path*.

De même, on peut remplacer 
```rs
use std::io;
use std::io::Write;
```
par
```rs
use std::io::{self, Write};
```

Magie noire suprême, on peut amener *tous* les objets publics d'un chemin dans le scope en utilisant l'opérateur `glob`, l'astérisque.
```rs
use std::collections::*;
```
**Attention** ça amène plein de noms utilisables. Attention aux doublons.


### Mettre les modules dans d'autres fichiers

Le fichier d'un module suit cette syntaxe `src/module.rs`. Il se trouve dans `src/` à côté de `main.rs`, il porte son propre nom et l'extension `.rs`.

`main.rs` ressemble alors à ça : 

```rs
mod module;

main() {
    // contenu avec un chemin du genre module::objet
}
```

Et `module.rs` ressemble à ça
```rs
pub struct Machin { }
impl Machin {}
```
**Attention !** Il ne faut pas laisser la ligne `mod module {` ni l'accolade fermée `}` dans le fichier `module.rs`.















