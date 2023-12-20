use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// ## 1000
/// This module provides functionality for working with data sizes in the SI format.
/// It includes constants for different size thresholds (e.g., kilobytes, megabytes),
/// a data structure (`SIUnit`) representing various units of data size, and methods
/// for convenient conversion and display of data sizes in human-readable formats.
pub mod si {
    use super::*;

    // SI format.
    /// Kilobyte in bytes.
    pub const BYTES_IN_KB: f64 = 1000.0;
    /// Megabyte in bytes.
    pub const BYTES_IN_MB: f64 = BYTES_IN_KB * BYTES_IN_KB;
    /// Gigabyte in bytes.
    pub const BYTES_IN_GB: f64 = BYTES_IN_MB * BYTES_IN_KB;
    /// Terabyte in bytes.
    pub const BYTES_IN_TB: f64 = BYTES_IN_GB * BYTES_IN_KB;
    /// Petabyte in bytes.
    pub const BYTES_IN_PB: f64 = BYTES_IN_TB * BYTES_IN_KB;
    /// Exabyte in bytes.
    pub const BYTES_IN_EB: f64 = BYTES_IN_PB * BYTES_IN_KB;

    /// Represents different units of data size, allowing for conversion between human-readable
    /// representations and precise byte values.
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(PartialOrd, PartialEq, Clone, Copy, EnumIter)]
    pub enum SIUnit {
        Byte(f64, f64),
        Kilobyte(f64, f64),
        Megabyte(f64, f64),
        Gigabyte(f64, f64),
        Terabyte(f64, f64),
        Petabyte(f64, f64),
        Exabyte(f64, f64),
        Overflow,
    }
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, EnumIter)]
    pub enum SISize {
        Byte,
        Kilobyte,
        Megabyte,
        Gigabyte,
        Terabyte,
        Petabyte,
        Exabyte,
    }

    impl SIUnit {
        pub fn new(value: f64, unit_type: SISize) -> SIUnit {
            if value == f64::INFINITY || value == f64::NEG_INFINITY || value > f64::MAX {
                return SIUnit::Overflow;
            } else if value.is_sign_negative() || value.is_nan() {
                return SIUnit::default();
            }
            match unit_type {
                SISize::Byte => SIUnit::Byte(value, value),
                SISize::Kilobyte => SIUnit::Kilobyte(value, value * BYTES_IN_KB),
                SISize::Megabyte => SIUnit::Megabyte(value, value * BYTES_IN_MB),
                SISize::Gigabyte => SIUnit::Gigabyte(value, value * BYTES_IN_GB),
                SISize::Terabyte => SIUnit::Terabyte(value, value * BYTES_IN_TB),
                SISize::Petabyte => SIUnit::Petabyte(value, value * BYTES_IN_PB),
                SISize::Exabyte => SIUnit::Exabyte(value, value * BYTES_IN_EB),
            }
        }

        pub fn auto(bytes: f64) -> SIUnit {
            if bytes.is_sign_negative() || bytes.is_nan() {
                return SIUnit::default();
            }
            match bytes {
                b if b == f64::INFINITY || b == f64::NEG_INFINITY || b > f64::MAX => {
                    SIUnit::Overflow
                }
                b if b < BYTES_IN_KB => SIUnit::Byte(b, b),
                b if b < BYTES_IN_MB => SIUnit::Kilobyte(b / BYTES_IN_KB, b),
                b if b < BYTES_IN_GB => SIUnit::Megabyte(b / BYTES_IN_MB, b),
                b if b < BYTES_IN_TB => SIUnit::Gigabyte(b / BYTES_IN_GB, b),
                b if b < BYTES_IN_PB => SIUnit::Terabyte(b / BYTES_IN_TB, b),
                b if b < BYTES_IN_EB => SIUnit::Petabyte(b / BYTES_IN_PB, b),
                _ => SIUnit::Exabyte(bytes / BYTES_IN_EB, bytes),
            }
        }

        #[cfg(not(tarpaulin_include))]
        pub fn get_values(&self) -> (f64, f64) {
            match self {
                SIUnit::Byte(value_h, value_b)
                | SIUnit::Kilobyte(value_h, value_b)
                | SIUnit::Megabyte(value_h, value_b)
                | SIUnit::Gigabyte(value_h, value_b)
                | SIUnit::Terabyte(value_h, value_b)
                | SIUnit::Petabyte(value_h, value_b)
                | SIUnit::Exabyte(value_h, value_b) => (*value_h, *value_b),
                SIUnit::Overflow => (f64::INFINITY, f64::INFINITY),
            }
        }
    }

    impl Default for SIUnit {
        fn default() -> Self {
            SIUnit::Byte(0.0, 0.0)
        }
    }

    impl Add for SIUnit {
        type Output = SIUnit;

        fn add(self, other: SIUnit) -> SIUnit {
            if self == SIUnit::Overflow || other == SIUnit::Overflow {
                SIUnit::Overflow
            } else {
                SIUnit::auto(self.get_values().1 + other.get_values().1)
            }
        }
    }

    impl Sub for SIUnit {
        type Output = SIUnit;

        fn sub(self, other: SIUnit) -> SIUnit {
            if self == SIUnit::Overflow || other == SIUnit::Overflow {
                SIUnit::Overflow
            } else {
                SIUnit::auto(self.get_values().1 - other.get_values().1)
            }
        }
    }

    impl Mul<f64> for SIUnit {
        type Output = SIUnit;

        fn mul(self, scalar: f64) -> SIUnit {
            if self == SIUnit::Overflow {
                SIUnit::Overflow
            } else {
                SIUnit::auto(self.get_values().1 * scalar)
            }
        }
    }

    impl Div<f64> for SIUnit {
        type Output = SIUnit;

        fn div(self, divisor: f64) -> SIUnit {
            if self == SIUnit::Overflow {
                SIUnit::Overflow
            } else {
                SIUnit::auto(self.get_values().1 / divisor)
            }
        }
    }

    impl From<SIUnit> for ies::IECUnit {
        fn from(si_unit: si::SIUnit) -> Self {
            match si_unit {
                SIUnit::Byte(value_h, value_b) => ies::IECUnit::Byte(value_h, value_b),
                SIUnit::Overflow => ies::IECUnit::Overflow,
                _ => ies::IECUnit::auto(si_unit.get_values().1),
            }
        }
    }

    impl From<SIUnit> for f64 {
        fn from(data_size_unit: SIUnit) -> Self {
            data_size_unit.get_values().1
        }
    }

    /// Converts an `SIUnit` to a `usize` value.
    impl From<SIUnit> for usize {
        /// Warning: This conversion may result in data loss.
        fn from(data_size_unit: SIUnit) -> Self {
            data_size_unit.get_values().1 as usize
        }
    }

    impl Display for SIUnit {
        #[cfg(not(tarpaulin_include))]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SIUnit::Byte(_, bytes) => write!(f, "{:.2} B", bytes),
                SIUnit::Kilobyte(kb, _) => write!(f, "{:.2} KB", kb),
                SIUnit::Megabyte(mb, _) => write!(f, "{:.2} MB", mb),
                SIUnit::Gigabyte(gb, _) => write!(f, "{:.2} GB", gb),
                SIUnit::Terabyte(tb, _) => write!(f, "{:.2} TB", tb),
                SIUnit::Petabyte(pb, _) => write!(f, "{:.2} PB", pb),
                SIUnit::Exabyte(eb, _) => write!(f, "{:.2} EB", eb),
                SIUnit::Overflow => write!(f, "Overflow"),
            }
        }
    }
}

/// ## 1024
/// This module offers functionality for dealing with data sizes in the IEC format.
/// Similar to the SI module, it contains constants for size thresholds and a data structure
/// (`IECUnit`) representing different units of data size. Additionally, it provides methods
/// for converting and displaying data sizes in human-readable formats according to the IEC standard.
pub mod ies {
    use super::*;
    // IEC format.
    /// Kibibyte in bytes.
    pub const BYTES_IN_KIB: f64 = 1024.0;
    /// Mebibyte in bytes.
    pub const BYTES_IN_MIB: f64 = BYTES_IN_KIB * BYTES_IN_KIB;
    /// Gibibyte in bytes.
    pub const BYTES_IN_GIB: f64 = BYTES_IN_MIB * BYTES_IN_KIB;
    /// Tebibyte in bytes.
    pub const BYTES_IN_TIB: f64 = BYTES_IN_GIB * BYTES_IN_KIB;
    /// Pebibyte in bytes.
    pub const BYTES_IN_PIB: f64 = BYTES_IN_TIB * BYTES_IN_KIB;
    /// Exbibytes in bytes.
    pub const BYTES_IN_EIB: f64 = BYTES_IN_PIB * BYTES_IN_KIB;

    /// Represents different units of data size, allowing for conversion between human-readable
    /// representations and precise byte values.
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(PartialOrd, PartialEq, Clone, Copy, EnumIter)]
    pub enum IECUnit {
        Byte(f64, f64),
        Kibibyte(f64, f64),
        Mebibyte(f64, f64),
        Gibibyte(f64, f64),
        Tebibyte(f64, f64),
        Pebibyte(f64, f64),
        Exbibyte(f64, f64),
        Overflow,
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, EnumIter)]
    pub enum IECSize {
        Byte,
        Kibibyte,
        Mebibyte,
        Gibibyte,
        Tebibyte,
        Pebibyte,
        Exbibyte,
    }

    impl IECUnit {
        pub fn new(value: f64, unit_type: IECSize) -> IECUnit {
            if value == f64::INFINITY || value == f64::NEG_INFINITY || value > f64::MAX {
                return IECUnit::Overflow;
            } else if value.is_sign_negative() || value.is_nan() {
                return IECUnit::default();
            }
            match unit_type {
                IECSize::Byte => IECUnit::Byte(value, value),
                IECSize::Kibibyte => IECUnit::Kibibyte(value, value * BYTES_IN_KIB),
                IECSize::Mebibyte => IECUnit::Mebibyte(value, value * BYTES_IN_MIB),
                IECSize::Gibibyte => IECUnit::Gibibyte(value, value * BYTES_IN_GIB),
                IECSize::Tebibyte => IECUnit::Tebibyte(value, value * BYTES_IN_TIB),
                IECSize::Pebibyte => IECUnit::Pebibyte(value, value * BYTES_IN_PIB),
                IECSize::Exbibyte => IECUnit::Exbibyte(value, value * BYTES_IN_EIB),
            }
        }

        pub fn auto(bytes: f64) -> IECUnit {
            if bytes.is_sign_negative() || bytes.is_nan() {
                return IECUnit::default();
            }
            match bytes {
                b if b == f64::INFINITY || b == f64::NEG_INFINITY || b > f64::MAX => {
                    IECUnit::Overflow
                }
                b if b < BYTES_IN_KIB => IECUnit::Byte(b, b),
                b if b < BYTES_IN_MIB => IECUnit::Kibibyte(b / BYTES_IN_KIB, b),
                b if b < BYTES_IN_GIB => IECUnit::Mebibyte(b / BYTES_IN_MIB, b),
                b if b < BYTES_IN_TIB => IECUnit::Gibibyte(b / BYTES_IN_GIB, b),
                b if b < BYTES_IN_PIB => IECUnit::Tebibyte(b / BYTES_IN_TIB, b),
                b if b < BYTES_IN_EIB => IECUnit::Pebibyte(b / BYTES_IN_PIB, b),
                _ => IECUnit::Exbibyte(bytes / BYTES_IN_EIB, bytes),
            }
        }

        #[cfg(not(tarpaulin_include))]
        pub fn get_values(&self) -> (f64, f64) {
            match self {
                IECUnit::Byte(value_h, value_b)
                | IECUnit::Kibibyte(value_h, value_b)
                | IECUnit::Mebibyte(value_h, value_b)
                | IECUnit::Gibibyte(value_h, value_b)
                | IECUnit::Tebibyte(value_h, value_b)
                | IECUnit::Pebibyte(value_h, value_b)
                | IECUnit::Exbibyte(value_h, value_b) => (*value_h, *value_b),
                IECUnit::Overflow => (f64::INFINITY, f64::INFINITY),
            }
        }
    }

    impl Default for IECUnit {
        fn default() -> Self {
            IECUnit::Byte(0.0, 0.0)
        }
    }

    impl Add for IECUnit {
        type Output = IECUnit;

        fn add(self, other: IECUnit) -> IECUnit {
            if self == IECUnit::Overflow || other == IECUnit::Overflow {
                IECUnit::Overflow
            } else {
                IECUnit::auto(self.get_values().1 + other.get_values().1)
            }
        }
    }

    impl Sub for IECUnit {
        type Output = IECUnit;

        fn sub(self, other: IECUnit) -> IECUnit {
            if self == IECUnit::Overflow || other == IECUnit::Overflow {
                IECUnit::Overflow
            } else {
                IECUnit::auto(self.get_values().1 - other.get_values().1)
            }
        }
    }

    impl Mul<f64> for IECUnit {
        type Output = IECUnit;

        fn mul(self, scalar: f64) -> IECUnit {
            if self == IECUnit::Overflow {
                IECUnit::Overflow
            } else {
                IECUnit::auto(self.get_values().1 * scalar)
            }
        }
    }

    impl Div<f64> for IECUnit {
        type Output = IECUnit;

        fn div(self, divisor: f64) -> IECUnit {
            if self == IECUnit::Overflow {
                IECUnit::Overflow
            } else {
                IECUnit::auto(self.get_values().1 / divisor)
            }
        }
    }

    impl From<ies::IECUnit> for si::SIUnit {
        fn from(iec_unit: ies::IECUnit) -> Self {
            match iec_unit {
                ies::IECUnit::Byte(value_h, value_b) => si::SIUnit::Byte(value_h, value_b),
                ies::IECUnit::Overflow => si::SIUnit::Overflow,
                _ => si::SIUnit::auto(iec_unit.get_values().1),
            }
        }
    }

    impl From<IECUnit> for f64 {
        fn from(data_size_unit: IECUnit) -> Self {
            data_size_unit.get_values().1
        }
    }

    /// Converts an `IECUnit` to a `usize` value.
    impl From<IECUnit> for usize {
        /// Warning: This conversion may result in data loss.
        fn from(data_size_unit: IECUnit) -> Self {
            data_size_unit.get_values().1 as usize
        }
    }

    impl Display for IECUnit {
        #[cfg(not(tarpaulin_include))]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                IECUnit::Byte(_, bytes) => write!(f, "{:.2} B", bytes),
                IECUnit::Kibibyte(kb, _) => write!(f, "{:.2} KiB", kb),
                IECUnit::Mebibyte(mb, _) => write!(f, "{:.2} MiB", mb),
                IECUnit::Gibibyte(gb, _) => write!(f, "{:.2} GiB", gb),
                IECUnit::Tebibyte(tb, _) => write!(f, "{:.2} TiB", tb),
                IECUnit::Pebibyte(pb, _) => write!(f, "{:.2} PiB", pb),
                IECUnit::Exbibyte(eb, _) => write!(f, "{:.2} EiB", eb),
                IECUnit::Overflow => write!(f, "Overflow"),
            }
        }
    }
}
