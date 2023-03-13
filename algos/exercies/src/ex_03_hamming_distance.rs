#[allow(dead_code)]
fn hamming_distance(str1: &str, str2: &str) -> Option<u32> {
    let distance: i32 = 0;
    if str1.len() != str2.len() {
        return None;
    }

    // for (c1, c2) in str1.chars().zip(str2.chars()) {
    //   if c1 != c2 {
    //     distance += 1;
    //   }
    // }
    // return Some(distance)

    Some(
        str1.chars()
            .zip(str2.chars())
            .map(|(a, b)| if a != b { 1 } else { 0 })
            .sum(),
    )
}
