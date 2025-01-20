<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import TheWelcome from './components/TheWelcome.vue'
import {BattleResult, Fleet, RngState, Ship, ShipType, simulate_battle} from '../../simulator/pkg'


let rng_state = new RngState(BigInt(42));

let defender_wins: number = 0;
let n: number = 100_000;

for (let i = 0; i < n; i++) {
  let attacker = new Fleet([
    new Ship(2, 0, 1, 1, 2, 0, ShipType.Cruiser),
    new Ship(2, 0, 1, 1, 2, 0, ShipType.Cruiser),
  ]);

  let defender = new Fleet([
    new Ship(0, 0, 1, 1, 2, 0, ShipType.Cruiser),
    new Ship(0, 0, 1, 1, 2, 0, ShipType.Cruiser),
  ]);
  // console.log("simulating with attacker", attacker, "defender", defender);
  let result = simulate_battle(attacker, defender, rng_state);
  // console.log(i, result);
  if (result === BattleResult.DefenderWins ){
    defender_wins += 1;
  }
}
console.log(defender_wins);
</script>

<template>
  <header>
    <img alt="Vue logo" class="logo" src="./assets/logo.svg" width="125" height="125"/>

    <div class="wrapper">
      <HelloWorld msg="You did it!"/>
    </div>
  </header>

  <main>
    <TheWelcome/>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
