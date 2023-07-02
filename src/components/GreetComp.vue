<script setup lang="ts">
import { ref } from "vue";
import * as App from "@/wits/app";

const greetMsg = ref("");
const lastGreetMsg = ref<null | string>("");

const name = ref("");

async function greet() {
    lastGreetMsg.value = await App.lastGreet();
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg.value = await App.greet(name.value);
}
</script>

<template>
  <div class="card">
    <form @submit.prevent>
      <input
        id="greet-input"
        v-model="name"
        placeholder="Enter a name..."
      >
      <button type="submit" @click="greet">
        Greet
      </button>
    </form>
  </div>

  <p>{{ greetMsg }}</p>
  <p>last: {{ lastGreetMsg }}</p>
</template>
