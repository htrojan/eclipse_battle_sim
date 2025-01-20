use rand::{Rng, RngCore};
use itertools::Itertools;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod simulator;
pub use simulator::{BattleResult, Fleet, Ship, ShipType};

#[cfg(not(target_arch = "wasm32"))]
pub use simulator::{simulate_round, simulate_battle};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn simulate_round(attacker: &mut Fleet, defender: &mut Fleet) {
    let mut rng = rand::thread_rng();
    simulator::simulate_round(attacker, defender, &mut rng);
}

#[cfg(target_arch = "wasm32")]
pub fn simulate_battle(attacker: &mut Fleet, defender: &mut Fleet) -> BattleResult {
    let mut rng = rand::thread_rng();
    simulator::simulate_battle(attacker, defender, &mut rng)
}

#[cfg(target_arch = "wasm32")]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}