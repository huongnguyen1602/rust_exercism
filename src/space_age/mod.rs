// the first way
// pub struct Duration(f64);

// impl From<u64> for Duration {
//     fn from(s: u64) -> Self {
//         Duration(s as f64)
//     }    
// }

// pub trait Planet {
//     const PERIOD: f64;
//     fn years_during(d: &Duration) -> f64 {
//         (d.0 / 31_557_600.0) / Self::PERIOD
//     }
// }

// macro_rules! planet {
//     ($n: ident, $p: expr) => {
//         pub struct $n; impl Planet for $n {
//             const PERIOD: f64 = $p;
//         }
//     };
// }

// planet!(Mercury, 0.2408467);
// planet!(Venus, 0.61519726);
// planet!(Earth, 1.0);
// planet!(Mars, 1.8808158);
// planet!(Jupiter, 11.862615);
// planet!(Saturn, 29.447498);
// planet!(Uranus, 84.016846);
// planet!(Neptune, 164.79132);


// the second way
// #[derive(Debug)]
// pub struct Duration{
//     seconds: f64,
// }

// impl From<u64> for Duration {
//     fn from(s: u64) -> Self {
//         Self { seconds: s as f64 }
//     }
// }

// pub trait Planet {
//     const ORBITAL_PERIOD: f64;
//     fn years_during(d: &Duration) -> f64 {
//         d.seconds / Self::ORBITAL_PERIOD
//     }
// }

// const EARTH_YEAR_SECONDS: f64 = 60.0 * 60.0 * 24.0 * 365.25;
// macro_rules! planets {
//     ($($planet_name:ident: $earth_year_ratio:literal),+) => {
//         $(
//             pub struct $planet_name;
//             impl Planet for $planet_name {
//                 const ORBITAL_PERIOD: f64 = EARTH_YEAR_SECONDS * $earth_year_ratio;
//             }
//         )+
//     };
// }

// planets!(
//     Mercury: 0.2408467,
//     Venus: 0.61519726,
//     Earth: 1.0,
//     Mars: 1.880815,
//     Jupiter: 11.862615,
//     Saturn: 29.447498,
//     Uranus: 84.016846,
//     Neptune: 164.79132
// );


// the third way
#[derive(Debug)]
pub struct Duration{
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
       Duration { years: s as f64 / 31557600 as f64 } 
    }
}

pub trait Planet {
    fn proportion() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.years / Self::proportion()
    }
}

macro_rules! planet {
    ($planet: ident, $proportion:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            fn proportion() -> f64{
                $proportion
            }
        }
    };
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);


// Test
fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta = 0.01;
    if diff > delta {
        panic!("your result of {actual} should be within {delta} of the expected result {expected}")
    }
}

pub fn test() {
    fn earth_age() {
        let duration = Duration::from(1_000_000_000);
        assert_in_delta(31.69, Earth::years_during(&duration));
    }
    fn mercury_age() {
        let duration = Duration::from(2_134_835_688);
        assert_in_delta(280.88, Mercury::years_during(&duration));
    }
    fn venus_age() {
        let duration = Duration::from(189_839_836);
        assert_in_delta(9.78, Venus::years_during(&duration));
    }
    fn mars_age() {
        let duration = Duration::from(2_129_871_239);
        assert_in_delta(35.88, Mars::years_during(&duration));
    }
    fn jupiter_age() {
        let duration = Duration::from(901_876_382);
        assert_in_delta(2.41, Jupiter::years_during(&duration));
    }
    fn saturn_age() {
        let duration = Duration::from(2_000_000_000);
        assert_in_delta(2.15, Saturn::years_during(&duration));
    }
    fn uranus_age() {
        let duration = Duration::from(1_210_123_456);
        assert_in_delta(0.46, Uranus::years_during(&duration));
    }
    fn neptune_age() {
        let duration = Duration::from(1_821_023_456);
        assert_in_delta(0.35, Neptune::years_during(&duration));
    }
    println!("it' ok");
}