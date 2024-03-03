<template>
    <input type="text" v-model="search" class="search">
</template>

<script setup lang="ts">
import { ref } from 'vue';

const search = ref<string>('');

</script>

<style lang="less" scoped>
.search {
    max-height: 22px;
    margin-top: 12px;
}
</style>