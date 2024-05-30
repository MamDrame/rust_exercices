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

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}



use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Antigen::*;
        match s {
            "A" => Ok(A),
            "AB" => Ok(AB),
            "B" => Ok(B),
            "O" => Ok(O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RhFactor::*;
        match s {
            "+" => Ok(Positive),
            "-" => Ok(Negative),
            _ => Err(()),
        }
    }
}

use std::cmp::{Ord, Ordering};
impl Ord for BloodType {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.antigen != other.antigen {
			self.antigen.cmp(&other.antigen)
		} else {
			self.rh_factor.cmp(&other.rh_factor)
		}
	}
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen_str = s.get(..s.len() - 1).unwrap_or_default();
        let rh_factor_str: &str = s.get(s.len() - 1..).unwrap_or_default();

        let antigen = Antigen::from_str(antigen_str)?;
        let rh_factor = RhFactor::from_str(rh_factor_str)?;

        Ok(Self { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Antigen::*;
        use RhFactor::*;
        write!(f,
            "{}{}",
            match self.antigen {
                A => "A",
                AB => "AB",
                B => "B",
                O => "O",
            },
            match self.rh_factor {
                Positive => "+",
                Negative => "-",
            }
        )
    }
}

impl BloodType {
    pub fn new(antigen: Antigen, rh_factor: RhFactor) -> Self {
        Self { antigen, rh_factor }
    }

    pub fn to_vec(&self, bloods_str: &str) -> Vec<Self> {
        let mut result = Vec::new();

        bloods_str
            .split(",")
            .for_each(|s| match BloodType::from_str(s) {
                Ok(value) => {
                    result.push(value);
                }
                Err(_) => (),
            });
        result
    }

    pub fn can_receive_from(&self, other: &Self) -> bool {
		self.donors().iter().any(|b| b.antigen == other.antigen && b.rh_factor == other.rh_factor)
	}

    pub fn donors(&self) -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;
        match (&self.antigen, &self.rh_factor) {
            (A, Positive) => self.to_vec("A+,A-,O+,O-"),
            (A, Negative) => self.to_vec("A-,O-"),
            (AB, Positive) => self.to_vec("A+,A-,B+,B-,AB+,AB-,O+,O-"),
            (AB, Negative) => self.to_vec("AB-,A-,B-,O-"),
            (B, Positive) => self.to_vec("B+,B-,O+,O-"),
            (B, Negative) => self.to_vec("B-,O-"),
            (O, Positive) => self.to_vec("O+,O-"),
            (O, Negative) => self.to_vec("O-"),
        }
    }

    pub fn recipients(&self) -> Vec<Self> {
		use Antigen::*;
        use RhFactor::*;
        match (&self.antigen, &self.rh_factor) {
            (A, Positive) => self.to_vec("A+,AB+"),
            (A, Negative) => self.to_vec("A+,A-,AB+,AB-"),
            (AB, Positive) => self.to_vec("AB+"),
            (AB, Negative) => self.to_vec("A+,A-,AB+,AB-"),
            (B, Positive) => self.to_vec("B+, AB+"),
            (B, Negative) => self.to_vec("B+,B-,AB+,AB-"),
            (O, Positive) => self.to_vec("O+,A+,B+,AB+"),
            (O, Negative) => self.to_vec("A+,A-,B+,B-,AB+,AB-,O+,O-"),
        }
	}
}
