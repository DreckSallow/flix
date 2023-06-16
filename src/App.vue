<script lang="ts" setup>
import { computed } from "vue";
import { NotifyState } from "./state";
import Notification from "@components/notification.vue";

function closeNotify(k: number) {
  NotifyState.remove(k);
}
const displayNotify = computed(() => NotifyState.notifications.length > 0);
</script>

<template>
  <router-view></router-view>
  <article
    class="fixed bottom-2 right-2 w-300px z-999 overflow-auto"
    v-if="displayNotify"
  >
    <ul class="p-3 flex flex-col gap-4">
      <li
        v-for="(notification, i) in NotifyState.notifications"
        class="overflow-auto bg-gray-200 rounded-md"
      >
        <Notification
          @close="closeNotify"
          :type="notification.type"
          :keyId="i"
          :text="notification.content"
          :title="notification.title"
        />
      </li>
    </ul>
  </article>
</template>

<style scoped>
/* article ul{
  padding: ;
} */

article {
  max-height: 240px;
}

article li {
  max-height: 100px;
}
</style>
