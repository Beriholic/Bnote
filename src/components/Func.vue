<template>
    <n-flex>
        <div class="bar">
            <button class="slide" @click="slidebarShowSwitch">
                <icon-list class="rotating" :style="{ fontSize: '30px' }" />
            </button>
            <button class=" file">
                <icon-file class="rotating" :style="{ fontSize: '30px' }" />
            </button>
            <button class="search">
                <icon-search class="rotating" :style="{ fontSize: '30px' }" /> </button>
            <button class="info">
                <icon-info-circle class="rotating" :style="{ fontSize: '30px' }" />
            </button>
            <button class="setting">
                <icon-settings class="rotating" :style="{ fontSize: '30px' }" />
            </button>
        </div>
        <div class="show" v-show="slidebarIsShow">
            <n-scrollbar style="max-height: 100vh" class="scrollbar">
                <!-- <div v-for="node in nodelist" :key="node.id">
                    <label>
                        <div class="info">
                            <div>{{ node.title }}</div>
                            <div>{{ node.createTime }}</div>
                        </div>
                    </label>
                </div> -->
                <n-list>
                    <n-list-item v-for="node in nodelist" :key="node.id">
                        <button class="info">
                            <n-ellipsis style="max-width: 240px">
                                {{ node.title }}
                            </n-ellipsis>
                            {{ node.createTime }}
                        </button>
                    </n-list-item>

                </n-list>
            </n-scrollbar>
        </div>
    </n-flex>

</template>


<style lang="less" scoped>
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

.show {
    display: flex;
    flex-direction: column;
    width: 250px;

    .info {
        margin: atuo;
    }
}
</style>


<script setup lang="ts">
import { ref } from 'vue';
let slidebarIsShow = ref(true)

const slidebarShowSwitch = () => {
    slidebarIsShow.value = !slidebarIsShow.value
}

type node = {
    id: number,
    title: string,
    createTime: string
}

const nodelist: node[] = [
    {
        id: 1,
        title: 'test',
        createTime: '2021-10-10'
    },
    {
        id: 2,
        title: 'test2',
        createTime: '2021-10-10'
    },
    {
        id: 3,
        title: '重生之我是原神高手',
        createTime: '2021-10-10'
    },
    {
        id: 4,
        title: '重重生之我是原神高重生之我是原神高重生之我是原神高重生之我是原神高重生之我是原神高重生之我是原神高重生之我是原神高重生之我是原神高重生之我是原神高生之我是原神高手',
        createTime: '2021-10-10'
    },

]


</script>
