pub fn reverse_complement(text: &String) -> String {
    let mut reverse = String::new();
    for char in text.chars().rev() {
        let c = complement_map(&char);
        reverse.push(*c);
    }
    reverse
}

fn complement_map(char: &char) -> &char {
    match char {
        'A' => &'T',
        'T' => &'A',
        'C' => &'G',
        'G' => &'C',
        _ => panic!("character not in DNA alphabet!")
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_reverse_complement() {
        #[derive(Debug)]
        struct TestCase<'a> {
            text: &'a String,
            reversed_complement: String
        }

        let cases = [
            TestCase {
                text: &String::from("AAAACCCGGT"),
                reversed_complement: String::from("ACCGGGTTTT")
            },
            TestCase {
                text: &String::from("ACACAC"),
                reversed_complement: String::from("GTGTGT")
            }
        ];

        for case in cases.iter() {
            let complement = reverse_complement(case.text);
            assert_eq!(complement, case.reversed_complement);
        }
    }

    #[test]
    #[should_panic]
    fn test_reverse_complement_panic() {
        let text = String::from("not a DNA sequence");
        reverse_complement(&text);
    }
}