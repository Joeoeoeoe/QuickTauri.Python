<template>
  <div class="bg-white shadow-md rounded-lg p-4 mb-4">
    <div class="flex items-center justify-between cursor-pointer" @click="toggleCollapse">
      <h2 class="text-xl font-semibold text-gray-800">{{ category.name }}</h2>
      <svg :class="{ 'rotate-90': isCollapsed }" class="w-5 h-5 text-gray-600 transform transition-transform duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
      </svg>
    </div>
    <div v-if="!isCollapsed" class="mt-4 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
      <slot></slot>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
  category: {
    type: Object,
    required: true,
  },
});

const isCollapsed = ref(false);

const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value;
};
</script>

<style scoped>
/* 可以添加一些自定义样式，如果 Tailwind CSS 不够用 */
</style>
