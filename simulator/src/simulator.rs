use std::cmp::Ordering;
use bumpalo::Bump;
use rand::{Rng, RngCore};
use itertools::Itertools;
use bumpalo::collections::Vec as BumpVec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(debug_assertions)]
macro_rules! info {
    ($($arg: tt)*) => { log::info!($($arg)*) }
}

#[cfg(not(debug_assertions))]
macro_rules! info {
    ($($arg: tt)*) => { }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ship {
    pub hull: i32,
    pub initiative: i32,
    pub shield: i32,
    pub computer: i32,
    pub weapon_1_dmg: i32,
    pub weapon_2_dmg: i32,
    pub ship_type: ShipType,
}

#[wasm_bindgen]
impl Ship {
    pub fn clone(&self) -> Ship {
        Ship {
            hull: self.hull,
            initiative: self.initiative,
            shield: self.shield,
            computer: self.computer,
            weapon_1_dmg: self.weapon_1_dmg,
            weapon_2_dmg: self.weapon_2_dmg,
            ship_type: self.ship_type,
        }
    }
}

#[wasm_bindgen]
impl Ship {
    #[wasm_bindgen(constructor)]
    pub fn new(hull: i32, initiative: i32, shield: i32, computer: i32, weapon_1_dmg: i32, weapon_2_dmg: i32, ship_type: ShipType) -> Ship {
        Ship {
            hull,
            initiative,
            shield,
            computer,
            weapon_1_dmg,
            weapon_2_dmg,
            ship_type,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ShipType {
    Interceptor,
    Cruiser,
    Dreadnought,
    Starbase,
}

impl Ship {
    fn get_damage_index(&self) -> f32 {
        (self.weapon_1_dmg + self.weapon_2_dmg) as f32 * (1. + self.computer as f32 / 6.).min(1.)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone,  Serialize, Deserialize)]
// #[wasm_bindgen(js_name = Fleet)]
pub struct WasmFleet{
    ships: Vec<Ship>,
}

impl WasmFleet {
    pub fn into_fleet(self, bump: &Bump) -> Fleet {
        Fleet::new(self.ships, &bump)
    }
}

#[wasm_bindgen]
impl WasmFleet {
    #[wasm_bindgen(constructor)]
    pub fn new(ships: Vec<Ship>) -> WasmFleet {
        // Sort ships by initiative at creation time.
        WasmFleet {
            ships
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(json: &str) -> WasmFleet {
        serde_json::from_str(json).unwrap()
    }
}

/// A fleet is a collection of ships
/// The ships are sorted by initiative
#[derive(Debug, Clone)]
pub struct Fleet<'a> {
    ships: BumpVec<'a, Ship>,
}

impl<'a> Fleet<'a> {
    pub fn new<T: IntoIterator<Item=Ship>>(ships: T, bump: &'a Bump) -> Fleet<'a> {
        // Sort ships by initiative at creation time.
        let mut ships = BumpVec::from_iter_in(ships, bump);
        ships.sort_by(|a, b| a.initiative.cmp(&b.initiative).reverse());
        Fleet {
            ships
        }
    }

    pub fn clone(&self) -> Fleet {
        Fleet {
            ships: self.ships.clone()
        }
    }


    pub fn has_ships_left(&self) -> bool {
        self.ships.iter().any(|ship| ship.hull >= 0)
    }

    pub fn num_ships(&self) -> usize {
        self.ships.iter().filter(|ship| ship.hull >= 0).count()
    }
    /// Returns the attack order of the ships in the fleet
    /// The attack order is determined by the initiative of the ships
    /// The format is (index_in_fleet, initiative)
    /// The index is a usize corresponding to the index the ship is stored in the ship vector
    #[inline]
    fn get_attack_order_max_init(&self, min_init: i32) -> impl Iterator<Item=InitiativeIndex> + use < '_ > {
        // Sort so that the highest initiative is first
        let ships = self.ships
            .iter()
            .enumerate()
            .filter(|(_, x)| x.hull >= 0)
            .filter(move |(_, x)| x.initiative <= min_init as i32)
            .map(|(index, x)| InitiativeIndex {
                index,
                initiative: x.initiative,
            });

        ships
    }

    fn get_attack_order(&self) -> impl Iterator<Item=InitiativeIndex> + use < '_ > {
        // Sort so that the highest initiative is first
        let ships = self.ships
            .iter()
            .enumerate()
            .filter(|(_, x)| x.hull >= 0)
            .map(|(index, x)| InitiativeIndex {
                index,
                initiative: x.initiative,
            });

        ships
    }

}

/// A struct to store the index of a ship in a fleet and its initiative
/// Used for sorting the ships by initiative
#[derive(Debug)]
struct InitiativeIndex {
    index: usize,
    initiative: i32,
}

struct AverageDamageIndex {
    index: usize,
    damage: f32,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResult {
    AttackerWins,
    DefenderWins,
    Draw,
}


pub fn simulate_battle<T: RngCore + Clone>(
    attacker: &mut Fleet,
    defender: &mut Fleet,
    rng: &mut T,
) -> BattleResult {
    let bump = Bump::new();
    simulate_battle_bump(attacker, defender, rng, &bump)
}

pub fn simulate_n_battles<T: RngCore + Clone>(
    attacker: Fleet,
    defender: Fleet,
    rng: &mut T,
    n: usize,
    bump: &Bump
) -> f32 {
    let mut attacker_wins = 0;
    let mut defender_wins = 0;
    let mut draws = 0;
    for _ in 0..n {
        let result = simulate_battle_bump(&mut attacker.clone(), &mut defender.clone(), rng, &bump);
        match result {
            BattleResult::AttackerWins => {
                attacker_wins += 1;
            }
            BattleResult::DefenderWins => {
                defender_wins += 1;
            }
            BattleResult::Draw => {
                draws += 1;
            }
        }
    }
    defender_wins as f32 / n as f32
}
pub fn simulate_battle_bump<T: RngCore + Clone>(
    attacker: &mut Fleet,
    defender: &mut Fleet,
    rng: &mut T,
    bump: &Bump
) -> BattleResult {
    // let bump = Bump::new();
    // let mut rng = rand::thread_rng();
    while attacker.has_ships_left() && defender.has_ships_left() {
        simulate_round_bump(attacker, defender, rng, &bump);
    }
    if !attacker.has_ships_left() && !defender.has_ships_left() {
        //todo: This should not happen, since the loop should terminate once one player does not have any ships left. Implement better error handling
        BattleResult::Draw
    } else if !attacker.has_ships_left() {
        BattleResult::DefenderWins
    } else {
        BattleResult::AttackerWins
    }
}


pub fn simulate_round<T: RngCore + Clone>(attacker: &mut Fleet, defender: &mut Fleet, rng: &mut T) {
    let bump = Bump::new();
    simulate_round_bump(attacker, defender, rng, &bump);
}

pub fn simulate_round_bump<T: RngCore + Clone>(attacker: &mut Fleet, defender: &mut Fleet, rng: &mut T, bump: &Bump) {
    info!("New Simulation round: \n");

    if !attacker.has_ships_left() || !defender.has_ships_left() {
        return;
    }
    let (mut best_attack_init, mut best_defend_init) = {
        (attacker.get_attack_order().next().unwrap().initiative, defender.get_attack_order().next().unwrap().initiative)
    };

    // Attack while there are ships left in both fleets and at least one opponent has ships that have not attacked yet
    // If an opponent has no ships left, his best_init value is set to -1
    while attacker.has_ships_left()
        && defender.has_ships_left()
        && (best_attack_init >= 0 || best_defend_init >= 0)
    {
        info!("New InitRound: Best attack init: {:?}, best defend init: {:?}", best_attack_init, best_defend_init);
        // Compare who has the highest initiative. In case of a draw, the defender (option two) attacks first

        // If the attacker has greater initiative, the attacker attacks first,
        // otherwise (so if draw), the defender attacks first
        if best_attack_init > best_defend_init {
            // Build a pool of all ships that attack at the same time

            let mut attacker_order = attacker.get_attack_order_max_init(best_attack_init).peekable();
            info!("Attacker attacker with init: {:?}", attacker_order.peek());
            let mut pool = AttackPool::new_in(&bump);
            while attacker_order.peek().is_some()
                && attacker_order.peek().unwrap().initiative > best_defend_init
            {
                pool.add_ship(
                    attacker.ships[attacker_order.next().unwrap().index].clone(),
                    rng,
                );
            }
            best_attack_init = match attacker_order.peek() {
                Some(x) => x.initiative,
                // Temp value, since the attacker has no ships left.
                // The loop will be terminated next iteration and this value will  never be used
                None => -1,
            };

            pool.attack_fleet(defender, bump);
        } else {
            // The defender attacks first
            let mut pool = AttackPool::new_in(&bump);

            // Peek forward to the ship with the correct initiative
            let mut defender_order = defender.get_attack_order_max_init(best_defend_init).peekable();
            info!("Defender attacker with init: {:?}", defender_order.peek());

            while defender_order.peek().is_some()
                && defender_order.peek().unwrap().initiative >= best_attack_init
            {
                pool.add_ship(
                    defender.ships[defender_order.next().unwrap().index].clone(),
                    rng,
                );
            }
            best_defend_init = match defender_order.peek() {
                Some(x) => x.initiative,
                // Temp value, since the defender has no ships left.
                // The loop will be terminated next iteration and this value will  never be used
                None => -1,
            };

            pool.attack_fleet(attacker, &bump);
        }
    }
}

/// Describes the amount of attacks from ships that happen at the same time
#[derive(Debug)]
struct AttackPool<'a> {
    /// The attack rolls of each ship in the pool, enhanced by the ships computer stat
    enhanced_rolls: BumpVec<'a, AttackRoll>,
}

#[derive(Debug)]
struct AttackRoll {
    damage: i32,
    hit_dc: i32,
}

impl<'a> AttackPool<'a> {
    /// Takes the number of ships that generate the attacks
    /// in order to preallocate the necessary space for the attack rolls
    fn new_in(bump: &'a Bump) -> AttackPool<'a> {
        AttackPool {
            // Each ship has two weapons, so the number of attack rolls is twice the number of ships
            enhanced_rolls: BumpVec::new_in(bump)
        }
    }

    #[inline]
    fn add_ship<T: RngCore>(&mut self, ship: Ship, rng: &mut T) {
        // let roll = rng.gen_range(1..=6) + ship.computer;
        let roll = rng.gen_range(1..=6);
        let roll = match roll {
            6 => 1_000_000,
            1 => -1_000_000,
            _ => roll + ship.computer,
        };
        self.enhanced_rolls.push(AttackRoll {
            damage: ship.weapon_1_dmg,
            hit_dc: roll,
        });
        if ship.weapon_2_dmg > 0 {
            let roll = rng.gen_range(1..=6);
            let roll = match roll {
                6 => 1_000_000,
                1 => -1_000_000,
                _ => roll + ship.computer,
            };
            self.enhanced_rolls.push(AttackRoll {
                damage: ship.weapon_2_dmg,
                hit_dc: roll,
            });
        }
    }

    fn compare_ship_damage(a: &Ship, b: &Ship) -> Ordering {
            let c = a.get_damage_index()
                .partial_cmp(&b.get_damage_index())
                // This is well-defined because the damage index is always between 0 and 1
                .unwrap();
            match c {
                Ordering::Equal => {
                    a.hull.cmp(&b.hull)
                }
                _ => c.reverse()
            }
    }

    fn attack_fleet(&self, fleet: &mut Fleet, bump: &Bump) {
        // info!("Attacking fleet: {:?}", opposing_fleet);

        let mut hit_graph = HitGraph::new(self.enhanced_rolls.len(), fleet.ships.len(), &bump);

        // Build hit graph
        for i in 0..self.enhanced_rolls.len() {
            fleet.ships
                .iter()
                .enumerate()
                // The attack hits if the roll is greater than 6 (shield and computer values ignored)
                .filter(|(_, ship)| self.enhanced_rolls[i].hit_dc >= ship.shield + 6)
                .filter(|(_, ship)| ship.hull >= 0)
                .map(|(i, _)| i)
                .for_each(|j| {
                    hit_graph.add_edge(i, j, self.enhanced_rolls[i].damage as u32);
                });
        }

        // For now just use a greedy approach. The ship with the highest damage index is destroyed first
        while hit_graph.has_active_edges() {
            let total_damage = hit_graph.total_possible_damage_per_ship();

            // Loop invariant:
            // IMPORTANT: The ships are sorted by damage index, so the ship with the highest damage index is first in the list
            let targeted_ship = fleet.ships
                .iter()
                .enumerate()
                // The ship is still alive. Ships that are destroyed have a hull of < 0
                // .filter(|(i, _)| ships[*i].hull >= 0)
                .filter(|(i, _)| total_damage[*i] > fleet.ships[*i].hull as u32)
                .max_by(|(i, a), (j, b)| {
                    Self::compare_ship_damage(a, b)
                })
                .map(|(i, _)| i);

            // There is a ship that can be destroyed. Since the list is sorted, the first element is the one with the highest damage index
            if let Some(ship_index) = targeted_ship {
                // Need one more damage as the hull value to destroy the ship
                hit_graph.deactivate_all_rolls_attacking_max_dmg(
                    ship_index,
                    fleet.ships[ship_index].hull as u32 + 1,
                );
                hit_graph.deactivate_all_edges_to_ship(ship_index);
                fleet.ships[ship_index].hull = -1;
                info!("Destroyed ship: {:?}", ship_index);
                // println!("New hit graph: {:?}", hit_graph);
            } else {
                // No ship can be destroyed. The ship with the highest damage index is attacked
                let ship_index = fleet.ships
                    .iter()
                    .enumerate()
                    // The ship is still alive. Ships that are destroyed have a hull of < 0
                    .filter(|(i, _)| fleet.ships[*i].hull >= 0)
                    .filter(|(i, _)| total_damage[*i] > 0)
                    .max_by(|(i, a), (j, b)| {
                        Self::compare_ship_damage(a, b)
                    })
                    .unwrap()
                    .0;
                let total_damage = hit_graph.get_total_possible_damage_to_ship(ship_index);

                fleet.ships[ship_index].hull -= total_damage as i32;
                info!("Damaged ship: {:?} with {} damage", ship_index, total_damage);
                hit_graph.deactivate_all_rolls_attacking(ship_index);
            }
        }

        // opposing_fleet.ships = ships;
    }
}

/// Stores information about which ship can hit which other ship (so which damage roll succeeded)
/// and how much damage a hit would deal
#[derive(Debug)]
struct HitGraph<'a> {
    edges: BumpVec<'a, HitEdge>,
    num_attack_rolls: usize,
    num_ships: usize,
}

impl<'a> HitGraph<'a> {
    fn has_active_edges(&self) -> bool {
        !self.edges.is_empty() && self.edges.iter().any(|edge| edge.active)
    }
    fn add_edge(&mut self, from: usize, to: usize, damage: u32) {
        self.edges.push(HitEdge {
            from,
            to,
            damage,
            active: true,
        });
    }

    fn new(num_attack_rolls: usize, num_ships: usize, bump: &'a Bump) -> HitGraph {
        HitGraph {
            // Just a estimate based on nothing, still decreases total allocation time.
            edges: BumpVec::new_in(bump),
            num_attack_rolls,
            num_ships,
        }
    }

    fn total_possible_damage_per_ship(&self) -> Vec<u32> {
        let mut total_damage = vec![0; self.num_ships];
        for edge in self.edges.iter().filter(|edge| edge.active) {
            total_damage[edge.to] += edge.damage;
        }
        total_damage
    }

    fn activate_all_edges(&mut self) {
        for edge in self.edges.iter_mut() {
            edge.active = true;
        }
    }

    fn deactivate_all_edges_to_ship(&mut self, ship_index: usize) {
        for edge in self.edges.iter_mut().filter(|edge| edge.to == ship_index) {
            edge.active = false;
        }
    }

    /// Deactivates all graph edges originating from ships attacking the ship with the given index
    fn deactivate_all_rolls_attacking(&mut self, ship_index: usize) {
        let affected_ships = self
            .edges
            .iter()
            .filter(|edge| edge.to == ship_index)
            .map(|edge| edge.from)
            .unique()
            .collect::<Vec<usize>>();

        for ship in affected_ships {
            for edge in self.edges.iter_mut().filter(|edge| edge.from == ship) {
                edge.active = false;
            }
        }
    }

    fn deactivate_all_edges_from_attack_roll(&mut self, index: usize) {
        // println!("Deactivate all edges from attack roll {:?}", index);
        for edge in self.edges.iter_mut().filter(|edge| edge.from == index) {
            edge.active = false;
        }
    }

    /// Deactivates all edges originating from attack rolls targeting the ship with the given index,
    /// where the damage originating from all deactivated attacking ships is only 'damage_needed'
    /// The minimum amount of damage needed is determined by a greedy algorithm that deactivates
    /// the ship with the highest damage first
    /// This may not be the optimal solution in all cases.
    fn deactivate_all_rolls_attacking_max_dmg(&mut self, ship_index: usize, damage_needed: u32) {
        // println!("Deactivate edges to ship {:?}. Damage needed: {}", ship_index, damage_needed);
        let total_damage = self.get_total_possible_damage_to_ship(ship_index);

        if total_damage < damage_needed {
            // Error, should not happen!
            panic!("Not enough damage to destroy ship");
        }
        if total_damage == damage_needed {
            // println!("Exactly enough damage to destroy ship. Deactivate all edges to {:?}", ship_index);
            // Deactivate all edges to the ship
            self.deactivate_all_rolls_attacking(ship_index);
            return;
        }

        // Use a greedy algorithm to deactivate the minimum amount of edges
        // Use the edge with the highest damage first.
        // Since the fleet attacking algorithm makes sure, the highest damage ship is destroyed first,
        // this should be the best approach (since the highes damage ships generally have the highest hull value)
        let mut damage_needed: i32 = damage_needed as i32;
        info!("Needing damage: {:?}", damage_needed);
        while damage_needed > 0 {
            let edge_to_deactivate = self
                .edges
                .iter_mut()
                .filter(|edge| edge.active)
                .filter(|edge| edge.to == ship_index)
                // .filter(|edge| edge.damage <= damage_needed)
                .max_by(|a, b| a.damage.cmp(&b.damage));

            if edge_to_deactivate.is_some() {
                let edge_to_deactivate = edge_to_deactivate.unwrap();
                // println!("edge_to_deactivate: {:?}", edge_to_deactivate);
                // println!("Deactivate edge from {:?} to {:?} with damage {:?}", edge_to_deactivate.from, edge_to_deactivate.to, edge_to_deactivate.damage);
                // edge_to_deactivate.active = false;
                damage_needed -= edge_to_deactivate.damage as i32;

                let from_index = edge_to_deactivate.from;
                self.deactivate_all_edges_from_attack_roll(from_index);

                // break;
            } else {
                // Error, should not happen since the calling algorithm ensures that the ship can be destroyed
                panic!("Not enough damage to destroy ship");
            }
        }
        info!("Used damage: {:?}", damage_needed);
    }

    fn get_total_possible_damage_to_ship(&self, ship_index: usize) -> u32 {
        self.edges
            .iter()
            .filter(|edge| edge.active)
            .filter(|edge| edge.to == ship_index)
            .map(|edge| edge.damage)
            .sum()
    }
}

#[derive(Clone, Debug)]
struct HitEdge {
    from: usize,
    to: usize,
    damage: u32,
    /// An edge can be deactivated if the ship that would be hit is already destroyed.
    /// Instead of removing the edge from the graph, the active flag is set to false
    /// in order to avoid changing the graph structure
    active: bool,
}

#[cfg(test)]
mod tests {
    use rand::prelude::StdRng;
    use rand::SeedableRng;
    use crate::init_log;
    use crate::simulator::{simulate_battle, simulate_round, BattleResult, Fleet, Ship, ShipType};

    #[test]
    pub fn test_fleet_attack() {
        let bump = bumpalo::Bump::new();
        let mut attacker = Fleet::new ( vec![Ship {
                hull: 0,
                initiative: 10,
                shield: 0,
                computer: 5,
                weapon_1_dmg: 2,
                weapon_2_dmg: 0,
                ship_type: ShipType::Interceptor,
            }],&bump);
        let mut defender = Fleet::new ( vec![Ship {
                hull: 0,
                initiative: 0,
                shield: 0,
                computer: 5,
                weapon_1_dmg: 2,
                weapon_2_dmg: 0,
                ship_type: ShipType::Interceptor,
            }],&bump);
        // Create a SEEDED RNG

        let mut seeded_rng = rand::rngs::StdRng::seed_from_u64(0);

        while attacker.has_ships_left() && defender.has_ships_left() {
            // println!("New Round: \n");
            simulate_round(&mut attacker, &mut defender, &mut seeded_rng);
        }
        // simulate_round(&mut attacker, &mut defender, seeded_rng);
        assert!(!defender.has_ships_left());
    }

    #[test]
    pub fn test2() {
        // env_logger::builder()
        //     .filter_level(log::LevelFilter::Info)
        //     .init();
        let bump = bumpalo::Bump::new();
        let mut rng = StdRng::seed_from_u64(0);
        let attacker_fleet = Fleet::new ( vec![
                Ship {
                    hull: 1,
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
            ],&bump);

        let defender_fleet = Fleet::new ( vec![
                Ship {
                    hull: 2,
                    initiative: 1,
                    shield: 0,
                    computer: 0,
                    weapon_1_dmg: 2,
                    weapon_2_dmg: 0,
                    ship_type: ShipType::Interceptor,
                },
            ],&bump);
        let mut defender_wins = 0;
        for i in 0..10 {
            // println!("Simulation {}", i);
            let result = simulate_battle(
                &mut attacker_fleet.clone(),
                &mut defender_fleet.clone(),
                &mut rng,
            );
            if result == BattleResult::DefenderWins {
                defender_wins += 1;
            }
        }
        info!("Result: {:?}", (defender_wins as f32) / (1000.0));
    }

    #[test]
    pub fn test_hull_effect() {
        init_log();
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
        let bump = bumpalo::Bump::new();
        let attacker_fleet = Fleet::new ( vec![ship_proto.clone(); 5], &bump );
        let defender_fleet = Fleet::new ( vec![ship_proto_def.clone(); 5], &bump);

        let mut defender_wins = 0;
        let n = 1;
        for i in 0..n {
            // println!("Simulation {}", i);
            let result = simulate_battle(
                &mut attacker_fleet.clone(),
                &mut defender_fleet.clone(),
                &mut rng,
            );
            if result == BattleResult::DefenderWins {
                defender_wins += 1;
            }
        }
        info!("Result: {:?}", (defender_wins as f32) / (n as f32));
    }
}