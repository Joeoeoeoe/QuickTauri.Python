import { createRouter, createWebHistory } from 'vue-router';
import Home from '@/views/Home.vue';
import GenericToolPage from '@/views/GenericToolPage.vue';

import { ref } from 'vue';

function useToolHistory() {
  const history = ref([]);
  const position = ref(-1);
  const maxLength = ref(5);

  const push = (toolId) => {
    // 新增的逻辑：如果toolId和前一个相同，则不再次加入
    if (history.value.length > 0 && history.value[position.value] === toolId) {
      console.log(`Tool ID "${toolId}" is already the last one, not adding again.`);
      return; // 直接返回，不进行后续操作
    }


    // 从当前position开始截断后续history
    if (position.value < history.value.length - 1) {
      history.value = history.value.slice(0, position.value + 1);
    }
    history.value.push(toolId);
 
    // 新增的维护逻辑
    if (history.value.length > maxLength.value) {
      history.value.shift(); // 移除最早的记录
      // 注意：如果shift了，position也需要相应调整
      // 如果移除了，但position指向的是被移除的元素，需要重新定位
      if (position.value > 0) { // 确保position不会变成负数
         position.value--;
      }
    }
    position.value++;
    
  };

  const back = () => {
    if (canBack()) {
      position.value--;
      return history.value[position.value];
    }
    return null;
  };

  const forward = () => {
    if (canForward()) {
      position.value++;
      return history.value[position.value];
    }
    return null;
  };

  const canBack = () => {
    return position.value > 0;
  };

  const canForward = () => {
    return position.value < history.value.length - 1;
  };

  return {
    history,
    position,
    push,
    back,
    forward,
    canBack,
    canForward,
  };
}

const toolHistory = useToolHistory();

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/tool/:toolId',
    name: 'GenericToolPage',
    component: GenericToolPage,
    props: true,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  if (to.name === 'Home') {

  }
  next();
});

export { toolHistory, router };
