use eclipse_sim::{BattleResult, Fleet, Ship, ShipType};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let mut rng = rand::thread_rng();
    let defender_fleet = Fleet::new (vec![
        Ship {
            hull: 6,
            initiative: 4,
            shield: 0,
            computer: 2,
            weapon_1_dmg: 3,
            weapon_2_dmg: 0,
            ship_type: ShipType::Interceptor,
        },
            Ship {
                hull: 6,
                initiative: 4,
                shield: 0,
                computer: 2,
                weapon_1_dmg: 3,
                weapon_2_dmg: 0,
                ship_type: ShipType::Interceptor,
            },
        ],);

    let attacker_fleet = Fleet::new(vec![
        Ship {
            hull: 2,
            initiative: 4,
            shield: 0,
            computer: 4,
            weapon_1_dmg: 3,
            weapon_2_dmg: 0,
            ship_type: ShipType::Interceptor,
        },
            Ship {
                hull: 0,
                initiative: 5,
                shield: 0,
                computer: 0,
                weapon_1_dmg: 2,
                weapon_2_dmg: 2,
                ship_type: ShipType::Interceptor,
            },
        Ship {
            hull: 0,
            initiative: 5,
            shield: 0,
            computer: 0,
            weapon_1_dmg: 2,
            weapon_2_dmg: 2,
            ship_type: ShipType::Interceptor,
        },
        ],);
    let mut attacker_wins = 0;
    let n = 1_000_000;
    for _ in 0..n {
        let result = eclipse_sim::simulate_battle(
            &mut attacker_fleet.clone(),
            &mut defender_fleet.clone(),
            &mut rng,
        );
        if result == BattleResult::AttackerWins {
            attacker_wins += 1;
        }
    }
    println!("Result: {:?}", (attacker_wins as f32) / (n as f32));
}

