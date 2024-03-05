<template>
    <div class="bar">
        <button class=" file">
            <icon-file class="rotating" :style="{ fontSize: '30px' }" @click="goTo('/', 0)" />
        </button>
        <button class="search">
            <icon-search class="rotating" :style="{ fontSize: '30px' }" @click="goTo('/search', 1)" />
        </button>
        <div class="info">
            <Info />
        </div>
    </div>

</template>

<style lang="less" scoped>
.total {
    display: flex;
    flex-direction: column;
    margin-top: 20px;
}

.bar {
    display: flex;
    flex-direction: column;
    align-items: center;

    .rotating {
        transition: transform 0.6s ease;
    }

    .rotating:hover {
        transform: rotate(180deg);
    }

    button {
        background-color: #FFFFFF;
        border-style: none;
        outline: none;
        padding-left: 10px;
        padding-top: 10px;
        padding-bottom: 8px;
    }

}
</style>


<script setup lang="ts">
import Info from '../views/Info.vue';
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
const router = useRouter();
let currentRoute = 0;
let isShow = true;
const emit = defineEmits(['update:isShow']);

onMounted(() => {
    isShow = true;
});

const updateHidden = (newValue: boolean) => {
    isShow = newValue;
    emit('update:isShow', newValue);
};
function goTo(to: string, route: number) {
    if (currentRoute === route) {
        updateHidden(!isShow);
    } else {
        currentRoute = route;
        router.push(to);
        if (isShow === false) {
            updateHidden(!isShow);
        }
    }
}

</script>
