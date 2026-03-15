// https://www.hackerrank.com/challenges/sock-merchant

fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut color_counts = std::collections::HashMap::new();

    for &color in ar {
        *color_counts.entry(color).or_insert(0) += 1;
    }

    color_counts.values().map(|&count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sock_merchant() {
        let n = 9;
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        let expected = 3;
        assert_eq!(sock_merchant(n, &ar), expected);
    }
}