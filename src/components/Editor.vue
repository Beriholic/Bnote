<template>
    <div class="container">
        <div id="vditor"></div>
        <button @click="save_note">保存</button>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue';
import Vditor from 'vditor';
import 'vditor/dist/index.css';
import { useThemeStore } from '../store';
import Note from '../models/Note';
import { ExtractTitle } from '../utils/note';

const vditorRef = ref<Vditor | null>(null);
const utheme = useThemeStore()
import { useRouter } from 'vue-router';
const router = useRouter()

let note = ref(new Note())

const theme_mode = computed(() => {
    return utheme.theme_mode as 'dark' | 'classic'
})

watch(utheme, () => {
    vditorRef.value?.setTheme(theme_mode.value, theme_mode.value)
})

onMounted(() => {
    vditorRef.value = new Vditor('vditor', {
        height: "100vh",
        width: "auto",
        counter: {
            enable: true,
            type: "text",
            after(cnt) {
                note.value.wordCount = cnt
            }
        },
        cache: {
            enable: true
        },
        "preview": {
            "hljs": {
                "style": "abap",
                "lineNumber": true
            }
        },
        theme: utheme.theme_mode as 'dark' | 'classic',
        after() {
            vditorRef.value?.setTheme(theme_mode.value, theme_mode.value)
        },
        input(content) {
            note.value.content = content
        }
    });
    enable_copy()
    load_note_from_cache()
});

onUnmounted(() => {
    disable_copy()
})


const disable_copy = () => {
    document.onselectstart = () => false;
    document.oncopy = () => false;
    document.oncut = () => false;
    document.onpaste = () => false;
}
const enable_copy = () => {
    document.onselectstart = () => true;
    document.oncopy = () => true;
    document.oncut = () => true;
    document.onpaste = () => true;
}
const load_note_from_cache = () => {
    const item = localStorage.getItem('vditorvditor')
    note.value.content = item == null ? '' : item
}

function save_note() {
    note.value.title = ExtractTitle(note.value.content)
    const res = note.value.save();
    console.log(res)
    vditorRef.value?.clearCache()
    router.push('/')
}

</script>

<style lang="less" scoped>
.container {
    display: flex;
    flex-direction: column;
    margin: 0px;
    padding: 0px;

    button {
        display: flex;
        justify-content: center;
        align-items: center;
    }
}
</style>