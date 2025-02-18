use bumpalo::Bump;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use eclipse_sim::{simulate_battle_bump, BattleResult, Fleet, Ship, ShipType};

fn benchmark_my_function(c: &mut Criterion) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(3);
    // rand_chacha::ChaChaRng::seed_from_u64(3);
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

    let mut bump = Bump::new();
    let attacker_fleet = Fleet::new(vec!(ship_proto.clone(); 20), &bump);
    let defender_fleet = Fleet::new(vec!(ship_proto_def.clone(); 20), &bump);
    
    c.bench_function("simulate_battle", |b| {
        let mut bump = Bump::new();
        b.iter(|| {
            simulate_battle_bump(
                black_box(&mut attacker_fleet.clone()),
                black_box(&mut defender_fleet.clone()),
                black_box(&mut rng),
                &bump
            );
            bump.reset()
        })
    });
}

criterion_group!(benches, benchmark_my_function);
criterion_main!(benches);