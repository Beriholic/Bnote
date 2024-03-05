<template>
    <div class="info">
        <button class="slide" @click="startShowSequence">
            <icon-info-circle class="rotating" :style="{ fontSize: '30px' }" />
        </button>
        <n-modal v-model:show="showModal">
            <n-card title="" :bordered="true" size="huge" role="dialog" aria-modal="true"
                style="height: 80vh;width: 100vh;">
                <div class="total">
                    <div v-if="step >= 1" style="font-size: 34px;">🎉🎉您已经写下了</div>
                    <div v-if="step >= 2" style="font-size: 42px;">
                        <n-number-animation ref="numberAnimationInstRef" :from="0" :to="writingNumber" :duration="3000"
                            @finish="finishShow" />
                        个字
                    </div>
                    <div v-if="step >= 4" style="font-size: 34px;">相当于写满了 {{ writingNumber / 16000 }} 本作文本</div>
                </div>
            </n-card>
        </n-modal>
    </div>

</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';
const step = ref(1);

const showModal = ref(false);
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

let writingNumber = ref(0);

onMounted(() => {
    get_total_number();

    //mock
    if (writingNumber.value === 0) {
        writingNumber.value = 1000000
    }
});


async function get_total_number() {
    await invoke('get_total_number').then((res) => {
        writingNumber.value = res as number
    })
}


</script>

<style lang="less">
.slide {
    background-color: #FFFFFF;
    border-style: none;
    outline: none;
    padding-left: 10px;
    padding-top: 10px;
    padding-bottom: 8px;
}
</style>