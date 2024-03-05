import { createRouter, createWebHashHistory } from "vue-router";
import NoteList from "../views/NoteList.vue";
import Search from "../views/Search.vue";
import Setting from "../views/Setting.vue";
const routes = [
  {
    path: "/",
    name: "NoteList",
    component: NoteList,
  },
  {
    path: "/search",
    name: "Search",
    component: Search,
  },
  {
    path: "/setting",
    name: "Setting",
    component: Setting,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});

export default router;
