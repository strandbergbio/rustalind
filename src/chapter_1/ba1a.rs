pub fn kmer_count(text: &String, pattern: &String) -> usize {
    let mut count = 0;
    let limit = text.len() - pattern.len();
    for start_pos in 0..=limit {
        let substring = &text[start_pos..(start_pos + pattern.len())];
        if substring == &pattern[..] {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmer_count() {
        #[derive(Debug)]
        struct TestCase<'a> {
            text: &'a String,
            pattern: &'a String,
            count: usize
        }
        let cases = [
            TestCase {
                text: &String::from("GCGCG"),
                pattern: &String::from("GCG"),
                count: 2
            },
            TestCase {
                text: &String::from("ACGTACGTACG"),
                pattern: &String::from("CG"),
                count: 3
            },
            TestCase {
                text: &String::from("AAAGAGTGTCTGATAGCAGCTTCTGAACTGGTTACCTGCCGTGAGTAAATTAAATTTTATTGACTTAGGTCACTAAATACTTTAACCAATATAGGCATAGCGCACAGACAGATAATAATTACAGAGTACACAACATCCA"),
                pattern: &String::from("AAA"),
                count: 4
            },
            TestCase {
                text: &String::from("AGCGTGCCGAAATATGCCGCCAGACCTGCTGCGGTGGCCTCGCCGACTTCACGGATGCCAAGTGCATAGAGGAAGCGAGCAAAGGTGGTTTCTTTCGCTTTATCCAGCGCGTTAACCACGTTCTGTGCCGACTTT"),
                pattern: &String::from("TTT"),
                count: 4
            },
            TestCase {
                text: &String::from("GGACTTACTGACGTACG"),
                pattern: &String::from("ACT"),
                count: 2
            }
        ];

        for case in cases.iter() {
            println!("{:?}", case);
            assert_eq!(kmer_count(case.text, case.pattern), case.count)
        }
    }
}
