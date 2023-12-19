#[cfg(feature = "size_format")]
mod size_format {
    use get_chunk::data_size_format::{ies_format::*, si_format::*};

    #[test]
    fn auto_si_t_0() {
        let values = [
            BYTES_IN_KB,
            BYTES_IN_MB,
            BYTES_IN_GB,
            BYTES_IN_TB,
            BYTES_IN_PB,
            BYTES_IN_EB,
        ];

        let units = [
            SIUnit::Kilobyte(1.0, BYTES_IN_KB),
            SIUnit::Megabyte(1.0, BYTES_IN_MB),
            SIUnit::Gigabyte(1.0, BYTES_IN_GB),
            SIUnit::Terabyte(1.0, BYTES_IN_TB),
            SIUnit::Petabyte(1.0, BYTES_IN_PB),
            SIUnit::Exabyte(1.0, BYTES_IN_EB),
        ];
        for (bytes, unit) in values.iter().zip(units.iter()) {
            assert_eq!(SIUnit::auto(*bytes), *unit);
        }
    }

    #[test]
    fn auto_iec_t_0() {
        let values = [
            BYTES_IN_KIB,
            BYTES_IN_MIB,
            BYTES_IN_GIB,
            BYTES_IN_TIB,
            BYTES_IN_PIB,
            BYTES_IN_EIB,
        ];

        let units = [
            IECUnit::Kibibyte(1.0, BYTES_IN_KIB),
            IECUnit::Mebibyte(1.0, BYTES_IN_MIB),
            IECUnit::Gibibyte(1.0, BYTES_IN_GIB),
            IECUnit::Tebibyte(1.0, BYTES_IN_TIB),
            IECUnit::Pebibyte(1.0, BYTES_IN_PIB),
            IECUnit::Exbibyte(1.0, BYTES_IN_EIB),
        ];
        for (bytes, unit) in values.iter().zip(units.iter()) {
            assert_eq!(IECUnit::auto(*bytes), *unit);
        }
    }
}
