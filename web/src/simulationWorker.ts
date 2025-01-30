// import {BattleResult, Fleet, RngState, simulate_battle} from "../../simulator/pkg";

self.onmessage = (e: MessageEvent) => {
    import("../../simulator/pkg").then((module) => {
        const {BattleResult, Fleet, RngState, simulate_battle} = module;
        let rng_state = new RngState(BigInt(e.data.seed));
        const attacker_fleet = Fleet.from_json(e.data.attacker_fleet);
        const defender_fleet = Fleet.from_json(e.data.defender_fleet);
        const n: number = e.data.simulation_steps;

        let defender_wins: number = 0;

        for (let i = 0; i < n; i++) {
            let new_fleet = attacker_fleet.clone();
            let new_defender_fleet = defender_fleet.clone();
            let result = simulate_battle(new_fleet, new_defender_fleet, rng_state);
            if (result === BattleResult.DefenderWins) {
                defender_wins += 1;
            }
            new_fleet.free();
            new_defender_fleet.free();
        }
        let win_percent = defender_wins / n;
        console.log("Defender wins: ", defender_wins);
        console.log("Win percent: ", win_percent);

        // self.postMessage("Worker Hello")
        // Post the result back to the main thread
        self.postMessage({defender_win_percent: win_percent});
    })
}

// export {};