use super::ba1g;

pub fn approximate_match_positions(pattern: &str, genome: &str, max_hamming_distance: usize) -> Vec<usize> {
    let mut positions = Vec::new();
    let limit = genome.len() - pattern.len() + 1;

    for pos in 0..limit {
        let slice = &genome[pos..pos + pattern.len()];
        if ba1g::hamming_distance(pattern, slice) <= max_hamming_distance {
            positions.push(pos)
        }
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approximate_match_positions() {
        struct TestCase<'a> {
            pattern: &'a str,
            genome: &'a str,
            max_hamming_distance: usize,
            positions: Vec<usize>
        }
        let cases = [
            TestCase {
                pattern: "abc",
                genome: "bbabd",
                max_hamming_distance: 1,
                positions: vec![2]
            }
        ];

        for case in cases.iter() {
            assert_eq!(
                approximate_match_positions(
                    case.pattern, 
                    case.genome, 
                    case.max_hamming_distance
                ), 
                case.positions
            );
        }
    }
}