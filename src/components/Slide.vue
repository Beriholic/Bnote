<template>
    <div class="bar">
        <button class=" file">
            <icon-file class="rotating" :style="{ fontSize: '30px' }" @click="goTo('/', 0)" />
        </button>
        <button class="search">
            <icon-search class="rotating" :style="{ fontSize: '30px' }" @click="goTo('/search', 1)" /> </button>
        <div class="info">
            <button @click="startShowSequence">
                <icon-info-circle class="rotating" :style="{ fontSize: '30px' }" />
            </button>
            <n-modal v-model:show="showModal">
                <n-card title="" :bordered="true" size="huge" role="dialog" aria-modal="true"
                    style="height: 80vh;width: 100vh;">
                    <div class="total">
                        <div v-if="step >= 1" style="font-size: 34px;">🎉🎉您已经写下了</div>
                        <div v-if="step >= 2" style="font-size: 42px;">
                            <n-number-animation ref="numberAnimationInstRef" :from="0" :to="mock_font_number"
                                :duration="3000" @finish="finishShow" />
                            个字
                        </div>
                        <div v-if="step >= 4" style="font-size: 34px;">相当于写满了 {{ mock_font_number / 16000 }} 本作文本</div>
                    </div>
                </n-card>
            </n-modal>
        </div>
    </div>

</template>

<style scoped>
.total {
    display: flex;
    flex-direction: column;
    margin-top: 20px;
}
</style>

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
</style>


<script setup lang="ts">
import { onMounted, ref } from 'vue';
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


const showModal = ref(false);
const mock_font_number = 100000000;
const step = ref(1);

const startShowSequence = async () => {
    showModal.value = true;
    step.value = 1;

    setTimeout(() => {
        step.value++;
    }, 500);

    setTimeout(() => {
        step.value++;
    }, 1000);
};
const finishShow = () => {
    step.value = 4;
};

</script>
