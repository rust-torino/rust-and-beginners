#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn run_all_funcs() {
        let value = Cell::new(0);
        let funcs: Vec<Box<dyn FnMut()>> = vec![
            Box::new(|| {
                value.replace(value.get() + 2);
            }),
            Box::new(|| {
                value.replace(value.get() * 3);
            }),
            Box::new(|| {
                value.replace(value.get() - 1);
            }),
            Box::new(|| {
                value.replace(value.get() * 2);
            }),
            Box::new(|| {
                value.replace(value.get() + 1);
            }),
        ];
        run_all(funcs.into_iter());
        assert_eq!(value.get(), 11);
    }
}
