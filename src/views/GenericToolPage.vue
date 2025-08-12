<template>
  <ToolPageTemplate v-if="tool" :title="tool.name" :description="tool.description">
    <p>这是 {{ tool.name }} 的具体功能界面。</p>
    <Suspense>
      <component :is="toolComponent" v-if="toolComponent" />
      <template #fallback>
        <div class="flex items-center justify-center p-4">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
        </div>
      </template>
    </Suspense>
    <div v-if="!toolComponent">
      <p>此工具的功能尚未实现。</p>
    </div>
  </ToolPageTemplate>
  <div v-else class="p-8 bg-white shadow-md rounded-lg text-red-500">
    <h1 class="text-3xl font-bold mb-6">工具未找到</h1>
    <p>抱歉，未找到您请求的工具。</p>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useToolStore } from '../stores/toolStore';
import ToolPageTemplate from './ToolPageTemplate.vue';

const route = useRoute();
const toolStore = useToolStore();
const toolComponent = ref(null);

const tool = computed(() => {
  const toolId = route.params.toolId;
  return toolStore.allTools.find(t => t.id === toolId);
});

onMounted(async () => {
  if (tool.value) {
    try {
      const component = await toolStore.getToolComponent(tool.value.id);
      toolComponent.value = component?.default || null;
    } catch (error) {
      console.error('Failed to load tool component:', error);
      toolComponent.value = null;
    }
  }
});
</script>

<style scoped>
/* 可以添加一些自定义样式 */
</style>
