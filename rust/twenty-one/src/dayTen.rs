use std::str::FromStr;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug)]
struct MismatchedChunk {
    expected: char,
    actual: char
}

impl MismatchedChunk {
    fn to_string(&self) -> String {
        return format!("Expected '{}' but found '{}'", self.expected, self.actual);
    }
}

#[derive(Debug)]
struct NavigationChunk {
    chunks: Vec<char>,
    error: Option<MismatchedChunk>
}

impl NavigationChunk {
    fn closing(opening: &char) -> Option<char> {
        return match opening {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None
        }
    }

    fn complete(&self) -> Self {
        let mut stack: Vec<&char> = Vec::new();
        let opening: HashSet<&char> = HashSet::from_iter(['(', '[', '{', '<'].iter());
        let closing: HashSet<&char> = HashSet::from_iter([')', ']', '}', '>'].iter());
        
        for c in self.chunks.iter() {
            if opening.contains(&c) {
                stack.push(c);
            }
            if closing.contains(&c) {
                stack.pop();
            }
        }
        let completions = stack.iter()
            .rev()
            .filter_map(|c| NavigationChunk::closing(c))
            .collect();
        //println!("autocompletions {:?}", completions);
        return NavigationChunk { chunks: completions,
                error: None
            };
    }
}

impl FromStr for NavigationChunk {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<char> = Vec::new();
        let opening: HashSet<&char> = HashSet::from_iter(['(', '[', '{', '<'].iter());
        let closing: HashSet<&char> = HashSet::from_iter([')', ']', '}', '>'].iter());
        let mut error = None;
        for c in input.chars() {
            if opening.contains(&c) {
                stack.push(c);
            }
            if closing.contains(&c) {
                let opening = stack.pop();
                let closingToken = match opening {
                    Some(opening) => NavigationChunk::closing(&opening),
                    None => None
                };
                error = match closingToken {
                    Some(x) if x == c => None,
                    Some(x) => Some(MismatchedChunk { expected: x, actual: c }),
                    None => None 
                };
                if error.is_some() {
                    break;
                }
            }
        }

        let chars: Vec<char> = input.chars().collect();
        return Ok(NavigationChunk { chunks: chars, error: error });
    }
}

struct SyntaxChecker {
    corrupt: Vec<NavigationChunk>
}

impl SyntaxChecker {
    fn error_score(character: char) -> Option<u32> {
        return match character {
            ')' => Some(3),
            ']' => Some(57),
            '}' => Some(1197),
            '>' => Some(25137),
            _ => None
        }
    }
}

impl FromStr for SyntaxChecker {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let corrupt: Vec<NavigationChunk> = input.lines()
            .map(|l| l.trim())
            .map(|l| l.parse::<NavigationChunk>().unwrap())
            .filter(|chunk| chunk.error.is_some())
            .collect();
        
        return Ok(SyntaxChecker { corrupt: corrupt })
    }
}

pub fn partOne(input: &str) -> u32 {
    let checker: SyntaxChecker = input.parse().unwrap();
    return checker.corrupt.iter()
        .map(|chunk| match &chunk.error { Some(e) => e.actual, None => 'n' })
        .map(|mismatch| SyntaxChecker::error_score(mismatch))
        .map(|score| match score { Some(x) => x, None => 0 })
        .sum();
}

struct Autocomplete {
    chunks: Vec<NavigationChunk>,
    completions: Vec<NavigationChunk>
}

impl FromStr for Autocomplete {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let uncompleted: Vec<NavigationChunk> = input.lines()
            .map(|l| l.trim())
            .map(|l| l.parse::<NavigationChunk>().unwrap())
            .filter(|chunk| chunk.error.is_none())
            .collect();
        return Ok(Autocomplete { chunks: uncompleted, completions: Vec::new() });
    }
}

impl Autocomplete {

    fn completions(&mut self) {
        self.completions = self.chunks.iter()
            .map(|uncompleted_chunk| uncompleted_chunk.complete())
            .collect();
    }

    fn char_score(character: &char) -> u32 {
        return match character {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        };
    }

    fn score(&self) -> Vec<u64> {
        return self.completions.iter()
            .map(|chunk| chunk.chunks.iter()
                .map(|c| Autocomplete::char_score(c))
                .map(|score| score as u64)
                .fold(0, |acc, score| return 5 * acc + score)
            ).collect();
    }
}

pub fn partTwo(input: &str) -> u64 {
    let mut autocomplete: Autocomplete = input.parse().unwrap();
    autocomplete.completions();
    let mut scores = autocomplete.score();
    scores.sort();
    return scores[(autocomplete.score().len() + (2 - 1)) / 2 - 1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partOneExample() {
        let input = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        let res = partOne(input);
        assert_eq!(26397, res);
    }

    #[test]
    fn partTwoExample() {
        let input = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        let res = partTwo(input);
        assert_eq!(288957, res);
    }
}