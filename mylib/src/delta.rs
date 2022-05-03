use std::convert::TryFrom;

struct DeltaIndex<'a> {
    limit: (usize, usize),
    delta: &'a [(i32, i32)],
}

impl<'a> DeltaIndex<'a> {
    fn new(limit: (usize, usize), delta: &'a [(i32, i32)]) -> Self {
        Self { limit, delta }
    }

    fn generate(&self, origin: (usize, usize)) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            while i < self.delta.len() {
                let dest = (
                    usize::try_from(origin.0 as i32 + self.delta[i].0).unwrap_or(self.limit.0),
                    usize::try_from(origin.1 as i32 + self.delta[i].1).unwrap_or(self.limit.1),
                );
                i += 1;
                if dest.0 < self.limit.0 && dest.1 < self.limit.1 {
                    return Some(dest);
                }
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_test() {
        let d4 = DeltaIndex::new((3, 3), &[(0, -1), (-1, 0), (0, 1), (1, 0)]);
        assert_eq!(d4.generate((0, 0)).collect::<Vec<_>>(), [(0, 1), (1, 0)]);
        assert_eq!(d4.generate((0, 2)).collect::<Vec<_>>(), [(0, 1), (1, 2)]);
        assert_eq!(d4.generate((2, 0)).collect::<Vec<_>>(), [(1, 0), (2, 1)]);
        assert_eq!(d4.generate((2, 2)).collect::<Vec<_>>(), [(2, 1), (1, 2)]);
        assert_eq!(
            d4.generate((0, 1)).collect::<Vec<_>>(),
            [(0, 0), (0, 2), (1, 1)]
        );
        assert_eq!(
            d4.generate((1, 0)).collect::<Vec<_>>(),
            [(0, 0), (1, 1), (2, 0)]
        );
        assert_eq!(
            d4.generate((1, 2)).collect::<Vec<_>>(),
            [(1, 1), (0, 2), (2, 2)]
        );
        assert_eq!(
            d4.generate((2, 1)).collect::<Vec<_>>(),
            [(2, 0), (1, 1), (2, 2)]
        );
        assert_eq!(
            d4.generate((1, 1)).collect::<Vec<_>>(),
            [(1, 0), (0, 1), (1, 2), (2, 1)]
        );
    }
}
