<template>
    <div class="container">
        <div class="title">测试文章</div>
        <div class="create-time">2023-12-32 12:32:20</div>
    </div>

</template>


<style lang="less" scoped>
.container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 20%;
    width: 20%;
    background-color: #f5f5f5;
    padding: 20px;
    box-sizing: border-box;
    margin: 10px;

    .title {
        font-size: 24px;
        font-weight: bold;
        margin-bottom: 20px;
    }

    .create-time {
        font-size: 14px;
        color: #666;
    }
}
</style>