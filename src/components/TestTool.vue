<template>
  <div>
    <input type="text" v-model="name" placeholder="Enter your name" />
    <button @click="greet">Greet</button>
    <p v-if="greeting">Greeting: {{ greeting }}</p>
    <br><br>

    <input type="number" v-model="num1" placeholder="Enter number 1" />
    <input type="number" v-model="num2" placeholder="Enter number 2" />
    <button @click="addNumbers">Add</button>
    <p v-if="sum">Sum: {{ sum }}</p>
    <br><br>

    <p>随机字符串为：{{ randomString }}</p>
    <br><br>

    <p v-if="error" style="color: red;">Error: {{ error }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { callPython } from '../services/rustService';

// 使用 ref 创建响应式数据
const name = ref('');
const greeting = ref('');
const num1 = ref('');
const num2 = ref('');
const sum = ref('');
const randomString = ref('');
const error = ref('');

// 定义 greet 函数
const greet = async () => {
  try {
    // 访问 ref 的值需要 .value
    greeting.value = await callPython(
      './python_scripts/example.py',
      'greet',
      name.value // 传递 ref 的值
    );
  } catch (e) {
    error.value = e.toString();
  }
};

// 定义 addNumbers 函数
const addNumbers = async () => {
  try {
    // 确保传递的是数字类型，使用 parseInt
    sum.value = await callPython(
      './python_scripts/example.py',
      'add',
      String(num1.value), // 需要传递字符串
      String(num2.value)
    );
  } catch (e) {
    error.value = e.toString();
  }
};


setInterval(async () => {
  try {
    const x = await callPython(
      './python_scripts/example.py',
      'generate_random_string',
    );
    randomString.value = x;
  } catch (e) {
    error.value = e.toString();
  }

}, 1000);
</script>

<style scoped>
/* 如果有组件特有的样式，可以放在这里 */
</style>
