// https://www.hackerrank.com/challenges/between-two-sets
fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().fold(1, |acc, &x| acc * x / gcd(acc, x));
    let gcd_b = b.iter().fold(b[0], |acc, &x| gcd(acc, x));

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_between_sets() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }
}