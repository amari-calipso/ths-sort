#[cfg(test)]
mod tests {
    const N_TESTS: u32 = 16;
    const MIN_LEN: u32 = 15;
    const MAX_LEN: u32 = 1 << 20;

    use num_traits::{Num, One, Zero};
    use rand::Rng;
    use std::{cmp::max, ops::{Rem, Div, Sub, Mul, Add}, num::ParseIntError};

    use crate::ths_sort::*;

    #[derive(PartialOrd, Debug)]
    struct Value {
        pub value: i32,
        pub key: usize
    }

    impl PartialEq for Value {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }

        fn ne(&self, other: &Self) -> bool {
            self.value != other.value
        }
    }

    impl Ord for Value {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl Clone for Value {
        fn clone(&self) -> Self {
            Value {
                value: self.value,
                key: self.key
            }
        }
    }

    impl Div for Value {
        type Output = Value;

        fn div(self, rhs: Self) -> Self::Output {
            Value {
                value: self.value / rhs.value,
                key: self.key
            }
        }
    }

    impl Sub for Value {
        type Output = Value;

        fn sub(self, rhs: Self) -> Self::Output {
            Value {
                value: self.value - rhs.value,
                key: self.key
            }
        }
    }

    impl Add for Value {
        type Output = Value;

        fn add(self, rhs: Self) -> Self::Output {
            Value {
                value: self.value + rhs.value,
                key: self.key
            }
        }
    }

    impl Mul for Value {
        type Output = Value;

        fn mul(self, rhs: Self) -> Self::Output {
            Value {
                value: self.value * rhs.value,
                key: self.key
            }
        }
    }

    impl Rem for Value {
        type Output = Value;

        fn rem(self, rhs: Self) -> Self::Output {
            Value {
                value: self.value % rhs.value,
                key: self.key
            }
        }
    }

    impl One for Value {
        fn one() -> Self {
            Value {
                value: 1,
                key: 0
            }
        }
    }

    impl Zero for Value {
        fn zero() -> Self {
            Value {
                value: 0,
                key: 0
            }
        }

        fn is_zero(&self) -> bool {
            self.value == 0
        }
    }

    impl Num for Value {
        type FromStrRadixErr = ParseIntError;

        fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
            let tmp = i32::from_str_radix(str, radix);
            match tmp {
                Ok(x) => Ok(Value {
                    value: x,
                    key: 0
                }),
                Err(e) => Err(e)
            }
        }
    }

    impl Eq       for Value {}
    impl Copy     for Value {}
    impl NumType  for Value {}
    impl SortType for Value {}

    fn gen_array(len: usize, unique: usize) -> Vec<Value> {
        let mut array: Vec<Value> = Vec::new();
    
        for i in 0 .. unique {
            for _ in 0 .. len / unique {
                array.push(Value {
                    value: i as i32, key: 0
                });
            }
        }
    
        for i in 0 .. array.len() {
            let r = rand::thread_rng().gen_range(i .. array.len());
            let tmp = array[i].clone();
            array[i] = array[r];
            array[r] = tmp;
        }
        
        for i in 0 .. array.len() {
            array[i].key = i;
        }
    
        return array;
    }

    fn check(array: &[Value], reference: &[Value], stable: bool) {
        assert!(array[0].eq(&reference[0]), "Index 0 does not match reference");

        for i in 1 .. array.len() {
            assert!(array[i - 1].cmp(&array[i]).is_le(), "Indices {} and {} are out of order", i - 1, i);
            assert!(array[i].eq(&reference[i]), "Index {} does not match reference", i);
            
            if stable && array[i - 1].eq(&array[i]) {
                assert!(array[i - 1].key <= array[i].key, "Indices {} and {} are unstable", i - 1, i);
            }
        }
    }

    #[test]
    fn test_sort() {
        for _ in 0 .. N_TESTS {
            let len = rand::thread_rng().gen_range(MIN_LEN .. MAX_LEN) as usize;
            let mut array = gen_array(
                len, rand::thread_rng().gen_range(max(MIN_LEN, len as u32) - 1 .. len as u32) as usize
            );

            let mut reference = array.clone();
            reference.sort();
            
            sort(&mut array, 0, len - 1);
            check(&array, &reference, false);
        }
    }

    #[test]
    fn test_stable_sort() {
        for _ in 0 .. N_TESTS {
            let len = rand::thread_rng().gen_range(MIN_LEN .. MAX_LEN) as usize;
            let mut array = gen_array(
                len, rand::thread_rng().gen_range(max(MIN_LEN, len as u32) - 1 .. len as u32) as usize
            );

            let mut reference = array.clone();
            reference.sort();
            
            stable_sort(&mut array, 0, len - 1);
            check(&array, &reference, true);
        }
    }

    #[test]
    fn test_static_sort() {
        for _ in 0 .. N_TESTS {
            let len = rand::thread_rng().gen_range(MIN_LEN .. MAX_LEN) as usize;
            let mut array = gen_array(
                len, rand::thread_rng().gen_range(max(MIN_LEN, len as u32) - 1 .. len as u32) as usize
            );

            let mut reference = array.clone();
            reference.sort();
            
            static_sort(&mut array, 0, len - 1, |x| {x.value as f32});
            check(&array, &reference, false);
        }
    }
}
