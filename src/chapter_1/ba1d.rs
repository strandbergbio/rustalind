pub fn find_substring_indices(pattern: &str, text: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();

    if text.len() == 0 || pattern.len() == 0 || pattern.len() > text.len() {
        return indices
    }

    let limit = text.len() - pattern.len() + 1;
    for index in 0..limit {
        if &text[index..index + pattern.len()] == pattern {
            indices.push(index);
        }
    }
    indices
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_substring_indices() {
        struct TestCase<'a> {
            pattern: &'a str,
            text: &'a str,
            indices: Vec<usize>,
        }

        let cases = [
            TestCase {
                pattern: "",
                text: "GATGAT",
                indices: vec![]
            },
            TestCase {
                pattern: "GATGAT",
                text: "",
                indices: vec![]
            },
            TestCase {
                pattern: "GATC",
                text: "GA",
                indices: vec![]
            },
            TestCase {
                pattern: "ATAT",
                text: "GATATATGCATATACTT",
                indices: vec![1,3,9]
            },
            TestCase {
                pattern: "ACAC",
                text: "TTTTACACTTTTTTGTGTAAAAA",
                indices: vec![4]
            },
            TestCase {
                pattern: "AAA",
                text: "AAAGAGTGTCTGATAGCAGCTTCTGAACTGGTTACCTGCCGTGAGTAAATTAAATTTTATTGACTTAGGTCACTAAATACTTTAACCAATATAGGCATAGCGCACAGACAGATAATAATTACAGAGTACACAACATCCAT",
                indices: vec![0, 46, 51, 74]
            },
            TestCase {
                pattern: "TTT",
                text: "AGCGTGCCGAAATATGCCGCCAGACCTGCTGCGGTGGCCTCGCCGACTTCACGGATGCCAAGTGCATAGAGGAAGCGAGCAAAGGTGGTTTCTTTCGCTTTATCCAGCGCGTTAACCACGTTCTGTGCCGACTTT",
                indices: vec![88, 92, 98, 132]
            }
        ];

        for case in cases.iter() {
            assert_eq!(find_substring_indices(case.pattern, case.text), case.indices);
        }
    }
}