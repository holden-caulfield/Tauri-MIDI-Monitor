<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const midiPorts = ref([]);

const refreshMidiPorts = async () => {
  try {
    const midiPortsData = await invoke("fetch_midi_ports");
    midiPorts.value = midiPortsData;
  } catch (error) {
    console.error("Error fetching MIDI ports:", error);
    midiPorts.value = [];
  }
};

onMounted(refreshMidiPorts);

const selectedMIDIPort = ref("");
</script>

<template>
  <h1>MIDI Ports</h1>
  <div>
    <button @click="refreshMidiPorts">Refresh</button>
  </div>
  <div class="card">
    <div>Selected MIDI Port: {{ selectedMIDIPort }}</div>

    <select v-model="selectedMIDIPort">
      <option v-for="port in midiPorts" :key="port">{{ port }}</option>
    </select>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
