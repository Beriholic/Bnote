<template>
    <div class="container">
        <div v-for="cate in categories" :key="cate.id">
            <v-card hover class="block" width="400" height="250">
                <v-card-title class="name">
                    {{ cate.name }}
                </v-card-title>
            </v-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Categories from '../../models/Categories';
import { invoke } from '@tauri-apps/api';
const categories = ref<Categories[]>([]);

onMounted(() => {
    getCategoriesList()
})

async function getCategoriesList() {
    await invoke('get_categories_list').then((res) => {
        categories.value = res as Categories[]
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

    .name {
        font-size: 24px;
        font-weight: bold;
        margin-bottom: 20px;
    }


}
</style>