//! Toggle mode for MSI `CoolerBoost`.
//!
//! Provides the CLI toggle action for the unified `msi-coolerboost` application.

/// Toggles `CoolerBoost` and prints the resulting state to stdout.
///
/// Prints `CoolerBoost: ON` or `CoolerBoost: OFF`.
pub fn run_toggle() {
    let is_enabled: bool = crate::toggle();
    println!("CoolerBoost: {}", if is_enabled { "ON" } else { "OFF" });
}
