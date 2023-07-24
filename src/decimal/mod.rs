use std::{
    cmp::Ordering,
    ops::{Add, Sub, Mul}, result, io::Seek
};

use num_bigint::BigInt;

#[derive(Debug)]
pub struct Decimal{
    significand: BigInt,
    exponent: u32,
}


impl Decimal {
    // pub fn try_from(input: &str) -> Option<Self> {
    //     let mut is_positive = true;
    //     let mut seperator_found = false;
    //     let mut exponent = 0;
    //     let mut significand: BigInt = 0.into();
    //     let mut base: BigInt = 10.into();
    //     for x in input.bytes().enumerate() {
    //         match x {
    //             (0, b'-') => is_positive = false,
    //             (0, b'+') => is_positive = true,
    //             (_, b'.') if seperator_found => seperator_found = true,
    //             (_,c) if c.is_ascii_digit() => {
    //                 significand = significand * &base + BigInt::from(c-b'0');
    //                 if seperator_found {
    //                     exponent += 1;
    //                 }
    //             }
    //             _ => return None,
    //         }
    //     }

    //     if !is_positive {
    //         significand = -significand;
    //     }

    //     let mut result = Decimal{
    //         significand,
    //         exponent,
    //     };

    //     result.normalize();
    //     println!("{result:?}");
    //     result.into()
    // }

    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut is_positive = true;
        let mut separator_found = false;
        let mut exponent = 0;
        let mut significand: BigInt = 0.into();
        let base: BigInt = 10.into();
        for x in input.bytes().enumerate() {
            match x {
                (0, b'-') => is_positive = false,
                (0, b'+') => is_positive = true,
                (_, b'.') if !separator_found => separator_found = true,
                (_, c) if c.is_ascii_digit() => {
                    significand = significand * &base + BigInt::from(c - b'0');
                    if separator_found {
                        exponent += 1;
                    }
                }
                _ => return None,
            }
        }
        if !is_positive {
            significand = -significand;
        }
        let mut result = Decimal {
            significand,
            exponent,
        };
        result.normalize();
        result.into()
    }

    fn normalize(&mut self) {
        while self.exponent > 0 && (&self.significand % 10) == 0i32.into(){
            self.significand /= 10;
            self.exponent -= 1;
        }
    }

    fn shift_significand(source: &BigInt, shift: &u32) -> BigInt{
        source * BigInt::from(10).pow(*shift)
    }

    fn coerce_add(self, rhs: Self) -> Self {
       let mut result = match Ord::cmp(&self.exponent, &rhs.exponent) {
           Ordering::Less => Self {
                significand: Self::shift_significand(
                    &self.significand, 
                    &(&rhs.exponent - &self.exponent)
                ) + rhs.significand,
                exponent: rhs.exponent,
           },
           Ordering::Equal => Self { 
                significand: self.significand + rhs.significand,
                exponent: self.exponent },
           Ordering::Greater => Self { 
                significand: Self::shift_significand(
                    &rhs.significand,
                    &(self.exponent - rhs.exponent)) + self.significand,
                exponent: self.exponent },
       }; 
       result.normalize();
       result
    }
}

impl Add for Decimal {
    type Output = Decimal;
    fn add(self, rhs: Self) -> Self::Output {
        self.coerce_add(rhs)
    }
}

impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, rhs: Self) -> Self::Output {
        self.coerce_add(Decimal { 
            significand: -rhs.significand,
            exponent: rhs.exponent 
            })
    }
}

impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = match Ord::cmp(&self.exponent, &rhs.exponent){
            Ordering::Less => Self {
                significand: Self::shift_significand(
                    &self.significand,
                    &(rhs.exponent - self.exponent)) * rhs.significand,
                exponent: rhs.exponent * 2,
            },
            Ordering::Equal => Self {
                significand: self.significand * rhs.significand,
                exponent: rhs.exponent * 2,
            },
            Ordering::Greater => Self {
                significand: Self::shift_significand(
                    &rhs.significand,
                    &(self.exponent - rhs.exponent)) * self.significand,
                exponent: self.exponent * 2,
            },
        };
        result.normalize();
        result
    }
}


impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.significand == other.significand && self.exponent == other.exponent
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match Ord::cmp(&self.exponent, &other.exponent) {
            Ordering::Less => PartialOrd::partial_cmp(
                &Self::shift_significand(&self.significand, &(other.exponent - self.exponent)),
                &other.significand
                ),
            Ordering::Equal => PartialOrd::partial_cmp(
                &self.significand,
                &other.significand
                ),
            Ordering::Greater => PartialOrd::partial_cmp(
                &self.significand,
                &Self::shift_significand(&other.significand, &(self.exponent - &other.exponent)),
                ),
            
        }
    }
    // chú ý thứ tự tham số trong một function
}

fn decimal(input: &str) -> Decimal{
    Decimal::try_from(input).expect("That was supported to be a valid value")
} 


pub fn test() {
    const BIGS: [&str; 3] = [
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000001",
    "100000000000000000000000000000000000000000000.00000000000000000000000000000000000000002",
    "200000000000000000000000000000000000000000000.00000000000000000000000000000000000000003",
    ];
    assert!(decimal("0.0") == decimal("0.0"));
    assert!(decimal("1.0") == decimal("1.0"));
    for big in BIGS.iter() {
        assert!(decimal(big) == decimal(big));
    }
    assert!(decimal("0.0") != decimal("1.0"));
    assert!(decimal(BIGS[0]) != decimal(BIGS[1]));
    for slice_2 in BIGS.windows(2) {
        assert!(decimal(slice_2[1]) > decimal(slice_2[0]));
    }
    for slice_2 in BIGS.windows(2) {
        assert!(decimal(slice_2[0]) < decimal(slice_2[1]));
    }
    assert_eq!(decimal("0.1") + decimal("0.2"), decimal("0.3"));
    assert_eq!(decimal(BIGS[0]) + decimal(BIGS[1]), decimal(BIGS[2]));
    assert_eq!(decimal(BIGS[1]) + decimal(BIGS[0]), decimal(BIGS[2]));
    assert_eq!(decimal(BIGS[2]) - decimal(BIGS[1]), decimal(BIGS[0]));
    assert_eq!(decimal(BIGS[2]) - decimal(BIGS[0]), decimal(BIGS[1]));
    assert_eq!(decimal("2.1") * decimal("1.0"), decimal("2.1"));
    assert_eq!(decimal("1.0") * decimal("2.1"), decimal("2.1"));
    for big in BIGS.iter() {
        assert_eq!(decimal(big) * decimal("2"), decimal(big) + decimal(big));
    }
    assert!(decimal("1.0") < decimal("1.1"));
    assert!(decimal("0.00000000000000000000001") > decimal("-20000000000000000000000000000"));


    println!("it's ok");
}