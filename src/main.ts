import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/router";

import ArcoVueIcon from "@arco-design/web-vue/es/icon";
import ArcoVue from "@arco-design/web-vue";
import "@arco-design/web-vue/dist/arco.css";
import "vuetify/styles";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

const vuetify = createVuetify({
  components,
  directives,
});

createApp(App)
  .use(router)
  .use(vuetify)
  .use(ArcoVue)
  .use(ArcoVueIcon)
  .use(pinia)
  .mount("#app");
