import {BattleResult, WasmFleet, RngState, simulate_battle, simulate_n_battles} from "simulator";

self.onmessage = (e: MessageEvent) => {
    let rng_state = new RngState(BigInt(e.data.rng_seed));
    const attacker_fleet = WasmFleet.from_json(e.data.attacker_fleet);
    const defender_fleet = WasmFleet.from_json(e.data.defender_fleet);
    console.log("Received attacker fleet: ", attacker_fleet.to_json());
    console.log("Received defender fleet: ", defender_fleet.to_json());
    const n: number = e.data.simulation_steps;


    let win_percent = simulate_n_battles(attacker_fleet, defender_fleet, n, rng_state);
    console.log("Win percent: ", win_percent);

    self.postMessage({defender_win_percent: win_percent});
}

export {};