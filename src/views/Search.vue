<template>
    <h1>搜索 🔍</h1>
    <v-divider :thickness="5"></v-divider>
    <v-text-field class="search-input" variant="solo-filled" v-model="input"></v-text-field>
</template>

<script setup lang="ts">
import { ref } from 'vue';

let input = ref('')
</script>

<style lang="less" scoped>
h1 {
    display: flex;
    justify-content: center;
    align-items: center;
}

.search-input {
    margin-top: 50px;
    margin-left: 500px;
    margin-right: 500px;
}
</style>