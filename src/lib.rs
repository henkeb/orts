use crate::sorter::Sorter;

pub mod sorter;

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn sort_std() {
        let mut values = [2, 5, 4, 1, 3];
        StdSorter::sort(&mut values);
        assert_eq!(values, [1, 2, 3, 4, 5]);
    }
}
