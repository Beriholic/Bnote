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
import { computed, onMounted, ref } from 'vue';
import Categories from '../../models/Categories';
const categories = ref<Categories[]>([]);
onMounted(() => {
    mock_data()
})
computed(() => {
    mock_data()
})
const mock_data = () => {
    for (let i = 1; i <= 9; i++) {
        categories.value.push(
            new Categories(
                i,
                `测试-分类${i}`,
            )
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

    .name {
        font-size: 24px;
        font-weight: bold;
        margin-bottom: 20px;
    }


}
</style>