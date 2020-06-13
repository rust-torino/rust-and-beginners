use std::convert::TryFrom;

const WIN_SIZE: usize = 5;

pub fn all_same_sum_tuples(data: &[u32]) -> Vec<(&[u32; WIN_SIZE], &[u32])> {
    data.windows(WIN_SIZE)
        .enumerate()
        .map(|(start_index, window)| {
            (
                <&[u32; WIN_SIZE]>::try_from(window).unwrap(),
                data.iter().skip(WIN_SIZE + start_index),
                start_index + WIN_SIZE,
            )
        })
        .filter_map(|(window, rest_iter, rest_start_index)| {
            let window_sum: u32 = window.iter().copied().sum();
            rest_iter
                .enumerate()
                .filter(|&(_, x)| x <= &window_sum)
                .map(|(rest_offset, &x)| (rest_offset + rest_start_index, x))
                .with_iter()
                .filter_map(|((first_index, x), rest)| {
                    rest.scan(x, |acc, (index, cur)| {
                        *acc += cur;
                        Some((index, *acc))
                    })
                    .take_while(|&(_, x)| x <= window_sum)
                    .last()
                    .filter(|&(_, sum)| sum == window_sum)
                    .map(|(last_index, _)| &data[first_index..=last_index])
                })
                .enumerate()
                .max_by(|(index_a, slice_a), (index_b, slice_b)| {
                    slice_a
                        .len()
                        .cmp(&slice_b.len())
                        .then_with(|| index_a.cmp(index_b))
                })
                .map(|(_, slice)| (window, slice))
        })
        .collect()
}

trait WithIterExt: Sized {
    fn with_iter(self) -> WithIter<Self>;
}

impl<I> WithIterExt for I
where
    I: Iterator,
{
    fn with_iter(self) -> WithIter<Self> {
        WithIter(self)
    }
}

struct WithIter<I>(I);

impl<I> Iterator for WithIter<I>
where
    I: Iterator + Clone,
{
    type Item = (<I as Iterator>::Item, I);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|value| (value, self.0.clone()))
    }
}
