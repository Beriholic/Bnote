import { createRouter, createWebHashHistory } from "vue-router";
import Notes from "../views/Notes.vue";
import Search from "../views/Search.vue";
import Setting from "../views/Setting.vue";
const routes = [
  {
    path: "/",
    redirect: "/notes",
  },
  {
    path: "/notes",
    name: "Notes",
    redirect: "/notes/list",
    component: Notes,
    children: [
      {
        path: "list",
        name: "List",
        component: () => import("../components/notes/NoteList.vue"),
      },
      {
        path: "categories",
        name: "Caregories",
        component: () => import("../components/notes/Categories.vue"),
      },
      {
        path: "timeline",
        name: "Timeline",
        component: () => import("../components/notes/TimeLine.vue"),
      },
      {
        path: "new",
        name: "New",
        component: () => import("../components/notes/NewNote.vue"),
      },
    ],
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
