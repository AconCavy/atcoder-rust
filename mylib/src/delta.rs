use std::convert::TryFrom;
use std::rc::Rc;

pub struct DeltaIndex {
    limit: (usize, usize),
    delta: Rc<Vec<(i32, i32)>>,
}

impl DeltaIndex {
    pub fn new(limit: (usize, usize), delta: Rc<Vec<(i32, i32)>>) -> Self {
        Self { limit, delta }
    }

    pub fn generate(&self, origin: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let mut i = 0;
        let delta = Rc::clone(&self.delta);
        let limit = self.limit;
        std::iter::from_fn(move || {
            while i < delta.len() {
                let nh = usize::try_from(origin.0 as i32 + delta[i].0).unwrap_or(limit.0);
                let nw = usize::try_from(origin.1 as i32 + delta[i].1).unwrap_or(limit.1);
                i += 1;
                if nh < limit.0 && nw < limit.1 {
                    return Some((nh, nw));
                }
            }

            None
        })
    }
}

#[test]
fn delta_test() {
    let h = 3;
    let w = 3;
    let d4 = DeltaIndex::new((h, w), Rc::new(vec![(0, -1), (-1, 0), (0, 1), (1, 0)]));
    assert_eq!(d4.generate((0, 0)).collect::<Vec<_>>(), [(0, 1), (1, 0)]);
    assert_eq!(
        d4.generate((0, 1)).collect::<Vec<_>>(),
        [(0, 0), (0, 2), (1, 1)]
    );
    assert_eq!(d4.generate((0, 2)).collect::<Vec<_>>(), [(0, 1), (1, 2)]);
    assert_eq!(
        d4.generate((1, 0)).collect::<Vec<_>>(),
        [(0, 0), (1, 1), (2, 0)]
    );
    assert_eq!(
        d4.generate((1, 1)).collect::<Vec<_>>(),
        [(1, 0), (0, 1), (1, 2), (2, 1)]
    );
    assert_eq!(
        d4.generate((1, 2)).collect::<Vec<_>>(),
        [(1, 1), (0, 2), (2, 2)]
    );
    assert_eq!(d4.generate((2, 0)).collect::<Vec<_>>(), [(1, 0), (2, 1)]);
    assert_eq!(
        d4.generate((2, 1)).collect::<Vec<_>>(),
        [(2, 0), (1, 1), (2, 2)]
    );
    assert_eq!(d4.generate((2, 2)).collect::<Vec<_>>(), [(2, 1), (1, 2)]);
}
