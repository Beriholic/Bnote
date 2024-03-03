import { createApp } from "vue";
import App from "./App.vue";
import router from './router/router'
import naive from 'naive-ui'
import ArcoVueIcon from '@arco-design/web-vue/es/icon';
import ArcoVue from '@arco-design/web-vue';
import '@arco-design/web-vue/dist/arco.css';
createApp(App)
    .use(router)
    .use(naive)
    .use(ArcoVue)
    .use(ArcoVueIcon)
    .mount("#app");
