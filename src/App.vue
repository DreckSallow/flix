<template>
  <router-view></router-view>
  <div id="url" class="fixed bottom-2 right-2 bg-blue-300">
    {{ $router.currentRoute.value.fullPath }}
  </div>
</template>

<style scoped></style>
