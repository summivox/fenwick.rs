pub mod one_based {
    #[inline]
    pub fn next_down(x: usize) -> usize {
        x & x.wrapping_sub(1)
    }
    pub fn down(init: usize) -> Down {
        Down(init)
    }
    pub struct Down(usize);
    impl Iterator for Down {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            let x = self.0;
            if x != 0 {
                self.0 = next_down(x);
                Some(x)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn next_up(x: usize) -> usize {
        (x | x.wrapping_sub(1)).wrapping_add(1)
    }
    pub fn up(init: usize, limit_inclusive: usize) -> Up {
        Up {
            curr: init,
            limit_inclusive,
        }
    }
    pub struct Up {
        curr: usize,
        limit_inclusive: usize,
    }
    impl Iterator for Up {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            let x = self.curr;
            if x <= self.limit_inclusive {
                self.curr = next_up(x);
                Some(x)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{down, up};

        #[test]
        fn down_boundary() {
            assert_eq!(down(0).collect::<Vec<usize>>(), vec![]);
            assert_eq!(down(1).collect::<Vec<usize>>(), vec![1usize]);
        }

        #[test]
        fn up_boundary() {
            assert_eq!(
                up(1usize, 0b100usize).collect::<Vec<usize>>(),
                vec![0b1usize, 0b10usize, 0b100usize]
            );
            assert_eq!(
                up(0b100usize, 0b100usize).collect::<Vec<usize>>(),
                vec![0b100usize]
            );
            assert_eq!(up(0b111usize, 0b100usize).collect::<Vec<usize>>(), vec![]);
        }
    }
}

pub mod zero_based {
    #[inline]
    pub fn next_down(x: usize) -> usize {
        (x & x.wrapping_add(1)).wrapping_sub(1)
    }
    pub fn down(init: usize) -> Down {
        Down(init)
    }
    pub struct Down(usize);
    impl Iterator for Down {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            let x = self.0;
            if x != !0 {
                self.0 = next_down(x);
                Some(x)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn next_up(x: usize) -> usize {
        x | x.wrapping_add(1)
    }
    pub fn up(init: usize, limit_exclusive: usize) -> Up {
        Up {
            curr: init,
            limit_exclusive,
        }
    }
    pub struct Up {
        curr: usize,
        limit_exclusive: usize,
    }
    impl Iterator for Up {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            let x = self.curr;
            if x < self.limit_exclusive {
                self.curr = next_up(x);
                Some(x)
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{down, up};

        #[test]
        fn down_boundary() {
            assert_eq!(down(!0).collect::<Vec<usize>>(), vec![]);
            assert_eq!(down(0).collect::<Vec<usize>>(), vec![0usize]);
        }

        #[test]
        fn up_boundary() {
            assert_eq!(
                up(0usize, 0b1111usize).collect::<Vec<usize>>(),
                vec![0b0usize, 0b1usize, 0b11usize, 0b111usize]
            );
            assert_eq!(
                up(0b100usize, 0b101usize).collect::<Vec<usize>>(),
                vec![0b100usize]
            );
            assert_eq!(up(0b100usize, 0b100usize).collect::<Vec<usize>>(), vec![]);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn down_example() {
        let init_one = 0b1101110101011010000usize;
        let ans_one = vec![
            0b1101110101011010000usize,
            0b1101110101011000000usize,
            0b1101110101010000000usize,
            0b1101110101000000000usize,
            0b1101110100000000000usize,
            0b1101110000000000000usize,
            0b1101100000000000000usize,
            0b1101000000000000000usize,
            0b1100000000000000000usize,
            0b1000000000000000000usize,
        ];

        assert_eq!(
            super::one_based::down(init_one).collect::<Vec<usize>>(),
            ans_one
        );
        assert_eq!(
            super::zero_based::down(init_one - 1)
                .map(|x| x + 1)
                .collect::<Vec<usize>>(),
            ans_one
        );
    }

    #[test]
    fn up_example() {
        let init_one = 0b1101110101011010000usize;
        let limit = 0b100000000000000000000usize;
        let ans_one = vec![
            0b001101110101011010000usize,
            0b001101110101011100000usize,
            0b001101110101100000000usize,
            0b001101110110000000000usize,
            0b001101111000000000000usize,
            0b001110000000000000000usize,
            0b010000000000000000000usize,
            0b100000000000000000000usize,
        ];
        assert_eq!(
            super::one_based::up(init_one, limit).collect::<Vec<usize>>(),
            ans_one
        );
        assert_eq!(
            super::zero_based::up(init_one - 1, limit)
                .map(|x| x + 1)
                .collect::<Vec<usize>>(),
            ans_one
        );
    }
}
