
pub mod indexed {

    pub struct Indexed<A, T> {
        iter: T,
        idx: uint
    }

    impl<A, T: Iterator<A>> Iterator<(uint, A)> for Indexed<A, T> {
        #[inline]
        fn next(&mut self) -> Option<(uint, A)> {
            match self.iter.next() {
                Some(v) => {
                    let result = (self.idx, v);
                    self.idx += 1;
                    Some(result)
                },
                None => None
            }
        }
    }

    pub trait ToIndexed<A, T> {
        fn indexed(self) -> Indexed<A, T>;
    }

    impl <A, T: Iterator<A>> ToIndexed<A, T> for T {
        fn indexed(self) -> Indexed<A, T> {
            Indexed { iter: self, idx: 0 }
        }
    }
}