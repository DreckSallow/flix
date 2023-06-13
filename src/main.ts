import { createApp } from "vue";
import { RouteRecordRaw, createRouter, createWebHistory } from "vue-router";
import "./reset.css";
import "./styles.css";
import "uno.css";
import { HomePage } from "./pages";
import AppVue from "./App.vue";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    component: HomePage,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const app = createApp(AppVue);
app.use(router);

app.mount("#app");
