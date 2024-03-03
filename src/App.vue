<template>
    <n-flex>
        <Slide v-model:is-show="isShow" />
        <RouterView v-show="isShow"></RouterView>
        <Editor />
    </n-flex>
</template>

<script setup lang="ts">
import Slide from './components/Slide.vue';
import Editor from './components/Editor.vue';
import { ref } from 'vue';
let isShow = ref<boolean>(true);
</script>