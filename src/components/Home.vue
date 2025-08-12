<template>
  <div class="p-8">
    <h1 class="text-3xl font-bold mb-6 text-gray-800">This is from Home.vue</h1>

    <div v-for="category in toolStore.categories" :key="category.name" class="mb-6">
      <ToolCategory :category="category">
        <ToolButton v-for="tool in category.tools" :key="tool.id" :tool="tool" />
      </ToolCategory>
    </div>
  </div>
</template>

<script setup>
import { useToolStore } from '../stores/toolStore';
import ToolCategory from '../components/ToolCategory.vue';
import ToolButton from '../components/ToolButton.vue';

const toolStore = useToolStore();

</script>


<style scoped>

</style>
