use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let text = String::from("ACGTTGCATGTCGCATGATGCATGAGAGCT");
    let k = 4;
    let hash = most_frequent_kmer_hash(&text, k);
    println!("{:?}", hash);
}

fn most_frequent_kmers(text: &String, k: usize) -> Vec<&str> {
    let hash = most_frequent_kmer_hash(text, k);
    let inverted_hash = invert_hash(hash);

    return vec![&"hello"]

}

type HashInvertible = Eq + Hash;

fn invert_hash<S: HashInvertible, T: HashInvertible>(hash: HashMap<S, T>) -> HashMap<T, Vec<S>> {
    let mut inverted = HashMap::new();
    let hash = hash.iter();

    for (key, value) in hash {
        let mut new_vec: Vec<S> = match inverted.get(value) {
            Some(vec) => vec.push(&key),
            None => vec![&key],
        };
        inverted.insert(value, new_vec);
    }
    inverted
}

fn most_frequent_kmer_hash(text: &String, k: usize) -> HashMap<&str, usize> {
    let mut freq_hash = HashMap::new();
    let limit = text.len() - k;
    for start_pos in 0..=limit {
        let substring = &text[start_pos..(start_pos + k)];
        let new_count: usize = match freq_hash.get(substring) {
            Some(num) => num + 1,
            None => 1,
        };
        freq_hash.insert(substring, new_count);
    }
    freq_hash
}

fn kmer_count(text: &String, pattern: &String) -> usize {
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

    // #[test]
    // fn test_most_frequent_kmers() {
    //     struct TestCase {
    //         text: String,
    //         k: usize,
    //         most_freq: Vec<String>
    //     }
    //     let cases = [
    //         TestCase {
    //             text: String::from("ACGTTGCATGTCGCATGATGCATGAGAGCT"),
    //             k: 4,
    //             most_freq: vec![String::from("CATG"), String::from("GCAT")]
    //         }
    //     ];

    //     let cases = cases.iter();

    //     for case in cases {
    //         let most_freq_vec: Vec<&str> = case.most_freq.iter().map(|x| &x[..]).collect();
    //         assert_eq!(most_frequent_kmers(&case.text, case.k), most_freq_vec);
    //     }
    // }

    // #[test]
    // fn test_most_frequent_kmer_hash() {
    //     struct TestCase {
    //         text: String,
    //         k: usize,
    //         freq_hash: HashMap<String, usize>
    //     }
    //     let cases = [
    //         TestCase {
    //             text: String::from("ACGTTGCATGTCGCATGATGCATGAGAGCT"),
    //             k: 4,
    //             freq_hash: HashMap::new()
    //         },
    //     ];

    //     let cases = cases.iter();

    //     fn build_new_hash<T>(hash: HashMap<String,T>) -> HashMap<&str,T> {
    //         let hash = hash.iter();
    //         let mut new_hash = HashMap::new();
    //         for (key, value) in hash {
    //             new_hash.insert(&key[..], value)
    //         }
    //         new_hash
    //     }

    //     for case in cases {
    //         let hash = build_new_hash(case.freq_hash);
    //         assert_eq!(most_frequent_kmer_hash(&case.text, case.k), hash);
    //     }
    // }

    #[test]
    fn test_most_frequent_kmer_hash() {
        let text = String::from("ACGTTGCATGTCGCATGATGCATGAGAGCT");
        let k = 4;

        let hash = most_frequent_kmer_hash(&text, k);
        let gcat_count: Option<&usize> = Some(&3);
        assert_eq!(hash.get("GCAT"), gcat_count);
        let gatg_count: Option<&usize> = Some(&1);
        assert_eq!(hash.get("GATG"), gatg_count);
        assert_eq!(hash.get("ZZZZ"), None);
    }

    #[test]
    fn test_kmer_count() {
        #[derive(Debug)]
        struct TestCase {
            text: String,
            pattern: String,
            count: usize
        }
        let cases = [
            TestCase {
                text: String::from("GCGCG"),
                pattern: String::from("GCG"),
                count: 2
            },
            TestCase {
                text: String::from("ACGTACGTACG"),
                pattern: String::from("CG"),
                count: 3
            },
            TestCase {
                text: String::from("AAAGAGTGTCTGATAGCAGCTTCTGAACTGGTTACCTGCCGTGAGTAAATTAAATTTTATTGACTTAGGTCACTAAATACTTTAACCAATATAGGCATAGCGCACAGACAGATAATAATTACAGAGTACACAACATCCA"),
                pattern: String::from("AAA"),
                count: 4
            },
            TestCase {
                text: String::from("AGCGTGCCGAAATATGCCGCCAGACCTGCTGCGGTGGCCTCGCCGACTTCACGGATGCCAAGTGCATAGAGGAAGCGAGCAAAGGTGGTTTCTTTCGCTTTATCCAGCGCGTTAACCACGTTCTGTGCCGACTTT"),
                pattern: String::from("TTT"),
                count: 4
            },
            TestCase {
                text: String::from("GGACTTACTGACGTACG"),
                pattern: String::from("ACT"),
                count: 2
            }
        ];

        let cases = cases.iter();

        for case in cases {
            println!("{:?}", case);
            assert_eq!(kmer_count(&case.text, &case.pattern), case.count)
        }
    }
}