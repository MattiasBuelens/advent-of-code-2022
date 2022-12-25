use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Digit {
    Two = 2,
    One = 1,
    Zero = 0,
    Minus = -1,
    DoubleMinus = -2,
}

impl From<char> for Digit {
    fn from(c: char) -> Digit {
        match c {
            '2' => Digit::Two,
            '1' => Digit::One,
            '0' => Digit::Zero,
            '-' => Digit::Minus,
            '=' => Digit::DoubleMinus,
            c => panic!("invalid snafu digit: {c}"),
        }
    }
}

impl From<Digit> for char {
    fn from(digit: Digit) -> char {
        match digit {
            Digit::Two => '2',
            Digit::One => '1',
            Digit::Zero => '0',
            Digit::Minus => '-',
            Digit::DoubleMinus => '=',
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Snafu {
    digits: Vec<Digit>,
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.chars().map(Digit::from).collect::<Vec<_>>();
        Ok(Snafu { digits })
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for &digit in self.digits.iter() {
            f.write_char(digit.into())?;
        }
        Ok(())
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<Snafu> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

impl From<&Snafu> for i64 {
    fn from(snafu: &Snafu) -> Self {
        snafu
            .digits
            .iter()
            .rev()
            .enumerate()
            .map(|(position, &digit)| 5i64.pow(position as u32) * (digit as i64))
            .sum()
    }
}

impl From<Snafu> for i64 {
    fn from(snafu: Snafu) -> Self {
        Self::from(&snafu)
    }
}

impl From<i64> for Snafu {
    fn from(mut value: i64) -> Self {
        let mut digits = Vec::new();
        while value != 0 {
            let remainder = value % 5;
            value /= 5;
            let digit = match remainder {
                0 => Digit::Zero,
                1 => Digit::One,
                2 => Digit::Two,
                3 => {
                    value += 1;
                    Digit::DoubleMinus
                }
                4 => {
                    value += 1;
                    Digit::Minus
                }
                -1 => Digit::Minus,
                -2 => Digit::DoubleMinus,
                -3 => {
                    value -= 1;
                    Digit::Two
                }
                -4 => {
                    value -= 1;
                    Digit::One
                }
                _ => panic!("unexpected remainder: {remainder} for {value}"),
            };
            digits.push(digit);
        }
        digits.reverse();
        Snafu { digits }
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &[Snafu]) -> String {
    let sum = input.iter().map(i64::from).sum::<i64>();
    Snafu::from(sum).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: &'static str = r"
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
        .trim();
    }

    #[test]
    fn test_part1() {
        let input = input_generator(&TEST_INPUT);
        assert_eq!(part1(&input), "2=-1=0");
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(Snafu::from(1).to_string(), "1");
        assert_eq!(Snafu::from(2).to_string(), "2");
        assert_eq!(Snafu::from(3).to_string(), "1=");
        assert_eq!(Snafu::from(4).to_string(), "1-");
        assert_eq!(Snafu::from(5).to_string(), "10");
        assert_eq!(Snafu::from(6).to_string(), "11");
        assert_eq!(Snafu::from(7).to_string(), "12");
        assert_eq!(Snafu::from(8).to_string(), "2=");
        assert_eq!(Snafu::from(9).to_string(), "2-");
        assert_eq!(Snafu::from(10).to_string(), "20");
        assert_eq!(Snafu::from(15).to_string(), "1=0");
        assert_eq!(Snafu::from(20).to_string(), "1-0");
        assert_eq!(Snafu::from(2022).to_string(), "1=11-2");
        assert_eq!(Snafu::from(12345).to_string(), "1-0---0");
        assert_eq!(Snafu::from(314159265).to_string(), "1121-1110-1=0");
    }

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(i64::from(Snafu::from_str("1=-0-2").unwrap()), 1747);
        assert_eq!(i64::from(Snafu::from_str("12111").unwrap()), 906);
        assert_eq!(i64::from(Snafu::from_str("2=0=").unwrap()), 198);
        assert_eq!(i64::from(Snafu::from_str("21").unwrap()), 11);
        assert_eq!(i64::from(Snafu::from_str("2=01").unwrap()), 201);
        assert_eq!(i64::from(Snafu::from_str("111").unwrap()), 31);
        assert_eq!(i64::from(Snafu::from_str("20012").unwrap()), 1257);
        assert_eq!(i64::from(Snafu::from_str("112").unwrap()), 32);
        assert_eq!(i64::from(Snafu::from_str("1=-1=").unwrap()), 353);
        assert_eq!(i64::from(Snafu::from_str("1-12").unwrap()), 107);
        assert_eq!(i64::from(Snafu::from_str("12").unwrap()), 7);
        assert_eq!(i64::from(Snafu::from_str("1=").unwrap()), 3);
        assert_eq!(i64::from(Snafu::from_str("122").unwrap()), 37);
    }
}
