use criterion::{black_box, criterion_group, criterion_main, Criterion};
use log::info;
use rand::prelude::StdRng;
use rand::SeedableRng;
use eclipse_sim::{simulate_battle, BattleResult, Fleet, Ship, ShipType};

fn benchmark_my_function(c: &mut Criterion) {
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
    let attacker_fleet = Fleet::new(vec!(ship_proto.clone(); 20));
    let defender_fleet = Fleet::new(vec!(ship_proto_def.clone(); 20));

    c.bench_function("simulate_battle", |b| {
        b.iter(|| {
            simulate_battle(
                black_box(&mut attacker_fleet.clone()),
                black_box(&mut defender_fleet.clone()),
                black_box(&mut rng),
            );
        })
    });
}

criterion_group!(benches, benchmark_my_function);
criterion_main!(benches);