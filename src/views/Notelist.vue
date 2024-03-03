<template>
    <div class="show">
        <n-scrollbar style="max-height: 100vh" class="scrollbar">
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
</template>


<style lang="less" scoped>
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
type node = {
    id: number;
    title: string;
    createTime: string;
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