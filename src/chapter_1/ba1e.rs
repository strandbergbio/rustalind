use std::collections::HashMap;
use std::collections::VecDeque;

pub fn find_pattern_clumps(genome: &str, kmer_size: usize, window_length: usize, min_occurrences: usize) -> Vec<&str> {
    if kmer_size == 0 || kmer_size > window_length || kmer_size > genome.len() || min_occurrences > genome.len() - kmer_size {
        return vec![]
    }

    let limit = genome.len() - kmer_size + 1;
    let mut kmer_hash: HashMap<&str, Vec<usize>> = HashMap::new();

    for index in 0..limit {
        let kmer = &genome[index..index+kmer_size];
        let mut vec = match kmer_hash.get(&kmer) {
            Some(vec) => vec.clone(),
            None => Vec::new(),
        };
        vec.push(index.clone());
        kmer_hash.insert(&kmer, vec);
    }

    let mut kmers = Vec::new();
    for (key, value) in kmer_hash.iter() {
        if has_clump(&value, kmer_size, window_length, min_occurrences) {
            kmers.push(key.clone());
        }
    }
    kmers
}

fn has_clump(position_vector: &Vec<usize>, kmer_size: usize, window_length: usize, min_occurrences: usize) -> bool {
    let mut queue: VecDeque<&usize> = VecDeque::new();

    for position in position_vector.iter() {
        if let Some(front) = queue.pop_front() {
            if ((position + kmer_size) - front) <= window_length {
                queue.push_front(front);
            };
        };
        queue.push_back(position);
        if queue.len() >= min_occurrences {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_clumps() {
        struct TestCase<'a> {
            genome: &'a str,
            kmer_size: usize,
            window_length: usize,
            min_occurrences: usize,
            kmers: Vec<&'a str>
        }

        let cases = [
            TestCase {
                genome: "GAT",
                kmer_size: 5,
                window_length: 5,
                min_occurrences: 5,
                kmers: vec![]
            },
            TestCase {
                genome: "CGGACTCGACAGATGTGAAGAAATGTGAAGACTGAGTGAAGAGAAGAGGAAACACGACACGACATTGCGACATAATGTACGAATGTAATGTGCCTATGGC",
                kmer_size: 5,
                window_length: 75,
                min_occurrences: 4,
                kmers: vec!["AATGT", "CGACA", "GAAGA"]
            },
            TestCase {
                genome: "CGGACTCGACAGATGTGAAGAACGACAATGTGAAGACTCGACACGACAGAGTGAAGAGAAGAGGAAACATTGTAA",
                kmer_size: 5,
                window_length: 50,
                min_occurrences: 4,
                kmers: vec!["CGACA", "GAAGA"]
            },
            TestCase {
                genome: "AAAACGTCGAAAAA",
                kmer_size: 2,
                window_length: 4,
                min_occurrences: 2,
                kmers: vec!["AA"]
            },
            TestCase {
                genome: "ACGTACGT",
                kmer_size: 1,
                window_length: 5,
                min_occurrences: 2,
                kmers: vec!["A", "C", "G", "T"]
            },
            TestCase {
                genome: "CCACGCGGTGTACGCTGCAAAAAGCCTTGCTGAATCAAATAAGGTTCCAGCACATCCTCAATGGTTTCACGTTCTTCGCCAATGGCTGCCGCCAGGTTATCCAGACCTACAGGTCCACCAAAGAACTTATCGATTACCGCCAGCAACAATTTGCGGTCCATATAATCGAAACCTTCAGCATCGACATTCAACATATCCAGCG",
                kmer_size: 3,
                window_length: 25,
                min_occurrences: 3,
                kmers: vec!["AAA", "CAG", "CAT", "CCA", "GCC", "TTC"]
            }
        ];

        for case in cases.iter() {
            let mut pattern_clumps = find_pattern_clumps(case.genome, case.kmer_size, case.window_length, case.min_occurrences);
            pattern_clumps.sort_unstable();
            assert_eq!(pattern_clumps, case.kmers);
        }
    }
}
