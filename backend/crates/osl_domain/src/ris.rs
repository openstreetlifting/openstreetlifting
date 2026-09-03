//! The Relative Index for Streetlifting, created by Waris Radji and Mathieu
//! Ardoin, <https://warisradji.com/ris/>. FinalRep refits the constants each
//! year, so a score only means something next to the edition that produced it.
//!
//! `RIS = Total * 100 / f(BW)` where
//! `f(x) = A + (K - A) / (1 + Q * e^(-B * (x - v)))`.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::Gender;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edition {
    V2024,
    V2025,
    V2026,
}

#[derive(Debug, Clone, Copy)]
pub struct Constants {
    pub a: f64,
    pub k: f64,
    pub b: f64,
    pub v: f64,
    pub q: f64,
}

const MEN_2024: Constants = Constants {
    a: 308.5,
    k: 536.363293273545,
    b: 0.1013742998889087,
    v: 60.39746396108519,
    q: 2.2100554021356436,
};
const WOMEN_2024: Constants = Constants {
    a: 180.83333333333334,
    k: 241.74894812447127,
    b: 0.24737447093352963,
    v: 50.192523991912516,
    q: 1.3834293214001367,
};
const MEN_2025: Constants = Constants {
    a: 338.0,
    k: 549.0,
    b: 0.11354,
    v: 74.777,
    q: 0.53096,
};
const WOMEN_2025: Constants = Constants {
    a: 164.0,
    k: 270.0,
    b: 0.13776,
    v: 57.855,
    q: 0.37089,
};
const MEN_2026: Constants = Constants {
    a: 335.5625,
    k: 556.1103380806655,
    b: 0.10289374204365953,
    v: 76.74125992622565,
    q: 0.4973075488457353,
};
const WOMEN_2026: Constants = Constants {
    a: 189.0625,
    k: 281.8317115772956,
    b: 0.28812554082125424,
    v: 69.83492229989282,
    q: 0.004337919108356886,
};

impl Edition {
    /// Every competition is scored with this one, whatever year it was lifted,
    /// so one ranking never mixes two scales.
    pub const CURRENT: Self = Self::V2026;

    pub const ALL: [Self; 3] = [Self::V2024, Self::V2025, Self::V2026];

    pub fn year(self) -> i32 {
        match self {
            Self::V2024 => 2024,
            Self::V2025 => 2025,
            Self::V2026 => 2026,
        }
    }

    pub fn from_year(year: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|it| it.year() == year)
    }

    pub fn credit(self) -> &'static str {
        match self {
            Self::V2024 | Self::V2025 => "Waris Radji and Mathieu Ardoin",
            Self::V2026 => "FinalRep",
        }
    }

    /// Mx takes the men's curve, the only other fit that exists.
    pub fn constants(self, gender: Gender) -> Constants {
        match (self, gender) {
            (Self::V2024, Gender::F) => WOMEN_2024,
            (Self::V2024, _) => MEN_2024,
            (Self::V2025, Gender::F) => WOMEN_2025,
            (Self::V2025, _) => MEN_2025,
            (Self::V2026, Gender::F) => WOMEN_2026,
            (Self::V2026, _) => MEN_2026,
        }
    }
}

impl Constants {
    /// The total an average elite athlete of this bodyweight is expected to
    /// put up.
    fn benchmark(&self, bodyweight: f64) -> f64 {
        self.a + (self.k - self.a) / (1.0 + self.q * (-self.b * (bodyweight - self.v)).exp())
    }
}

pub fn compute(bodyweight: Decimal, total: Decimal, gender: Gender, edition: Edition) -> Decimal {
    let constants = edition.constants(gender);
    let benchmark = constants.benchmark(bodyweight.to_f64().unwrap_or_default());
    let score = total.to_f64().unwrap_or_default() * 100.0 / benchmark;

    Decimal::from_f64_retain(score)
        .unwrap_or(Decimal::ZERO)
        .round_dp(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;

    fn dec(value: f64) -> Decimal {
        Decimal::from_f64(value).expect("a test constant is a decimal")
    }

    /// Bodyweight, total and score as the RIS presentation publishes them for
    /// Final Rep Worlds 2023, which the 2024 edition scored.
    const MEN_WORLDS_2023: [(f64, f64, f64); 15] = [
        (91.7, 586.5, 113.43),
        (85.6, 557.25, 110.79),
        (73.0, 494.0, 109.90),
        (93.0, 570.0, 109.77),
        (66.0, 442.75, 108.08),
        (106.9, 572.5, 107.63),
        (98.0, 562.5, 106.99),
        (79.6, 513.75, 106.65),
        (86.3, 532.5, 105.46),
        (113.2, 562.5, 105.34),
        // The presentation prints 532.7, the only total in its table that is
        // not a multiple of 0.25. At 532.75 the published score reproduces.
        (87.1, 532.75, 105.06),
        (72.7, 470.0, 104.94),
        (72.5, 467.5, 104.64),
        (93.9, 541.5, 104.00),
        (93.0, 540.0, 103.99),
    ];

    const WOMEN_WORLDS_2023: [(f64, f64, f64); 9] = [
        (62.3, 257.5, 108.28),
        (56.0, 245.0, 108.09),
        (67.6, 260.0, 108.05),
        (53.7, 230.5, 105.08),
        (56.6, 237.5, 104.03),
        (65.9, 248.75, 103.62),
        (62.2, 245.0, 103.07),
        (54.7, 228.75, 102.70),
        (62.7, 242.5, 101.82),
    ];

    #[test]
    fn reproduces_the_published_worlds_2023_scores() {
        for (bodyweight, total, published) in MEN_WORLDS_2023 {
            assert_eq!(
                compute(dec(bodyweight), dec(total), Gender::M, Edition::V2024),
                dec(published),
                "men, bodyweight {bodyweight}, total {total}"
            );
        }

        for (bodyweight, total, published) in WOMEN_WORLDS_2023 {
            assert_eq!(
                compute(dec(bodyweight), dec(total), Gender::F, Edition::V2024),
                dec(published),
                "women, bodyweight {bodyweight}, total {total}"
            );
        }
    }
}
