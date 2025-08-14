#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}


use std::cmp::{Ord, Ordering};
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.len() > 3 {
            return Err(());
        }
        let (antigen_str, rh_str) = if s.starts_with("AB") {
            if s.len() == 2 {
                ("AB", "+")
            } else if s.len() == 3 {
                ("AB", &s[2..3])
            } else {
                return Err(());
            }
        } else {
            if s.len() == 1 {
                (&s[0..1], "+")
            } else if s.len() == 2 {
                (&s[0..1], &s[1..2])
            } else {
                return Err(());
            }
        };

        let antigen = antigen_str.parse().map_err(|_| ())?;
        let rh_factor = rh_str.parse().map_err(|_| ())?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh = match self.rh_factor {
            RhFactor::Negative => "-",
            RhFactor::Positive => "+",
        };
        write!(f, "{:?}{}", self.antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let rh_compatible = match (self.rh_factor.clone(), other.rh_factor.clone()) {
            (RhFactor::Negative, RhFactor::Positive) => false,
            _ => true,
        };

        let antigen_compatible = match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
        };

        rh_compatible && antigen_compatible
    }

    pub fn donors(&self) -> Vec<Self> {
        let all_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        all_types.into_iter().filter(|donor| self.can_receive_from(donor)).collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
        let all_types = vec![
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
            BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
        ];

        all_types.into_iter().filter(|recipient| recipient.can_receive_from(self)).collect()
    }
}
