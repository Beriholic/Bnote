<template>
    <v-app>
        <Slide />
        <v-main>
            <RouterView />
        </v-main>
    </v-app>
</template>


<script setup lang="ts">
import { onMounted } from 'vue';
import Slide from './components/Slide.vue';
onMounted(() => {
    disable_copy()
})
const disable_copy = () => {
    document.onselectstart = () => false;
    document.oncopy = () => false;
    document.oncut = () => false;
    document.onpaste = () => false;
}
</script>
