// https://www.hackerrank.com/challenges/migratory-birds

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];

    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut min_type = 0;

    for (i, &count) in counts.iter().enumerate().skip(1) {
        if count > max_count {
            max_count = count;
            min_type = i as i32;
        }
    }

    min_type
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_migratory_birds_1() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        let expected = 4;
        assert_eq!(migratory_birds(&arr), expected);
    }
    #[test]
    fn test_migratory_birds_2() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        let expected = 3;
        assert_eq!(migratory_birds(&arr), expected);
    }
}