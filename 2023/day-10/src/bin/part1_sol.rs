use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Ash,
    Rock,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseError {
    InvalidChar,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidChar => write!(f, "Invalid character encountered"),
        }
    }
}
impl TryFrom<char> for Cell {
    type Error = ParseError;
    fn try_from(c: char) -> Result<Self, ParseError> {
        match c {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            _ => Err(ParseError::InvalidChar),
        }
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    rows: Vec<Vec<Cell>>,
    cols: Vec<Vec<Cell>>,
}

impl Pattern {
    fn from_rows(rows: Vec<Vec<Cell>>) -> Pattern {
        let cols = rows
            .iter()
            .next()
            .map(|c| {
                (0..c.len())
                    .map(|idx| {
                        rows.iter()
                            .map(|row| *row.iter().nth(idx).unwrap())
                            .collect()
                    })
                    .collect()
            })
            .unwrap();
        Pattern { rows, cols }
    }

    fn mirror(i: usize, r: &Vec<Vec<Cell>>) -> bool {
        (0..i.min(r.len() - i)).all(|idx| r[i - idx - 1] == r[i + idx])
    }
}
fn main() {
    let patterns: Vec<Pattern> = include_str!("./inputs/input1.txt")
        .trim_end()
        .split("\n\n")
        .map(|l| {
            Pattern::from_rows(
                l.split_ascii_whitespace()
                    .map(|s| s.chars().map(|c| Cell::try_from(c).unwrap()).collect())
                    .collect(),
            )
        })
        .collect();
    println!(
        "{}",
        patterns
            .iter()
            .map(|p| {
                100 * (1..p.rows.len())
                    .skip_while(|&i| !Pattern::mirror(i, &p.rows))
                    .next()
                    .unwrap_or_default()
                    + (1..p.cols.len())
                        .skip_while(|&i| !Pattern::mirror(i, &p.cols))
                        .next()
                        .unwrap_or_default()
            })
            .sum::<usize>()
    );
}
