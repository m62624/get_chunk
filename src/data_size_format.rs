use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;

impl From<si_format::SIUnit> for ies_format::IECUnit {
    fn from(si_unit: si_format::SIUnit) -> Self {
        match si_unit {
            si_format::SIUnit::Bytes(value_h, value_b) => {
                ies_format::IECUnit::Bytes(value_h, value_b)
            }
            si_format::SIUnit::Overflow => ies_format::IECUnit::Overflow,
            _ => ies_format::IECUnit::auto(si_unit.get_values().1),
        }
    }
}

impl From<ies_format::IECUnit> for si_format::SIUnit {
    fn from(iec_unit: ies_format::IECUnit) -> Self {
        match iec_unit {
            ies_format::IECUnit::Bytes(value_h, value_b) => {
                si_format::SIUnit::Bytes(value_h, value_b)
            }
            ies_format::IECUnit::Overflow => si_format::SIUnit::Overflow,
            _ => si_format::SIUnit::auto(iec_unit.get_values().1),
        }
    }
}

/// 1000
pub mod si_format {
    use super::*;

    // SI format.
    /// Kilobytes in bytes.
    pub const BYTES_IN_KB: f64 = 1000.0;
    /// Megabytes in bytes.
    pub const BYTES_IN_MB: f64 = BYTES_IN_KB * BYTES_IN_KB;
    /// Gigabytes in bytes.
    pub const BYTES_IN_GB: f64 = BYTES_IN_MB * BYTES_IN_KB;
    /// Terabytes in bytes.
    pub const BYTES_IN_TB: f64 = BYTES_IN_GB * BYTES_IN_KB;
    /// Petabytes in bytes.
    pub const BYTES_IN_PB: f64 = BYTES_IN_TB * BYTES_IN_KB;
    /// Exabytes in bytes.
    pub const BYTES_IN_EB: f64 = BYTES_IN_PB * BYTES_IN_KB;

    /// Represents different units of data size, allowing for conversion between human-readable
    /// representations and precise byte values.
    #[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
    pub enum SIUnit {
        Bytes(f64, f64),
        Kilobytes(f64, f64),
        Megabytes(f64, f64),
        Gigabytes(f64, f64),
        Terabytes(f64, f64),
        Petabytes(f64, f64),
        Exabytes(f64, f64),
        Overflow,
    }

    pub enum SISize {
        Bytes,
        Kilobytes,
        Megabytes,
        Gigabytes,
        Terabytes,
        Petabytes,
        Exabytes,
    }

    impl SIUnit {
        pub fn new(value: f64, unit_type: SISize) -> SIUnit {
            match unit_type {
                SISize::Bytes => SIUnit::Bytes(value, value),
                SISize::Kilobytes => SIUnit::Kilobytes(value, value * BYTES_IN_KB),
                SISize::Megabytes => SIUnit::Megabytes(value, value * BYTES_IN_MB),
                SISize::Gigabytes => SIUnit::Gigabytes(value, value * BYTES_IN_GB),
                SISize::Terabytes => SIUnit::Terabytes(value, value * BYTES_IN_TB),
                SISize::Petabytes => SIUnit::Petabytes(value, value * BYTES_IN_PB),
                SISize::Exabytes => SIUnit::Exabytes(value, value * BYTES_IN_EB),
            }
        }

        pub fn auto(bytes: f64) -> SIUnit {
            if bytes == f64::INFINITY {
                SIUnit::Overflow
            } else if bytes < BYTES_IN_KB {
                SIUnit::Bytes(bytes, bytes)
            } else if bytes < BYTES_IN_MB {
                SIUnit::Kilobytes(bytes / BYTES_IN_KB, bytes)
            } else if bytes < BYTES_IN_GB {
                SIUnit::Megabytes(bytes / BYTES_IN_MB, bytes)
            } else if bytes < BYTES_IN_TB {
                SIUnit::Gigabytes(bytes / BYTES_IN_GB, bytes)
            } else if bytes < BYTES_IN_PB {
                SIUnit::Terabytes(bytes / BYTES_IN_TB, bytes)
            } else if bytes < BYTES_IN_EB {
                SIUnit::Petabytes(bytes / BYTES_IN_PB, bytes)
            } else {
                SIUnit::Exabytes(bytes / BYTES_IN_EB, bytes)
            }
        }

        #[cfg(not(tarpaulin_include))]
        pub fn get_values(&self) -> (f64, f64) {
            match self {
                SIUnit::Bytes(value_h, value_b)
                | SIUnit::Kilobytes(value_h, value_b)
                | SIUnit::Megabytes(value_h, value_b)
                | SIUnit::Gigabytes(value_h, value_b)
                | SIUnit::Terabytes(value_h, value_b)
                | SIUnit::Petabytes(value_h, value_b)
                | SIUnit::Exabytes(value_h, value_b) => (*value_h, *value_b),
                SIUnit::Overflow => (f64::INFINITY, f64::INFINITY),
            }
        }
    }

    impl Default for SIUnit {
        fn default() -> Self {
            SIUnit::Bytes(0.0, 0.0)
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

    impl Eq for SIUnit {}

    #[cfg(not(tarpaulin_include))]
    impl PartialOrd for SIUnit {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for SIUnit {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
        }
    }

    impl From<SIUnit> for f64 {
        fn from(data_size_unit: SIUnit) -> Self {
            data_size_unit.get_values().1
        }
    }

    impl Display for SIUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SIUnit::Bytes(_, bytes) => write!(f, "{:.2} bytes", bytes),
                SIUnit::Kilobytes(kb, _) => write!(f, "{:.2} KB", kb),
                SIUnit::Megabytes(mb, _) => write!(f, "{:.2} MB", mb),
                SIUnit::Gigabytes(gb, _) => write!(f, "{:.2} GB", gb),
                SIUnit::Terabytes(tb, _) => write!(f, "{:.2} TB", tb),
                SIUnit::Petabytes(pb, _) => write!(f, "{:.2} PB", pb),
                SIUnit::Exabytes(eb, _) => write!(f, "{:.2} EB", eb),
                SIUnit::Overflow => write!(f, "Overflow"),
            }
        }
    }
}

/// 1024
pub mod ies_format {
    use super::*;
    // IEC format.
    /// Kibibytes in bytes.
    pub const BYTES_IN_KIB: f64 = 1024.0;
    /// Mebibytes in bytes.
    pub const BYTES_IN_MIB: f64 = BYTES_IN_KIB * BYTES_IN_KIB;
    /// Gibibytes in bytes.
    pub const BYTES_IN_GIB: f64 = BYTES_IN_MIB * BYTES_IN_KIB;
    /// Tebibytes in bytes.
    pub const BYTES_IN_TIB: f64 = BYTES_IN_GIB * BYTES_IN_KIB;
    /// Pebibytes in bytes.
    pub const BYTES_IN_PIB: f64 = BYTES_IN_TIB * BYTES_IN_KIB;
    /// Exbibytes in bytes.
    pub const BYTES_IN_EIB: f64 = BYTES_IN_PIB * BYTES_IN_KIB;

    /// Represents different units of data size, allowing for conversion between human-readable
    /// representations and precise byte values.
    #[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
    pub enum IECUnit {
        Bytes(f64, f64),
        Kibibytes(f64, f64),
        Mibibytes(f64, f64),
        Gibibytes(f64, f64),
        Tebibytes(f64, f64),
        Pebibytes(f64, f64),
        Exbibyte(f64, f64),
        Overflow,
    }

    pub enum IECSize {
        Bytes,
        Kibibytes,
        Mebibytes,
        Gibibytes,
        Tebibytes,
        Pebibytes,
        Exbibyte,
    }

    impl IECUnit {
        pub fn new(value: f64, unit_type: IECSize) -> IECUnit {
            match unit_type {
                IECSize::Bytes => IECUnit::Bytes(value, value),
                IECSize::Kibibytes => IECUnit::Kibibytes(value, value * BYTES_IN_KIB),
                IECSize::Mebibytes => IECUnit::Mibibytes(value, value * BYTES_IN_MIB),
                IECSize::Gibibytes => IECUnit::Gibibytes(value, value * BYTES_IN_GIB),
                IECSize::Tebibytes => IECUnit::Tebibytes(value, value * BYTES_IN_TIB),
                IECSize::Pebibytes => IECUnit::Pebibytes(value, value * BYTES_IN_PIB),
                IECSize::Exbibyte => IECUnit::Exbibyte(value, value * BYTES_IN_EIB),
            }
        }

        pub fn auto(bytes: f64) -> IECUnit {
            if bytes == f64::INFINITY {
                IECUnit::Overflow
            } else if bytes < BYTES_IN_KIB {
                IECUnit::Bytes(bytes, bytes)
            } else if bytes < BYTES_IN_MIB {
                IECUnit::Kibibytes(bytes / BYTES_IN_KIB, bytes)
            } else if bytes < BYTES_IN_GIB {
                IECUnit::Mibibytes(bytes / BYTES_IN_MIB, bytes)
            } else if bytes < BYTES_IN_TIB {
                IECUnit::Gibibytes(bytes / BYTES_IN_GIB, bytes)
            } else if bytes < BYTES_IN_PIB {
                IECUnit::Tebibytes(bytes / BYTES_IN_TIB, bytes)
            } else if bytes < BYTES_IN_EIB {
                IECUnit::Pebibytes(bytes / BYTES_IN_PIB, bytes)
            } else {
                IECUnit::Exbibyte(bytes / BYTES_IN_EIB, bytes)
            }
        }

        #[cfg(not(tarpaulin_include))]
        pub fn get_values(&self) -> (f64, f64) {
            match self {
                IECUnit::Bytes(value_h, value_b)
                | IECUnit::Kibibytes(value_h, value_b)
                | IECUnit::Mibibytes(value_h, value_b)
                | IECUnit::Gibibytes(value_h, value_b)
                | IECUnit::Tebibytes(value_h, value_b)
                | IECUnit::Pebibytes(value_h, value_b)
                | IECUnit::Exbibyte(value_h, value_b) => (*value_h, *value_b),
                IECUnit::Overflow => (f64::INFINITY, f64::INFINITY),
            }
        }
    }

    impl Default for IECUnit {
        fn default() -> Self {
            IECUnit::Bytes(0.0, 0.0)
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

    impl Eq for IECUnit {}

    #[cfg(not(tarpaulin_include))]
    impl PartialOrd for IECUnit {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for IECUnit {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or(Ordering::Equal)
        }
    }

    impl From<IECUnit> for f64 {
        fn from(data_size_unit: IECUnit) -> Self {
            data_size_unit.get_values().1
        }
    }

    impl Display for IECUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                IECUnit::Bytes(_, bytes) => write!(f, "{:.2} bytes", bytes),
                IECUnit::Kibibytes(kb, _) => write!(f, "{:.2} KiB", kb),
                IECUnit::Mibibytes(mb, _) => write!(f, "{:.2} MiB", mb),
                IECUnit::Gibibytes(gb, _) => write!(f, "{:.2} GiB", gb),
                IECUnit::Tebibytes(tb, _) => write!(f, "{:.2} TiB", tb),
                IECUnit::Pebibytes(pb, _) => write!(f, "{:.2} PiB", pb),
                IECUnit::Exbibyte(eb, _) => write!(f, "{:.2} EiB", eb),
                IECUnit::Overflow => write!(f, "Overflow"),
            }
        }
    }
}
