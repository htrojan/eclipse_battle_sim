use bumpalo::Bump;
use log::info;
use rand::prelude::StdRng;
use rand::SeedableRng;
use eclipse_sim::{simulate_battle, simulate_battle_bump, BattleResult, Fleet, Ship, ShipType};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let bump = Bump::new();
    // init_log();
    let mut rng = StdRng::seed_from_u64(3);
    let ship_proto = Ship {
        hull: 2,
        initiative: 0,
        shield: 1,
        computer: 1,
        weapon_1_dmg: 2,
        weapon_2_dmg: 0,
        ship_type: ShipType::Interceptor,
    };
    let ship_proto_def = Ship {
        hull: 3,
        initiative: 0,
        shield: 1,
        computer: 1,
        weapon_1_dmg: 2,
        weapon_2_dmg: 0,
        ship_type: ShipType::Interceptor,
    };
    let attacker_fleet = Fleet::new(vec!(ship_proto.clone(); 20), &bump);
    let defender_fleet = Fleet::new(vec!(ship_proto_def.clone(); 20), &bump);

    let mut defender_wins = 0;
    let n = 1_000_000;
    for i in 0..n {
        // println!("Simulation {}", i);
        let result = simulate_battle_bump(
            &mut attacker_fleet.clone(),
            &mut defender_fleet.clone(),
            &mut rng,
            &bump,
        );
        if result == BattleResult::DefenderWins {
            defender_wins += 1;
        }
    }
    info!("Result: {:?}", (defender_wins as f32) / (n as f32));
}

