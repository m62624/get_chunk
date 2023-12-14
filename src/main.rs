use get_chunk::data_size_format::ies_format::*;
use get_chunk::data_size_format::si_format::*;
fn main() {
    let size_iec: IECUnit = IECUnit::new(95.367431640625, IECSize::Mebibytes);
    println!("IEC {}", size_iec);
    let size_si: SIUnit = size_iec.into();
    println!("IEC {}", size_si);
}
