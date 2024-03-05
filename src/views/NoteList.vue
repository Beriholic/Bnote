<template>
    <div>
        <h1>笔记列表 💾</h1>
        <v-divider :thickness="5"></v-divider>
        <div class="container">
            <div v-for="note in notes" :key="note.id">
                <v-card hover class="block" width="700" height="400">
                    <v-card-title class="title">
                        {{ note.title }}
                    </v-card-title>
                    <v-card-subtitle class="create-time">
                        {{ note.createdAt }}
                    </v-card-subtitle>
                </v-card>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import Note from '../models/Note';

const notes = ref<Note[]>([]);

onMounted(() => {
    mock_data()
})
computed(() => {
    mock_data()
})


const mock_data = () => {
    for (let i = 1; i <= 9; i++) {
        notes.value.push(
            new Note(
                i,
                `测试-笔记${i}`,
                `这是测试文章${i}的内容`,
                '2023-12-32 12:32:20'
            )
        )
    }
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
    margin: 30px;
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
        color: #666;
    }

}
</style>