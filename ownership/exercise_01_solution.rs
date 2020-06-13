use std::{convert::identity, iter};

pub fn vowels_slices<'a>(strings: &'a [&'a str]) -> Vec<(usize, usize, &'a str)> {
    strings
        .iter()
        .enumerate()
        .flat_map(|(string_index, s)| {
            s.char_indices()
                .chain(iter::once((s.len(), '!')))
                .scan(None, move |start, (char_index, c)| {
                    match c.to_ascii_lowercase() {
                        'a' | 'e' | 'i' | 'o' | 'u' => {
                            if start.is_none() {
                                *start = Some(char_index);
                            }
                            Some(None)
                        }
                        _ => match *start {
                            Some(start_index) => {
                                *start = None;
                                Some(Some((
                                    string_index,
                                    start_index,
                                    &s[start_index..char_index],
                                )))
                            }
                            None => Some(None),
                        },
                    }
                })
                .filter_map(identity)
        })
        .collect()
}
