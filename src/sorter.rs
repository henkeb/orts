pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}
