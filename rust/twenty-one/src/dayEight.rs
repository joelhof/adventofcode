use regex::Regex;

pub fn partOne(input: &str) -> u32 {
    let UNIQUE_SEG_COUNTER: Regex = Regex::new(r"(\b[a-z]{2}\b|\b[a-z]{3}\b|\b[a-z]{4}\b|\b[a-z]{7}\b)").unwrap();
    let SIGNAL_PATTERN: Regex = Regex::new(r"(.+\|)").unwrap();
    return input.lines()
        .map(|l| l.split("|").collect::<Vec<&str>>()[1])
        .map(|output_digit| UNIQUE_SEG_COUNTER.captures_iter(output_digit).count())
        .sum::<usize>() as u32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
        fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
        fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
        cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
        efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
        gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
        gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
        cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
        ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
        gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
        fgae cfgab fg bagce";
        let res = partOne(input);
        assert_eq!(26, res);
    }
}