<template>
  <button @click="navigateToTool" class="flex flex-col items-center justify-center p-4 bg-gray-100 text-gray-800 rounded-lg shadow-md hover:bg-gray-200 transition-colors duration-200 text-center">
    <div class="flex items-center">
      <img :src="getToolIconUrl(tool.icon)" v-if="tool.icon" class="w-6 h-6 mr-2" :alt="tool.name + ' icon'">
      <span class="text-lg font-medium">{{ tool.name }}</span>
    </div>
    <span v-if="tool.description" class="text-sm mt-1 opacity-80">{{ tool.description }}</span>
  </button>
</template>

<script setup>
import { useRouter } from 'vue-router';
import { toolHistory } from '@/router/index';

const props = defineProps({
  tool: {
    type: Object,
    required: true,
  },
});

const router = useRouter();


const getToolIconUrl = (iconPath) => {
  if (!iconPath) return '';
  if (iconPath.startsWith('./')) {
    iconPath = iconPath.substring(2);
  } else if (iconPath.startsWith('/') || !iconPath.endsWith('.svg')) {
    iconPath = 'imgsplit.svg'; // 如果是绝对路径，返回默认图标
  }
  return new URL(`../assets/icons/${iconPath}`, import.meta.url).href;
};

const navigateToTool = () => {
  if (props.tool.id) {
    // 确保 toolHistory 在这里是可用的，并且是正确的
    toolHistory.push(props.tool.id);
    router.push({ name: 'GenericToolPage', params: { toolId: props.tool.id } });
  }
};
</script>

<style scoped>
/* 可以添加一些自定义样式 */
</style>
