<template>
    <v-navigation-drawer :width="85">
        <v-list-item>
            <button class="file">
                <icon-file :style="{ fontSize: '30px' }" @click="goTo('/')" />
            </button>
        </v-list-item>
        <v-list-item>
            <button class="search" @click="goTo('/search')">
                <icon-search :style="{ fontSize: '30px' }" />
            </button>
        </v-list-item>
        <v-list-item>
            <button v-if="is_dark_mode" @click="switch_theme">
                <icon-moon-fill :style="{ fontSize: '30px' }" />
            </button>
            <button v-else @click="switch_theme">
                <icon-moon :style="{ fontSize: '30px' }" />
            </button>
        </v-list-item>
        <v-list-item>
            <button class="setting" @click="goTo('/setting')">
                <icon-settings :style="{ fontSize: '30px' }" />
            </button>
        </v-list-item>
    </v-navigation-drawer>


</template>

<script setup lang="ts">
import { useTheme } from 'vuetify'
import { useRouter } from 'vue-router';
import { useThemeStore } from '../store';
import { computed, ref } from 'vue';
const utheme = useThemeStore()
const theme = useTheme()

const is_dark_mode = computed(() => {
    let mode: boolean = theme.global.current.value.dark
    return ref(mode)
})
const switch_theme = () => {
    utheme.toggleTheme()
}

//----------------------------------
const router = useRouter()

function goTo(to: string) {
    router.push(to)
}






</script>

<style lang="less">
button {
    margin: 10px;
    padding: 10px;
    border: none;
    border-radius: 8px;
    background-color: #f0f0f0;
    cursor: pointer;
    transition: background-color 0.3s;

    &:hover {
        background-color: #d1d1d1;
    }
}
</style>
