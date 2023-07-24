use num_bigint::BigInt;
use num_traits::Zero;
use std::iter::Sum;
use std::str::FromStr;
use std::ops::{Add, Sub, Mul};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Decimal{
    value: BigInt,
    exp: isize,
}

impl Decimal {
    pub fn new(input: &str) -> Option<Self>{
        let value = BigInt::from_str(input.replace('.',"").as_str()).unwrap();
        let exp = if let Some(index) = input.find('.'){
            -((input.len()-1-index) as isize)
        } else {
            0
        };
        Some(Self{value, exp})
    }

    pub fn try_from(input: &str) -> Option<Self>{
        let Some(mut d) = Decimal::new(input) else{
            return None;
        };
        d.normalize();
        Some(d)
    }

    pub fn normalize(&mut self) {
        while &self.value %10 == Zero::zero() && self.exp != 0 {
            self.value /= 10;
            self.exp += 1;
        }
    }

    pub fn dec_scale(&mut self, exp: isize) {
        while self.exp > exp { 
            self.value *= 10;
            self.exp -= 1;
        }
    }

    pub fn negative(&self) -> Self {
        Self { value: -self.value.clone(), exp: self.exp }
    }

    
}

pub fn equal_exp(l: &mut Decimal, r: &mut Decimal) {
    match l.exp.cmp(&r.exp) {
        Ordering::Equal => {},
        Ordering::Less => r.dec_scale(l.exp),
        Ordering::Greater => l.dec_scale(r.exp),
    }
}  

impl Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let mut l = self.clone();
        let mut r = other.clone();
        equal_exp(&mut l, &mut r);

        let mut result = Self{value: l.value + r.value, exp: l.exp.min(r.exp)};
        result.normalize();
        result
    }
}


impl Sub for Decimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self + other.negative()
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut result = Self{value: self.value * other.value, exp: self.exp + other.exp};
        result.normalize();
        result
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut l = self.clone();
        let mut r = self.clone();
        equal_exp(&mut l, &mut r);
        Some(l.value.cmp(&other.value))
    }
}


pub fn test() {
    let a = Decimal::new("10000.21").unwrap();
    let b = Decimal::new("0.00001").unwrap();
    let sum = a.clone()+b.clone();
    let subtract = a.clone()-b.clone();
    let multiply = a.clone()*b.clone();
    println!("Sum: {sum:?} \n Subtraction: {subtract:?} \n Multiplication: {multiply:?}");
    println!("it's ok");
}