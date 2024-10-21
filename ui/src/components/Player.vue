<template>
  <div>
    <button @click="play">Play</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import init, { generate } from '../../app_bindgen'

const duration = ref(2) // Default duration (seconds)
let audioContext = null

// Initialize AudioContext and Wasm module when component is mounted
onMounted(async () => {
  audioContext = new (window.AudioContext || window.webkitAudioContext)()

  // Initialize the WASM module
  await init()
})

// Function to play the generated sine wave using Rust's generate function
const play = async () => {
  // Call generate function to get audio samples (Vec<f32>)
  const audioSamples = generate() // Vec<f32>

  // Create an audio buffer using the AudioContext
  const buffer = audioContext.createBuffer(1, audioSamples.length, audioContext.sampleRate)
  const channelData = buffer.getChannelData(0)

  // Copy the Rust-generated audio data into the buffer
  for (let i = 0; i < audioSamples.length; i++) {
    channelData[i] = audioSamples[i]
  }

  // Play the audio buffer
  const source = audioContext.createBufferSource()
  source.buffer = buffer
  source.connect(audioContext.destination)
  source.start()
}
</script>

<style scoped>
input[type='range'] {
  width: 100%;
}

button {
  margin-top: 10px;
  padding: 10px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}
button:hover {
  background-color: #369e70;
}
</style>
