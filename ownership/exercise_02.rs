#[cfg(test)]
mod tests {
    use super::*;

    const DATA: [&str; 3] = ["aIebaeabbioubbAiebaiE", "baiebbiouioub", "ioubioub"];

    // Remove one of the test if you want to test only one version of the exercise

    #[test]
    fn count_vowel_slices_occurrences() {
        let mut result: Vec<_> = vowels_slices_occurrences(&DATA)
            .into_iter()
            .map(|(slice, occurrences)| (slice.to_lowercase(), occurrences))
            .collect();
        result.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

        assert_eq!(
            result,
            vec![
                ("aea".to_string(), 1),
                ("aie".to_string(), 4),
                ("iou".to_string(), 3),
                ("iouiou".to_string(), 1),
            ]
        );
    }

    #[test]
    fn count_vowel_slices_occurrences_case_sensitive() {
        let mut result: Vec<_> = vowels_slices_occurrences_case_sensitive(&DATA)
            .into_iter()
            .map(|(slice, occurrences)| (slice, occurrences))
            .collect();
        result.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

        assert_eq!(
            result,
            vec![
                ("Aie", 1),
                ("aIe", 1),
                ("aea", 1),
                ("aiE", 1),
                ("aie", 1),
                ("iou", 3),
                ("iouiou", 1),
            ]
        );
    }
}
