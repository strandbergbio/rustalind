pub fn find_min_skew(genome: &str) -> Vec<usize> {
    let mut skew: isize = 0;
    let mut min_skew: isize = 0;
    let mut min_positions = vec![0];

    for (position, base) in genome.chars().enumerate() {
        match base {
            'G' => { skew += 1; },
            'C' => { skew -= 1; },
            _ => {}
        };
        if skew == min_skew {
            min_positions.push(position + 1);
        } else if skew < min_skew {
            min_skew = skew;
            min_positions = vec![position + 1];
        };
    };
    min_positions
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_min_skew() {
        struct TestCase<'a> {
            genome: &'a str,
            positions: Vec<usize>
        }

        let cases = [
            TestCase {
                genome: "",
                positions: vec![0]
            },
            TestCase {
                genome: "CCCCC",
                positions: vec![5]
            },
            TestCase {
                genome: "CCTATCGGTGGATTAGCATGTCCCTGTACGTTTCGCCGCGAACTAGTTCACACGGCTTGATGGCAAATGGTTTTTCCGGCGACCGTAATCGTCCACCGAG",
                positions: vec![53, 97]
            }
        ];

        for case in cases.iter() {
            assert_eq!(find_min_skew(case.genome), case.positions);
        }
    }
}