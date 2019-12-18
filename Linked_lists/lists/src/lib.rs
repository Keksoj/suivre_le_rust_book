// 2019-04-26
// le super tuto trouvé en ligne

// utiliser la première liste
pub mod first;
pub mod second;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
