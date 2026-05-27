import { createRouter, createWebHashHistory } from "vue-router";
import SettingsPage from "@/views/SettingsPage.vue";
import LogsPage from "@/views/LogsPage.vue";

const routes = [
  { path: "/settings", component: SettingsPage },
  { path: "/logs", component: LogsPage },
  { path: "/:pathMatch(.*)*", redirect: "/settings" },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
