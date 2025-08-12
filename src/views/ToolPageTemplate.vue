<template>
  <div class="p-8 bg-white shadow-md rounded-lg">
    <h1 class="text-3xl font-bold mb-6 text-gray-800">{{ title }}</h1>
    <p v-if="description" class="text-gray-600 mb-4">{{ description }}</p>
    <slot></slot>
  </div>
</template>

<script setup>

const props = defineProps({
  title: {
    type: String,
    required: true,
  },
  description: {
    type: String,
    default: '',
  },
});
</script>

<style scoped>
/* 可以添加一些自定义样式 */
</style>
