// https://www.hackerrank.com/challenges/staircase/problem
fn staircase(n: i32) {
    print!("{}", build_staircase(n));
}

fn build_staircase(n: i32) -> String {
    let mut result = String::new();
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        result.push_str(&format!("{}{}\n", spaces, hashes));
    }
    result
}


#[test]
fn test_staircase_logic() {
    let n = 3;
    let expected = "  #\n ##\n###\n";
    assert_eq!(build_staircase(n), expected);
}

#[test]
fn test_staircase_execution() {
    staircase(5);
}