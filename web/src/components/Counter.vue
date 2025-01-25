<script setup lang="ts">
import {ref, toRef} from 'vue';


// const count = ref(0);
// const count = ref(props.initialCount);
const count = defineModel<number>('count', {default: 0});

function handleInput(event: InputEvent) {
  // count.value = event.target.value;
  console.log(event);
  if (event.data === null) {
    return;
  }
  let parseResult = parseInt(event.data);
  if (isNaN(parseResult)) {
    // In order to trigger an update even if count.value already is 0
    count.value = 1;
    count.value = 0;
    return;
  }
  count.value = parseInt(event.data);
}


</script>

<template>
  <div>
    <button class="w-8" @click="count--">-</button>
    <input type="text" class="border-2 text-center h-6 w-6 aspect-square rounded-xl" v-bind:value="count"
           @input="handleInput">
    <button class="w-8" @click="count++">+</button>
  </div>
</template>

<style scoped>

</style>