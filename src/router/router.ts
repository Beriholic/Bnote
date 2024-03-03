import { createRouter, createWebHashHistory } from "vue-router";
import NoteList from "../views/Notelist.vue";
import Search from "../views/Search.vue";
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
];

const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});

export default router;
