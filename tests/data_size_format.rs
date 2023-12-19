#[cfg(feature = "size_format")]
mod size_format {

    mod si {
        use get_chunk::data_size_format::si::*;

        #[test]
        fn auto_t_0() {
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
        fn new_t_0() {
            assert_eq!(SIUnit::new(1.0, SISize::Byte), SIUnit::Byte(1.0, 1.0));

            assert_eq!(
                SIUnit::new(21.0, SISize::Kilobyte),
                SIUnit::Kilobyte(21.0, 21_000.0)
            );

            assert_eq!(
                SIUnit::new(57.0, SISize::Megabyte),
                SIUnit::Megabyte(57.0, 57_000_000.0)
            );

            assert_eq!(
                SIUnit::new(100.0, SISize::Gigabyte),
                SIUnit::Gigabyte(100.0, 100_000_000_000.0)
            );

            assert_eq!(
                SIUnit::new(1203.0, SISize::Terabyte),
                SIUnit::Terabyte(1203.0, 1_203_000_000_000_000.0)
            );

            assert_eq!(
                SIUnit::new(14.0, SISize::Petabyte),
                SIUnit::Petabyte(14.0, 14_000_000_000_000_000.0)
            );

            assert_eq!(
                SIUnit::new(1.0, SISize::Exabyte),
                SIUnit::Exabyte(1.0, 1_000_000_000_000_000_000.0)
            );
        }

        mod ops {
            use super::*;
            use get_chunk::data_size_format::IntoEnumIterator;

            #[test]
            fn add_t_0() {
                let mut next_size = SISize::iter();
                next_size.next();
                for (prev, next) in SISize::iter().zip(next_size) {
                    assert_eq!(
                        SIUnit::new(500.0, prev) + SIUnit::new(500.0, prev),
                        SIUnit::new(1.0, next)
                    );
                }
            }

            #[test]
            fn sub_t_0() {
                let mut next_size = SISize::iter();
                next_size.next();
                for (prev, next) in SISize::iter().zip(next_size) {
                    assert_eq!(
                        SIUnit::new(1.0, next) - SIUnit::new(100.0, prev),
                        SIUnit::new(900.0, prev)
                    );
                }
            }

            #[test]
            fn mul_t_0() {
                let mut next_size = SISize::iter();
                next_size.next();
                for (prev, next) in SISize::iter().zip(next_size) {
                    assert_eq!(SIUnit::new(500.0, prev) * 2.0, SIUnit::new(1.0, next));
                }
            }

            #[test]
            fn div_t_0() {
                let mut next_size = SISize::iter();
                next_size.next();
                for (prev, next) in SISize::iter().zip(next_size) {
                    assert_eq!(SIUnit::new(1.0, next) / 2.0, SIUnit::new(500.0, prev));
                }
            }
        }
    }

    mod iec {
        use get_chunk::data_size_format::ies::*;

        #[test]
        fn auto_t_0() {
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

        #[test]
        fn new_t_0() {
            assert_eq!(IECUnit::new(1.0, IECSize::Byte), IECUnit::Byte(1.0, 1.0));

            assert_eq!(
                IECUnit::new(21.0, IECSize::Kibibyte),
                IECUnit::Kibibyte(21.0, 21_504.0)
            );

            assert_eq!(
                IECUnit::new(57.0, IECSize::Mebibyte),
                IECUnit::Mebibyte(57.0, 59_768_832.0)
            );

            assert_eq!(
                IECUnit::new(100.0, IECSize::Gibibyte),
                IECUnit::Gibibyte(100.0, 107_374_182_400.0)
            );

            assert_eq!(
                IECUnit::new(1203.0, IECSize::Tebibyte),
                IECUnit::Tebibyte(1203.0, 1_322_712_488_214_528.0)
            );

            assert_eq!(
                IECUnit::new(14.0, IECSize::Pebibyte),
                IECUnit::Pebibyte(14.0, 15_762_598_695_796_736.0)
            );

            assert_eq!(
                IECUnit::new(1.0, IECSize::Exbibyte),
                IECUnit::Exbibyte(1.0, 1_152_921_504_606_846_976.0)
            );
        }

        mod ops {
            use super::*;
            use get_chunk::data_size_format::IntoEnumIterator;

            #[test]
            fn add_t_0() {
                let mut next_size = IECSize::iter();
                next_size.next();
                for (prev, next) in IECSize::iter().zip(next_size) {
                    assert_eq!(
                        IECUnit::new(512.0, prev) + IECUnit::new(512.0, prev),
                        IECUnit::new(1.0, next)
                    );
                }
            }

            #[test]
            fn sub_t_0() {
                let mut next_size = IECSize::iter();
                next_size.next();
                for (prev, next) in IECSize::iter().zip(next_size) {
                    assert_eq!(
                        IECUnit::new(1.0, next) - IECUnit::new(124.0, prev),
                        IECUnit::new(900.0, prev)
                    );
                }
            }

            #[test]
            fn mul_t_0() {
                let mut next_size = IECSize::iter();
                next_size.next();
                for (prev, next) in IECSize::iter().zip(next_size) {
                    assert_eq!(IECUnit::new(512.0, prev) * 2.0, IECUnit::new(1.0, next));
                }
            }

            #[test]
            fn div_t_0() {
                let mut next_size = IECSize::iter();
                next_size.next();
                for (prev, next) in IECSize::iter().zip(next_size) {
                    assert_eq!(IECUnit::new(1.0, next) / 2.0, IECUnit::new(512.0, prev));
                }
            }
        }
    }
}
