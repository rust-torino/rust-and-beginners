#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_vowels_slices() {
        const DATA: [&str; 3] = ["aEiobbAebOiUbba", "babAbababAbaba", "bbbbbbb"];

        assert_eq!(
            vowels_slices(&DATA),
            vec![
                (0, 0, "aEio"),
                (0, 6, "Ae"),
                (0, 9, "OiU"),
                (0, 14, "a"),
                (1, 1, "a"),
                (1, 3, "A"),
                (1, 5, "a"),
                (1, 7, "a"),
                (1, 9, "A"),
                (1, 11, "a"),
                (1, 13, "a"),
            ]
        );
    }
}
