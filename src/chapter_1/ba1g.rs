pub fn hamming_distance(string1: &str, string2: &str) -> usize {
    let mut distance = 0;
    for (c1, c2) in string1.chars().zip(string2.chars()) {
        if c1 != c2 {
            distance += 1
        }
    };
    distance
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        struct TestCase<'a> {
            string1: &'a str,
            string2: &'a str,
            distance: usize
        }
        let cases = [
            TestCase {
                string1: "aa",
                string2: "aa",
                distance: 0
            },
            TestCase {
                string1: "abcd",
                string2: "acce",
                distance: 2
            }
        ];

        for case in cases.iter() {
            assert_eq!(hamming_distance(case.string1, case.string2), case.distance);
        }
    }
}