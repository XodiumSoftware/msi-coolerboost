//! CLI binary for toggling MSI CoolerBoost fan mode.
//!
//! This binary provides a simple command-line interface for toggling
//! the CoolerBoost feature on MSI laptops via the `isw` tool.
//!
//! # Usage
//! ```
//! msi-coolerboost-toggle
//! ```
//!
//! Or via symlink:
//! ```
//! msi-coolerboost toggle
//! ```

/// Entry point for toggling MSI CoolerBoost from the CLI.
///
/// Calls the shared [`toggle`](msi_coolerboost::toggle) function and prints the
/// resulting state to stdout as either `CoolerBoost: ON` or `CoolerBoost: OFF`.
fn main() {
    let is_enabled: bool = msi_coolerboost::toggle();
    println!("CoolerBoost: {}", if is_enabled { "ON" } else { "OFF" });
}
