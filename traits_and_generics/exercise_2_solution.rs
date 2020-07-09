pub fn run_all<'a, I>(funcs: I)
where
    I: Iterator<Item = Box<dyn FnMut() + 'a>>,
{
    funcs.for_each(|mut f| f());
}
