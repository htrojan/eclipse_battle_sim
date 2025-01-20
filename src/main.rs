use eclipse_sim::{BattleResult, Fleet, Ship, ShipType};

fn main() {
    let mut rng = rand::thread_rng();
    let attacker_fleet = Fleet::new (vec![
            Ship {
                hull: 2,
                initiative: 1,
                shield: 0,
                computer: 1,
                weapon_1_dmg: 2,
                weapon_2_dmg: 0,
                ship_type: ShipType::Interceptor,
            },
            Ship {
                hull: 2,
                initiative: 1,
                shield: 0,
                computer: 1,
                weapon_1_dmg: 2,
                weapon_2_dmg: 0,
                ship_type: ShipType::Interceptor,
            },
        ],);

    let defender_fleet = Fleet::new(vec![
            Ship {
                hull: 2,
                initiative: 0,
                shield: 0,
                computer: 2,
                weapon_1_dmg: 1,
                weapon_2_dmg: 0,
                ship_type: ShipType::Interceptor,
            },
            Ship {
                hull: 2,
                initiative: 0,
                shield: 0,
                computer: 1,
                weapon_1_dmg: 2,
                weapon_2_dmg: 0,
                ship_type: ShipType::Interceptor,
            },
        ],);
    let mut defender_wins = 0;
    for _ in 0..1000 {
        let result = eclipse_sim::simulate_battle(
            &mut attacker_fleet.clone(),
            &mut defender_fleet.clone(),
            &mut rng,
        );
        if result == BattleResult::DefenderWins {
            defender_wins += 1;
        }
    }
    println!("Result: {:?}", (defender_wins as f32) / (1000.0));
}

