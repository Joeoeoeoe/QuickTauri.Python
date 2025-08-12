import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export const useToolStore = defineStore('tool', () => {
  // 组件注册表
  const toolComponents = ref({});

  // state
  // route用于路由参数
  // id <=> 组件 <=> 路由参数
  // 因此即使（1）id重复了也不会报错，因为通过categories渲染，并不要求不能重复（2）组件未注册，也可以通过路由访问
  // icon可以用简单的相对路径，例如./x/y.svg x/y.svg /x.svg 但是不能用.. 
  const categories = ref([
    { 
      name: '测试工具', 
      tools: [
        { id: 'TestTool', icon: 'test.svg', name: '测试子工具', description: '这是一个用于测试的子工具' },
      ] 
    },
  ]);


  // getters
  const allTools = computed(() => categories.value.flatMap(category => category.tools));

  // actions
  function registerToolComponent(toolId, importFn) {
    toolComponents.value[toolId] = importFn;
  }

  function getToolComponent(toolId) {
    const importFn = toolComponents.value[toolId];
    return importFn ? importFn() : null;
  }

  function addCategory(category) {
    categories.value.push(category);
  }

  function addToolToCategory(categoryName, tool) {
    const category = categories.value.find(cat => cat.name === categoryName);
    if (category) {
      category.tools.push(tool);
    } else {
      console.warn(`Category "${categoryName}" not found.`);
    }
  }

  // 初始化时注册工具组件
  // 基础组件
  toolComponents.value = {
    TestTool: () => import('../views/TestTool.vue')
  };
  // 其余组件可以在任意.vue中通过registerToolComponent进行注册
  // 例如：
  // import { useToolStore } from '../stores/toolStore';
  // const toolStore = useToolStore();
  // toolStore.registerToolComponent('ImageMergeTool', () => import('../components/ImageMergeTool.vue'));

  return {
    categories,
    allTools,
    registerToolComponent,
    getToolComponent,
    addCategory,
    addToolToCategory
  };
});
