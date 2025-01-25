<script setup lang="ts">

import Counter from "@/components/Counter.vue";
import {Ship} from "../../../simulator/pkg";
import {ref} from "vue";

const props = defineProps({
  shipName: String,
  // shipCount: Number,
});
const ship = defineModel<Ship>('ship');
const shipCount = defineModel<number>('shipCount');
// const shipCount = ref<number>(0);
// console.log(ship.hull);
console.log(shipCount.valueOf());

</script>

<template>
  <div class="grid grid-cols-1 gap-1 border-4 w-48 rounded-md border-gray-800 bg-gray-400 p-2">
    <div class="text-center">{{props.shipName}}</div>
    <div class="flex flex-row justify-center content-center">
      <Counter v-model:count="shipCount"/>
    </div>
    <div class="flex flex-row justify-between">
      <div class="text-left">
        Hull:
      </div>
      <div>
        <Counter v-model:count="ship.hull"/>
      </div>
    </div>
    <div class="flex flex-row justify-between">
      <div class="text-left">
        Initiative:
      </div>
      <div>
        <Counter v-model:count="ship.initiative"/>
      </div>
    </div>
    <div class="flex flex-row justify-between">
      <div class="text-left">
        Shield:
      </div>
      <div>
        <Counter v-model:count="ship.shield"/>
      </div>
    </div>
    <div class="flex flex-row justify-between">
      <div class="text-left">
        Computer:
      </div>
      <div>
        <Counter v-model:count="ship.computer"/>
      </div>
    </div>
    <div class="flex flex-row justify-between">
      <div class="text-left">
        Weapon 1:
      </div>
      <div>
        <Counter v-model:count="ship.weapon_1_dmg"/>
      </div>
    </div>
    <div class="flex flex-row justify-between">
      <div class="text-left">
        Weapon 2:
      </div>
      <div>
        <Counter v-model:count="ship.weapon_2_dmg"/>
      </div>
    </div>
  </div>

</template>

<style scoped>

</style>