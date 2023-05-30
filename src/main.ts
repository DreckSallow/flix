import { createApp } from "vue";
import { RouteRecordRaw, createRouter, createWebHistory } from "vue-router";
import "./reset.css";
import "./styles.css";
import "uno.css";
import { HomePage, StudyAreaPage } from "./pages";
import AppVue from "./App.vue";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    component: HomePage,
    children: [
      {
        path: "new-space",
        name: "NewSpace",
        component: {
          template: "<div>CREATE NEW SPACE</div>",
        },
      },
      {
        path: ":area",
        name: "StudyArea",
        component: StudyAreaPage,
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const app = createApp(AppVue);
app.use(router);

app.mount("#app");
