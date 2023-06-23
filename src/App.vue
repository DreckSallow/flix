<script lang="ts" setup>
import { computed } from "vue";
import { NotifyState } from "./state";
import Notification from "@components/notification.vue";

function closeNotify(k: number) {
  NotifyState.remove(k);
}
const hasContent = computed(() => NotifyState.notifications.length > 0);
</script>

<template>
  <router-view></router-view>
  <article
    class="fixed bottom-2 right-2 w-300px z-999 overflow-auto"
    :class="{
      'pointer-events-none': !hasContent,
    }"
  >
    <TransitionGroup
      class="p-3 flex flex-col gap-4"
      name="notifications"
      tag="ul"
    >
      <li
        v-for="(notification, i) in NotifyState.notifications"
        class="overflow-auto bg-accent rounded-md"
        :key="notification.id"
      >
        <Notification
          @close="closeNotify"
          :type="notification.type"
          :keyId="notification.id"
          :content="notification.content"
          :debug="notification.debug"
          :title="notification.title"
        />
      </li>
    </TransitionGroup>
  </article>
</template>

<style scoped>
article {
  max-height: 240px;
  overflow-x: hidden;
}

article li {
  max-height: 100px;
  min-height: 40px;
}

.notifications-enter-active,
.notifications-leave-active {
  transition: all 0.5s ease;
}
.notifications-enter-from,
.notifications-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
