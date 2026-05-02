fn main() {
    let is_on = msi_coolerboost::toggle();
    println!("CoolerBoost: {}", if is_on { "ON" } else { "OFF" });
}
