<template>
    <div class="container">
        <div v-for="note in notes" :key="note.id">
            <v-card hover class="block" width="400" height="250">
                <v-card-title class="title">
                    {{ note.title }}
                </v-card-title>
                <v-card-subtitle class="create-time">
                    {{ note.createdAt }}
                </v-card-subtitle>
            </v-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import Note, { new_note } from '../../models/Note';
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
            new_note(i, `测试-笔记${i}`, `这是测试文章${i}的内容测试测试测试测测试测.....`, 114514, '2023-12-32')
        )
    }
}


</script>

<style scoped lang="less">
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