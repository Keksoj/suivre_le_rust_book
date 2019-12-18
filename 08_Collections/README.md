### Les collections 

Contrairement aux **tableaux** (*arrays*) et **n-uplets** (*tuples*), les collections que nous allons voir sont stockées sur le **tas** (*heap*). Leur taille ne sera donc pas limitée, on pourra les agrandir ou les réduire à volonté.

Les trois principales collections :
* Les **vecteurs** (*vector*) contiennent des valeurs côte à côte
* Les **chaînes de caractères** (*string*) qu'on a déjà vu
* Les **cartes de hash** (*hash map*) permettent d'associer une valeur à une clé. 

#### Les vecteurs
Un vecteur ne peut contenir que des valeurs de même type. Pratique pour les listes.
Quand on sort du scope, la mémoire est nettoyée, avec toutes les valeurs contenues pas le vecteur.

Dans un scope, on ne peut pas avoir à la fois une référence modifiable et une référence immuable à un même vecteur.
