use advent_of_code_2021::file_to_vec;
use std::iter::FromIterator;
use std::collections::HashSet;


trait Sorted {
    fn sorted(&self) -> String;
}

impl Sorted for str {
    fn sorted(&self) -> String {
        let mut chars: Vec<char> = self.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        String::from_iter(chars)
    }
}

#[derive(Debug)]
struct SevenSegDisplay {
    zero_pattern: Option<String>,
    one_pattern: Option<String>,
    two_pattern: Option<String>,
    three_pattern: Option<String>,
    four_pattern: Option<String>, 
    five_pattern: Option<String>, 
    six_pattern: Option<String>, 
    seven_pattern: Option<String>,
    eight_pattern: Option<String>,
    nine_pattern: Option<String>
}

impl SevenSegDisplay {
    fn new(patterns: &[&str]) -> Self {
        let mut out = SevenSegDisplay {
            zero_pattern: None,
            one_pattern: None,
            two_pattern: None,
            three_pattern: None,
            four_pattern: None, 
            five_pattern: None, 
            six_pattern: None, 
            seven_pattern: None,
            eight_pattern: None,
            nine_pattern: None
        };
        out.solve_display(patterns);
        out
    }

    fn get_readout(&self, digits: Vec<&str>) -> usize {
        let mut readout = 0;
        for d in digits {
            readout *= 10;
            readout += self.get_number(d);
        }
        readout
    }

    fn get_number(&self, pattern: &str) -> usize {
        let pattern = pattern.sorted();
        if *self.zero_pattern.as_ref().unwrap() == pattern {
            0
        } else if *self.one_pattern.as_ref().unwrap() == pattern {
            1
        } else if *self.two_pattern.as_ref().unwrap() == pattern {
            2
        } else if *self.three_pattern.as_ref().unwrap() == pattern {
            3
        } else if *self.four_pattern.as_ref().unwrap() == pattern {
            4
        } else if *self.five_pattern.as_ref().unwrap() == pattern {
            5
        } else if *self.six_pattern.as_ref().unwrap() == pattern {
            6
        } else if *self.seven_pattern.as_ref().unwrap() == pattern {
            7
        } else if *self.eight_pattern.as_ref().unwrap() == pattern {
            8
        } else if *self.nine_pattern.as_ref().unwrap() == pattern {
            9
        } else {
            panic!("No match for pattern");
        }
    }

    fn solve_display(&mut self, patterns: &[&str]) {
        self.find_one(patterns);
        self.find_seven(patterns);
        self.find_four(patterns);
        self.find_eight(patterns);
        self.find_six(patterns);
        self.find_zero_two_three_five_nine(patterns);
    }

    fn find_one(&mut self, patterns: &[&str]) {
        let one_pattern: &str = patterns.iter().filter(|s| s.len() == 2).next().unwrap();
        self.one_pattern = Some(one_pattern.sorted());
    }

    fn find_seven(&mut self, patterns: &[&str]) {
        let seven_pattern: &str = patterns.iter().filter(|s| s.len() == 3).next().unwrap();
        self.seven_pattern = Some(seven_pattern.sorted());
    }

    fn find_four(&mut self, patterns: &[&str]) {
        let four_pattern: &str = patterns.iter().filter(|s| s.len() == 4).next().unwrap();
        self.four_pattern = Some(four_pattern.sorted());
    }

    fn find_eight(&mut self, patterns: &[&str]) {
        let eight_pattern: &str = patterns.iter().filter(|s| s.len() == 7).next().unwrap();
        self.eight_pattern = Some(eight_pattern.sorted());
    }

    fn find_six(&mut self, patterns: &[&str]) {
        // Six is the only digit with 6 segments lit that doesn't contain both 1 segments
        let one_pattern = self.one_pattern.as_ref().unwrap();
        let six : Vec<&str> = patterns.iter()
            .filter(|s| s.len() == 6)
            .filter(|s| s.contains(one_pattern.chars().nth(0).unwrap()) ^ s.contains(one_pattern.chars().nth(1).unwrap()))
            .map(|s| *s).collect();
        self.six_pattern = Some(six[0].sorted());
    }
    
    fn find_zero_two_three_five_nine(&mut self, patterns: &[&str]) {
        let four_pattern: HashSet<char> = HashSet::from_iter(self.four_pattern.as_ref().unwrap().chars());
        let one_pattern = self.one_pattern.as_ref().unwrap();

        let five_seg_candidates: Vec<&str> = patterns.iter().filter(|s| s.len() == 5).map(|s| *s).collect();


        let middle_segments: HashSet<char> = five_seg_candidates.iter().map(|s| HashSet::from_iter(s.chars())).reduce(|left: HashSet<char>, right| {
           left.intersection(&right).map(|c| *c).collect()
        }).unwrap();

        // zero is the only pattern with 6 chars that contains exactly 2 of the middle 
        let zero = patterns.iter()
            .filter(|s| s.len() == 6)
            .filter(|s| {
                let chars: HashSet<char> = HashSet::from_iter(s.chars());
                middle_segments.intersection(&chars).count() == 2
            })
            .next().unwrap();
        self.zero_pattern = Some(zero.sorted());

        // ideally I just look for the segment of length 6 that isn't zero pattern or six pattern
        // but I already sorted the six pattern
        let nine = patterns.iter()
            .filter(|s| s.len() == 6)
            .filter(|s| s.contains(one_pattern.chars().nth(0).unwrap()) && s.contains(one_pattern.chars().nth(1).unwrap()))
            .filter(|s| *s != zero)
            .next().unwrap();
        self.nine_pattern = Some(nine.sorted());

        let one_candidates: HashSet<char> = HashSet::from_iter(self.one_pattern.as_ref().unwrap().chars());
        let one_chars: Vec<&char> = one_candidates.iter().collect();
        let non_top_left: HashSet<char> = one_candidates.union(&middle_segments).map(|c| *c).collect();

        // one itty bitty use of clone
        let top_left: &char = &(&four_pattern - &non_top_left).iter().next().unwrap().clone();

        // Why so many derefs??
        // five is the only five segment pattern which contains the top left
        let five: &str = five_seg_candidates.iter().filter(|s| s.contains(*top_left)).next().unwrap();
        // three is the only five segment pattern which contains the top left
        let three: &str = five_seg_candidates.iter().filter(|s| s.contains(*one_chars[0]) && s.contains(*one_chars[1])).next().unwrap();
        // two is the only five segment pattern that isn't three or five
        let two: &str = five_seg_candidates.iter().filter(|s| **s != three).filter(|s| **s != five).next().unwrap();

        self.three_pattern = Some(three.sorted());
        self.two_pattern = Some(two.sorted());
        self.five_pattern = Some(five.sorted());
    }

}




fn main() {
    // optimization idea: instead of all these stupid temporary HashSet<>, write a method which
    // converts a pattern to a u8 and compare the u8s
    let output: usize = file_to_vec("input/8/a.txt") 
        .iter()
        .map(|s| s.split(" | ").collect())
        .map(|parts: Vec<&str>| {
                let display = SevenSegDisplay::new(&parts[0].split(" ").collect::<Vec<&str>>()[..]);
                display.get_readout(parts[1].split(" ").collect())
            })
        .sum();
    println!("Sum of displays: {}", output);
}
