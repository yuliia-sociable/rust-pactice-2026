// https://www.hackerrank.com/challenges/kangaroo/problem
fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 <= v2 {
        return "NO";
    }

    let distance_diff = x2 - x1;
    let velocity_diff = v1 - v2;

    if distance_diff % velocity_diff == 0 {
        "YES"
    } else {
        "NO"
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_meets() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_slower_behind() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_kangaroo_same_speed() {
        assert_eq!(kangaroo(0, 2, 5, 2), "NO");
    }

    #[test]
    fn test_kangaroo_overshoots() {
        assert_eq!(kangaroo(0, 4, 5, 1), "NO");
    }
}