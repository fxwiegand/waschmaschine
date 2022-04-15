use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize, Debug, Clone, PartialEq)]
enum Region {
    Single,
    Double,
    Triple,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub(crate) struct Dart {
    field: u16,
    region: Region,
}

impl Dart {
    fn _score(&self) -> u16 {
        match self.region {
            Region::Single => self.field,
            Region::Double => self.field * 2,
            Region::Triple => self.field * 3,
        }
    }
}

impl FromStr for Dart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(1) {
            ("S", s) => Ok(Dart {
                field: u16::from_str(s).unwrap(),
                region: Region::Single,
            }),
            ("D", s) => Ok(Dart {
                field: u16::from_str(s).unwrap(),
                region: Region::Double,
            }),
            ("T", s) => Ok(Dart {
                field: u16::from_str(s).unwrap(),
                region: Region::Triple,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub(crate) struct Throw {
    pub(crate) darts: Vec<Dart>,
}

impl Throw {
    fn _score(&self) -> u16 {
        self.darts.iter().map(|d| d._score()).sum()
    }
}

impl FromStr for Throw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Throw {
            darts: s
                .split_whitespace()
                .map(|d| Dart::from_str(d).unwrap())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Throw;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_checkouts() {
        let checkouts: HashMap<u16, Throw> = include_str!("../checkouts.txt")
            .lines()
            .map(|l| l.trim().split_once(' ').unwrap())
            .map(|(score, throw)| {
                (
                    u16::from_str(score).unwrap(),
                    Throw::from_str(throw).unwrap(),
                )
            })
            .collect();
        for (score, throw) in checkouts {
            assert_eq!(score, throw._score())
        }
    }
}
