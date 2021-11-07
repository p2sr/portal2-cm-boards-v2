use num::pow;

/// Calcultes the score using the pre-existing iVerb point formula.
pub fn score(i: i32) -> f32{
    let i = i as f32;
    let res: f32 = pow(200.0-(i-1.0), 2)/200.0;
    if 1.0 > res{
        1.0
    } else{
        res
    }
}