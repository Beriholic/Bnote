<template>
    <div class="container">
        <v-timeline align="start">
            <v-timeline-item v-for="note in notes" :key="note.id">
                <template v-slot:opposite>
                    {{ note.created_at }}
                </template>
                <div class="text-h6">{{ note.title }}</div>
            </v-timeline-item>
        </v-timeline>

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

<style scoped lang="less"></style>