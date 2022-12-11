//! An extension to the Rust std crate with common, general utility that is
//! often useful but not included in the std cratue.

pub trait IterExt {
    type Item;
    fn max_n<const N: usize>(self) -> [Option<Self::Item>; N] where Self: Sized, Self::Item: Ord + Copy;
}


pub fn max_n<const N: usize, I: IntoIterator<Item = T>, T: Ord + Copy>(iter: I) -> [Option<T>; N] {
    if N == 0 { return std::array::from_fn(|_| unreachable!()) }
    let mut max = std::array::from_fn(|_| None);
    'items: for item in iter {
        for i in 0..N {
            match &max[i] {
                Some(curr) => match item.cmp(curr) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        if i != 0 {
                            max[i-1] = Some(item);
                        }
                        continue 'items;
                    }
                    std::cmp::Ordering::Greater => if i > 0 {
                        max[i-1] = max[i];
                    }
                }
                None => {}
            }
        }
        if N != 0 {
            max[N-1] = Some(item);
        }
    }
    max
}

impl<I: IntoIterator<Item = T>, T> IterExt for I {
    type Item = T;

    fn max_n<const N: usize>(self) -> [Option<T>; N] where T: Ord + Copy {
        max_n(self.into_iter())
    }
}

#[test]
fn max_n_test() {
    assert_eq!([1, 2, 3].max_n(), [Some(3)]);
    assert_eq!([3, 2, 1].max_n(), [Some(3)]);
    assert_eq!([1, 3, 2].max_n(), [Some(3)]);

    assert_eq!([1, 3, 2].max_n(), [Some(2), Some(3)]);
    assert_eq!([2, 1].max_n(), [None, Some(1), Some(2)]);
}
