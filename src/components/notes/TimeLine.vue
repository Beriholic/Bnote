<template>
    <div class="container">
        <v-timeline align="start">
            <v-timeline-item v-for="note in notes" :key="note.id">
                <template v-slot:opposite>
                    {{ note.createdAt }}
                </template>
                <div class="text-h6">{{ note.title }}</div>
            </v-timeline-item>
        </v-timeline>

    </div>

</template>


<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Note, { new_note } from '../../models/Note';
let notes = ref<Note[]>([]);
onMounted(() => {
    mock_data()
})

const mock_data = () => {
    for (let i = 1; i <= 10; i++) {
        notes.value.push(
            new_note(i, `测试-笔记${i}`, `这是测试文章${i}的内容测试测试测试测测试测.....`, 114514, '2023-12-32')
        )
    }
}


</script>

<style scoped lang="less"></style>