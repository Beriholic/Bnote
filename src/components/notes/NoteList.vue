<template>
    <div class="container">
        <div v-for="note in notes" :key="note.id">
            <v-card hover class="block" width="400" height="250">
                <v-card-title class="title">
                    {{ note.title }}
                </v-card-title>
                <v-card-subtitle class="create-time">
                    {{ note.created_at }}
                </v-card-subtitle>
            </v-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Note from '../../models/Note';
import { invoke } from '@tauri-apps/api';
let notes = ref<Note[]>([]);

onMounted(() => {
    getNoteList()
})

async function getNoteList() {
    await invoke('get_note_list').then((res) => {
        notes.value = res as Note[]
        console.log(notes.value)
    })
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
