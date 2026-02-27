// https://www.hackerrank.com/challenges/apple-and-orange/problem
fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count() as i32;
    let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count() as i32;

    println!("{}", apple_count);
    println!("{}", orange_count);

    (apple_count, orange_count)
}

#[test]
fn test_apple_orange_logic() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    let result = count_apples_and_oranges(s, t, a, b, &apples, &oranges);
    assert_eq!(result, (1, 1));
}