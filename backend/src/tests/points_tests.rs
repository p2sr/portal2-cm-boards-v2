#[cfg(test)]
#[test]
fn test_score_calc() {
    use crate::points::score_calc;

    let score = 329275; // Exaple of Zypeh overall aggregate time.
    let (ms, seconds, minutes) = score_calc(score);
    assert_eq!(ms, 750);
    assert_eq!(seconds, 52);
    assert_eq!(minutes, 54);
    println!("{} -> {}:{}.{}", score, minutes, seconds, ms);
}
