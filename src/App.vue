<template>
    <v-app>
        <Slide />
        <v-main>
            <RouterView />
        </v-main>
    </v-app>
</template>


<script setup lang="ts">
import Slide from './components/Slide.vue';
</script>
