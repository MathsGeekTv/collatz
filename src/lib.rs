use std::collections::HashSet;

pub fn collatz(n: u64) -> Option<Vec<u64>> {
    if n == 0 {
        return None;
    }
    let mut seq = vec![n];
    let mut current = n;
    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
        seq.push(current);
    }
    Some(seq)
}

pub fn collatz_count_original(n: u64) -> Option<usize> {
    if n == 0 {
        return None;
    }
    collatz(n).map(|seq| seq.len())
}

pub fn collatz_count_efficient(n: u64) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let mut count = 1;
    let mut current = n;
    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }
        count += 1;
    }
    Some(count)
}

pub fn collatz_up_to_n(n: u64) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }
    let mut collected_numbers: HashSet<u64> = HashSet::new();
    let mut required: HashSet<u64> = (1..=n).collect();

    for i in 1..=n {
        if let Some(sequence) = collatz(i) {
            for &val in &sequence {
                collected_numbers.insert(val);
                required.remove(&val);
            }
        }
        if required.is_empty() {
            break;
        }
    }

    let mut result_vec: Vec<u64> = collected_numbers.into_iter().collect();
    result_vec.sort_unstable();
    result_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_1() {
        assert_eq!(collatz(1), Some(vec![1]));
    }

    #[test]
    fn test_collatz_3() {
        assert_eq!(collatz(3), Some(vec![3, 10, 5, 16, 8, 4, 2, 1]));
    }

    #[test]
    fn test_collatz_10() {
        assert_eq!(collatz(10), Some(vec![10, 5, 16, 8, 4, 2, 1]));
    }

    #[test]
    fn test_collatz_0() {
        assert_eq!(collatz(0), None);
    }
}

#[cfg(test)]
mod tests_collatz_count {
    use super::*;

    #[test]
    fn test_collatz_count_original_1() {
        assert_eq!(collatz_count_original(1), Some(1));
    }

    #[test]
    fn test_collatz_count_original_3() {
        assert_eq!(collatz_count_original(3), Some(8));
    }

    #[test]
    fn test_collatz_count_original_0() {
        assert_eq!(collatz_count_original(0), None);
    }

    #[test]
    fn test_collatz_count_efficient_1() {
        assert_eq!(collatz_count_efficient(1), Some(1));
    }

    #[test]
    fn test_collatz_count_efficient_3() {
        assert_eq!(collatz_count_efficient(3), Some(8));
    }

    #[test]
    fn test_collatz_count_efficient_0() {
        assert_eq!(collatz_count_efficient(0), None);
    }
}

#[cfg(test)]
mod tests_collatz_up_to_n {
    use super::*;

    #[test]
    fn test_collatz_up_to_n_0() {
        assert_eq!(collatz_up_to_n(0), vec![]);
    }

    #[test]
    fn test_collatz_up_to_n_1() {
        assert_eq!(collatz_up_to_n(1), vec![1]);
    }

    #[test]
    fn test_collatz_up_to_n_2() {
        assert_eq!(collatz_up_to_n(2), vec![1, 2]);
    }

    #[test]
    fn test_collatz_up_to_n_3() {
        assert_eq!(collatz_up_to_n(3), vec![1, 2, 3, 4, 5, 8, 10, 16]);
    }

    #[test]
    fn test_collatz_up_to_n_4() {
        assert_eq!(collatz_up_to_n(4), vec![1, 2, 3, 4, 5, 8, 10, 16]);
    }
}
