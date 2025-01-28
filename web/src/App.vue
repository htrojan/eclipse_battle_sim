<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import TheWelcome from './components/TheWelcome.vue'
import {BattleResult, Fleet, RngState, Ship, ShipType, simulate_battle} from '../../simulator/pkg'
import ShipDisplay from "@/components/ShipDisplay.vue";
import {ref} from "vue";


interface ShipDescription {
  shipCount: number;
  shipName: string;
  ship: Ship;
}

const defender_win_percent = ref(0);
const simulation_steps = ref(100_000);

const attacker_ships = ref(
    [
      {shipCount: 1, shipName: "Dreadnought", ship: new Ship(2, 0, 1, 1, 2, 0, ShipType.Dreadnought)},
      {shipCount: 0, shipName: "Cruiser", ship: new Ship(2, 0, 1, 1, 2, 0, ShipType.Cruiser)},
      {shipCount: 0, shipName: "Interceptor", ship: new Ship(2, 0, 1, 1, 2, 0, ShipType.Interceptor)},
    ]
)

const defender_ships = ref(
    [
      {shipCount: 1, shipName: "Dreadnought", ship: new Ship(2, 0, 1, 1, 2, 0, ShipType.Dreadnought)},
      {shipCount: 0, shipName: "Cruiser", ship: new Ship(2, 0, 1, 1, 2, 0, ShipType.Cruiser)},
      {shipCount: 0, shipName: "Interceptor", ship: new Ship(2, 0, 1, 1, 2, 0, ShipType.Interceptor)},
    ]
)

function simulate_battle_js() {
  console.log("Simulating battle");
  let defender_wins: number = 0;
  let n: number = simulation_steps.value;
  // Always use the same seed for the RNG so that the results are reproducible
  let rng_state = new RngState(BigInt(42));
  let attacker: Ship[] = [];
  let defender: Ship[] = [];
  for (let ship of attacker_ships.value) {
    for (let i = 0; i < ship.shipCount; i++) {
      attacker.push(ship.ship.clone());
    }
  }

  for (let ship of defender_ships.value) {
    for (let i = 0; i < ship.shipCount; i++) {
      defender.push(ship.ship.clone());
    }
  }

  let attacker_fleet = new Fleet(attacker);
  let defender_fleet = new Fleet(defender);
  console.log("Attacker fleet: ", attacker_fleet);
  console.log("Defender fleet: ", defender_fleet);
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
  defender_win_percent.value = defender_wins / n;
  console.log("Defender wins: ", defender_wins);
}


// console.log("simulating with attacker", attacker, "defender", defender);
//   let result = simulate_battle(attacker, defender, rng_state);
//   // console.log(i, result);
//   if (result === BattleResult.DefenderWins ){
//     defender_wins += 1;
//   }
// }
// console.log(defender_wins);
</script>

<template>
  <header>
  </header>
  <body>
  <main>
    <div class="flex flex-col justify-center md:flex-row md:justify-normal ">
      <div class="flex flex-col items-center">
        <div class="text-center text-lg">Attacker</div>
        <ShipDisplay v-for="(ship_if, index) in attacker_ships" :ship-name="ship_if.shipName"
                     v-model:ship="ship_if.ship" v-model:ship-count="ship_if.shipCount" class="m-2"/>
      </div>
      <div class="flex flex-col items-center">
        <div class="text-center text-lg">Defender</div>
        <ShipDisplay v-for="(ship_if, index) in defender_ships" :ship-name="ship_if.shipName"
                     v-model:ship="ship_if.ship" v-model:ship-count="ship_if.shipCount" class="m-2"/>
      </div>
      <div class="flex flex-col justify-center basis-48">
          <div class="text-center">Results</div>
          <div class="text-center">Defender win: {{ defender_win_percent * 100 }}%</div>
          <div class="text-center">Attacker win: {{ (1 - defender_win_percent) * 100 }}%</div>
        <div class="flex justify-center w-full">
          <button class="shadow-lg  w-24 text-white bg-gray-800 hover:bg-gray-700" @click="simulate_battle_js" type="button">Simulate</button>
        </div>
      </div>
    </div>

  </main>

  </body>

</template>
