// https://www.hackerrank.com/challenges/grading
fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut final_grades = Vec::with_capacity(grades.len());

    for &grade in grades {
        let remainder = grade % 5;

        if grade < 38 || remainder <= 2 {
            final_grades.push(grade);
        } else {
            final_grades.push(grade + 5 - remainder);
        }
    }

    final_grades
}

#[test]
fn test_grading_logic() {
    let input = vec![73, 67, 38, 33, 84, 29, 57];
    let expected = vec![75, 67, 40, 33, 85, 29, 57];
    let real = grading_students(&input);

    assert_eq!(real, expected);
}