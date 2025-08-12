<template>
  <button
    :class="[
      'flex items-center justify-center rounded-lg shadow-md transition-colors duration-200 text-center',
      colorClasses,
      sizeClasses,
      { 'cursor-not-allowed opacity-50': disabled },
    ]"
    :disabled="disabled"
    @click="$emit('click')"
  >
    <slot />
  </button>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  color: {
    type: String,
    default: 'blue',
    validator: (value) => ['blue', 'green', 'red', 'gray'].includes(value),
  },
  size: {
    type: String,
    default: 'md',
    validator: (value) => ['sm', 'md', 'lg'].includes(value),
  },
  disabled: {
    type: Boolean,
    default: false,
  },
});

const colorClasses = computed(() => {
  switch (props.color) {
    case 'blue':
      return 'bg-blue-500 text-white hover:bg-blue-600';
    case 'green':
      return 'bg-green-500 text-white hover:bg-green-600';
    case 'red':
      return 'bg-red-500 text-white hover:bg-red-600';
    case 'gray':
      return 'bg-gray-200 text-gray-700 hover:bg-gray-300';
    default:
      return 'bg-blue-500 text-white hover:bg-blue-600';
  }
});

const sizeClasses = computed(() => {
  switch (props.size) {
    case 'sm':
      return 'p-2 text-sm';
    case 'md':
      return 'p-4 text-base';
    case 'lg':
      return 'p-6 text-lg';
    default:
      return 'p-4 text-base';
  }
});
</script>
