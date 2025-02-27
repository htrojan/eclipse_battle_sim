use rand::rngs::{StdRng};
use rand::{SeedableRng};
use rand_chacha::ChaCha8Rng;
use wasm_bindgen::prelude::*;

mod simulator;
pub use simulator::{BattleResult, Fleet, Ship, ShipType};

#[cfg(not(target_arch = "wasm32"))]
pub use simulator::{simulate_battle, simulate_round, simulate_battle_bump, simulate_n_battles};
use crate::simulator::WasmFleet;

cfg_if::cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            use log::Level;
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {
            //Init env logger with default logging trace and info values
            env_logger::builder()
                .filter_level(log::LevelFilter::Info)
                .init();

        }
    }

}


#[wasm_bindgen]
pub struct RngState {
    rng_state: ChaCha8Rng,
}

#[wasm_bindgen]
impl RngState {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u64) -> Self {
        let rng_state = ChaCha8Rng::seed_from_u64(seed);
        RngState { rng_state }
    }
}
// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub fn simulate_round(attacker: &mut Fleet, defender: &mut Fleet, rng: &mut RngState) {
//     simulator::simulate_round(attacker, defender, &mut rng.rng_state);
// }

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn simulate_n_battles(
    attacker: &WasmFleet,
    defender: &WasmFleet,
    n: usize,
    rng: &mut RngState,
) -> f32 {
    let bump = bumpalo::Bump::new();
    let attacker = attacker.clone().into_fleet(&bump);
    let defender = defender.clone().into_fleet(&bump);
    simulator::simulate_n_battles(attacker,  defender, &mut rng.rng_state, n, &bump)
}
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn simulate_battle(
    attacker: &WasmFleet,
    defender: &WasmFleet,
    rng: &mut RngState,
) -> BattleResult {
    let bump = bumpalo::Bump::new();
    let mut attacker = attacker.clone().into_fleet(&bump);
    let mut defender = defender.clone().into_fleet(&bump);
    simulator::simulate_battle(&mut attacker, &mut defender, &mut rng.rng_state)
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
