use stripe::{Charge, ChargeId};

fn main() {
    let _ = std::mem::size_of::<Charge>();
    let _ = std::mem::size_of::<ChargeId>();
}
