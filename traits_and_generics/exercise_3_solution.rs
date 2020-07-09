pub trait IterExt: Sized {
    fn with_iter(self) -> WithIter<Self> {
        WithIter(self)
    }
}

pub struct WithIter<I>(I);

impl<T, I> Iterator for WithIter<I>
where
    I: Iterator<Item = T> + Clone,
{
    type Item = (T, I);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|el| (el, self.0.clone()))
    }
}

impl<I: Iterator> IterExt for I {}
