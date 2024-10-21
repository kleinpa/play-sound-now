<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header>
    <div class="wrapper">
      <nav>
        <ul>
          <!-- <li><RouterLink to="/">Home</RouterLink></li> -->
          <li><RouterLink to="/now">Play</RouterLink></li>
        </ul>
      </nav>
    </div>
  </header>

  <RouterView />
</template>

<style scoped></style>
