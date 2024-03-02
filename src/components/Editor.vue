<template>
    <div id="vditor" />
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Vditor from 'vditor';
import 'vditor/dist/index.css';

const vditor = ref<Vditor | null>(null);

onMounted(() => {
    vditor.value = new Vditor('vditor', {
        height: "100vh",
        width: "auto",
    });
});
</script>

<style lang="less" scoped>
#vditor {
    flex-grow: 1;
}
</style>