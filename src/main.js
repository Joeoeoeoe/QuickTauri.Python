// 样式相关
import "@/assets/styles/index.css"; // 假设你有一个基础的CSS文件
import { createApp } from 'vue'
import App from './App.vue'
const app = createApp(App)


// 状态仓库
import { createPinia } from 'pinia'
import piniaPersist from 'pinia-plugin-persistedstate'
const pinia = createPinia()
pinia.use(piniaPersist)
app.use(pinia)


// 路由
import { router } from './router'
app.use(router)


// element-plus
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
app.use(ElementPlus)


app.mount('#app')
