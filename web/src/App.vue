<script setup lang="ts">
import {BattleResult, Fleet, RngState, Ship, ShipType, simulate_battle} from "simulator"
import ShipDisplay from "@/components/ShipDisplay.vue";
import {ref} from "vue";


interface ShipDescription {
  shipCount: number;
  shipName: string;
  ship: Ship;
}

const defender_win_percent = ref(0);
const simulation_steps = ref(100_000);
const calculating = ref(false);

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
  let rng_state = new RngState(BigInt(42));
  // Always use the same seed for the RNG so that the results are reproducible
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
  // Spawn a simulationWorker.ts webworker to run the simulation

  let worker = new Worker(new URL("./simulationWorker.ts", import.meta.url), {
    type: "module",
  });
  worker.onmessage = (event) => {
    console.log("Received message from worker", event.data);
    defender_win_percent.value = event.data.defender_win_percent;
    calculating.value = false;
  }
  console.log("Sending message to worker");
  calculating.value = true;
  worker.postMessage({
    attacker_fleet: attacker_fleet.to_json(),
    defender_fleet: defender_fleet.to_json(),
    simulation_steps: simulation_steps.value,
    rng_seed: 42
  });
}

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
        <div class="text-center">Defender win: {{ (defender_win_percent * 100).toPrecision(2) }}%</div>
        <div class="text-center">Attacker win: {{ ((1 - defender_win_percent) * 100).toPrecision(2) }}%</div>
        <div class="flex justify-center w-full">
          <button class="shadow-lg  w-24 text-white bg-gray-800 hover:bg-gray-700" @click="simulate_battle_js"
                  type="button">Simulate
          </button>
          <div class="w-5 h-5 border-4 border-blue-500 border-t-transparent rounded-full animate-spin" v-if="calculating"></div>

        </div>
      </div>
    </div>

  </main>

  </body>

</template>
