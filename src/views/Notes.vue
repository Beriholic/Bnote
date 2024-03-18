<template>
    <div>
        <h1>笔记 💾 </h1>
        <div class="options">
            <v-btn @click="switch_option('list')">列表</v-btn>
            <!-- <v-btn @click="switch_option('categories')">分类</v-btn> -->
            <v-btn @click="switch_option('timeline')">时间线</v-btn>
            <v-btn @click="switch_option('new')">新建</v-btn>
        </div>
        <v-divider :thickness="5"></v-divider>
        <router-view></router-view>
    </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
const router = useRouter()
const switch_option = (where: string) => {
    router.push(`/notes/${where}`)
}
</script>

<style lang="less" scoped>
.container {
    display: flex;
    flex-wrap: wrap;
    align-self: center;
    justify-content: center;
    flex-grow: 1;
    margin-top: 30px;

}

h1 {
    display: flex;
    justify-content: center;
}

.block {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 20px;
    box-sizing: border-box;
    margin: 10px;
    border-radius: 10px;

    .title {
        font-size: 24px;
        font-weight: bold;
        margin-bottom: 20px;
    }

    .create-time {
        font-size: 14px;
    }

}

.options {
    display: flex;
    justify-content: center;

    button {
        margin: 10px;
        padding: 10px;
        border: none;
        border-radius: 8px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

}
</style>