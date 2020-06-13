use std::{cmp::Ordering, collections::BTreeMap, ops::Deref};

pub fn vowels_slices_occurrences<'a>(strings: &'a [&'a str]) -> BTreeMap<StrWrap<'a>, u32> {
    let mut occurrences = BTreeMap::new();

    let mut inc_slice = |slice| {
        occurrences
            .entry(StrWrap(slice))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    };

    for string in strings {
        let mut start = None;
        string
            .char_indices()
            .map(|(index, c)| (index, c.to_ascii_lowercase()))
            .for_each(|(index, c)| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    if start.is_none() {
                        start = Some(index);
                    }
                }
                _ => {
                    if let Some(start_index) = start {
                        inc_slice(&string[start_index..index]);
                        start = None;
                    }
                }
            });

        if let Some(start) = start {
            inc_slice(&string[start..]);
        }
    }
    occurrences
}

#[derive(Copy, Clone)]
pub struct StrWrap<'a>(&'a str);

impl AsRef<str> for StrWrap<'_> {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl Deref for StrWrap<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl PartialOrd for StrWrap<'_> {
    fn partial_cmp(&self, other: &StrWrap<'_>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StrWrap<'_> {
    fn cmp(&self, other: &StrWrap<'_>) -> Ordering {
        self.0
            .chars()
            .zip(other.0.chars())
            .map(|(a, b)| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()))
            .find(|order| !matches!(order, Ordering::Equal))
            .unwrap_or_else(|| self.0.len().cmp(&other.0.len()))
    }
}

impl PartialEq for StrWrap<'_> {
    fn eq(&self, other: &StrWrap<'_>) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl Eq for StrWrap<'_> {}
