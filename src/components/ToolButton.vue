<!-- <template>
  <button @click="navigateToTool" class="flex flex-col items-center justify-center p-4 bg-gray-100 text-gray-800 rounded-lg shadow-md hover:bg-gray-200 transition-colors duration-200 text-center">
    <div class="flex items-center">
      <img :src="require(`@/assets/icons/${tool.icon}`)" v-if="tool.icon" class="w-6 h-6 mr-2" :alt="tool.name + ' icon'">
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

const navigateToTool = () => {
  if (props.tool.id) {
    toolHistory.push(props.tool.id);
    router.push({ name: 'GenericToolPage', params: { toolId: props.tool.id } });
  }
};
</script>

<style scoped>
/* 可以添加一些自定义样式 */
</style> -->

<template>
  <button @click="navigateToTool" class="flex flex-col items-center justify-center p-4 bg-gray-100 text-gray-800 rounded-lg shadow-md hover:bg-gray-200 transition-colors duration-200 text-center">
    <div class="flex items-center">
      <!-- 修改了这里 -->
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

// 新增的函数
const getToolIconUrl = (iconName) => {
  if (!iconName) return '';
  // 注意这里的路径是相对于当前文件的。
  // 如果你的图标都放在 src/assets/icons/ 下，且这个组件在 src/components/ 或者类似的地方，
  // 那么相对路径可能是 ../assets/icons/ 或 ../../assets/icons/ 等。
  // 最好是使用相对路径来定位到 assets 目录下的文件。
  return new URL(`../assets/icons/${iconName}`, import.meta.url).href;

  // 如果你的构建工具支持别名，并且你希望使用别名（例如 @/），
  // 那么你可能需要结合具体的构建工具配置。
  // 对于 Vite，通常直接使用相对路径到 public 或 assets 目录更常见。
  // 如果你的 assets 文件夹在 src 目录下，那么使用 src/assets/icons/` 或 `../assets/icons/` 会是更健壮的选择。
  // 假设你的 ToolButton.vue 在 src/components 目录下，而图标在 src/assets/icons/ 目录下：
  // return new URL(`../assets/icons/${iconName}`, import.meta.url).href;
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
