#![allow(dead_code)]

pub fn get_factors(v: i64) -> Vec<i64> {
    let mut result = Vec::new();
    if v < 2 {
        return result;
    }

    let mut v = v;
    let mut f = |v: &mut i64, x: i64| {
        while *v % x == 0 {
            *v /= x;
            result.push(x);
        }
    };

    f(&mut v, 2);
    let mut i = 3;
    while i * i <= v {
        f(&mut v, i);
        i += 2;
    }

    if v > 1 {
        result.push(v);
    }

    result
}

pub fn is_prime(v: i64) -> bool {
    if v == 2 || v == 3 {
        return true;
    }

    if v < 2 || v % 2 == 0 || v % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= v {
        if v % i == 0 || v % (i + 2) == 0 {
            return false;
        }

        i += 6;
    }

    true
}

pub fn sieve(v: i64) -> Vec<i64> {
    let mut result = Vec::new();
    if v < 2 {
        return result;
    }

    result.push(2);
    let mut sieve = vec![true; ((v + 1) / 2) as usize];
    for i in 1..sieve.len() {
        if sieve[i] {
            result.push(i as i64 * 2 + 1);
            let mut j = i;
            while j < sieve.len() {
                sieve[j] = false;
                j += i * 2 + 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_factors_test() {
        assert_eq!(get_factors(0), []);
        assert_eq!(get_factors(1), []);
        assert_eq!(get_factors(2), [2]);
        assert_eq!(get_factors(3), [3]);
        assert_eq!(get_factors(4), [2, 2]);
        assert_eq!(get_factors(5), [5]);
        assert_eq!(get_factors(6), [2, 3]);
        assert_eq!(get_factors(7), [7]);
        assert_eq!(get_factors(8), [2, 2, 2]);
        assert_eq!(get_factors(9), [3, 3]);
    }

    #[test]
    fn is_prime_test() {
        fn naive(v: i64) -> bool {
            if v == 2 {
                return true;
            }

            if v < 2 || v % 2 == 0 {
                return false;
            }

            let mut i = 3;
            while i * i <= v {
                if v % i == 0 {
                    return false;
                }

                i += 2;
            }

            true
        }

        for i in 0..10000 {
            assert_eq!(is_prime(i), naive(i));
        }
    }

    #[test]
    fn sieve_test() {
        assert_eq!(sieve(0), []);
        assert_eq!(sieve(1), []);
        assert_eq!(sieve(2), [2]);
        assert_eq!(sieve(3), [2, 3]);
        assert_eq!(sieve(4), [2, 3]);
        assert_eq!(sieve(5), [2, 3, 5]);
        assert_eq!(sieve(6), [2, 3, 5]);
        assert_eq!(sieve(7), [2, 3, 5, 7]);
        assert_eq!(sieve(8), [2, 3, 5, 7]);
        assert_eq!(sieve(9), [2, 3, 5, 7]);
    }
}
