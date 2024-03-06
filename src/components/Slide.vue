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
            <button v-if="is_dark_theme" @click="switch_theme">
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
import { ref } from 'vue';
import { useTheme } from 'vuetify'
import { useRouter } from 'vue-router';

const theme = useTheme()

let is_dark_theme = ref(false)
const switch_theme = () => {
    theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
    is_dark_theme.value = !is_dark_theme.value
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
