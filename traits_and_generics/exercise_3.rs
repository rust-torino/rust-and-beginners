#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_iter() {
        let v = vec![0, 1, 2, 3, 4];
        let mut iter = v.into_iter().with_iter();

        let (x, sub_iter) = iter.next().unwrap();
        assert_eq!(x, 0);
        assert_eq!(sub_iter.collect::<Vec<_>>(), vec![1, 2, 3, 4]);

        let (x, sub_iter) = iter.next().unwrap();
        assert_eq!(x, 1);
        assert_eq!(sub_iter.collect::<Vec<_>>(), vec![2, 3, 4]);

        let (x, sub_iter) = iter.next().unwrap();
        assert_eq!(x, 2);
        assert_eq!(sub_iter.collect::<Vec<_>>(), vec![3, 4]);

        let (x, sub_iter) = iter.next().unwrap();
        assert_eq!(x, 3);
        assert_eq!(sub_iter.collect::<Vec<_>>(), vec![4]);

        let (x, mut sub_iter) = iter.next().unwrap();
        assert_eq!(x, 4);
        assert!(sub_iter.next().is_none());

        assert!(iter.next().is_none());
    }
}
