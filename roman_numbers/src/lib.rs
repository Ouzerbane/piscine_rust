use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut result = Vec::new();

        let roman_map: &[(u32, &[RomanDigit])] = &[
            (1000, &[M]),
            (900, &[C, M]),
            (500, &[D]),
            (400, &[C, D]),
            (100, &[C]),
            (90, &[X, C]),
            (50, &[L]),
            (40, &[X, L]),
            (10, &[X]),
            (9, &[I, X]),
            (5, &[V]),
            (4, &[I, V]),
            (1, &[I]),
        ];

        for &(value, digits) in roman_map {
            while num >= value {
                result.extend_from_slice(digits);
                num -= value;
            }
        }

        RomanNumber(result)
    }
}
