<template>
  <component :is="dynamicIcon" v-bind="$attrs" v-if="dynamicIcon" />
  <span v-else class="icon-placeholder">❓</span>
</template>

<script lang="ts" setup>
import { onMounted, shallowRef, watch } from "vue";

const props = defineProps<{
  name: string;
}>();

const dynamicIcon = shallowRef<any>(null);

const icons = import.meta.glob("../assets/icons/*.svg", { eager: true });

const loadSvgIcon = (iconName: string) => {
  const iconPath = `../assets/icons/${iconName}.svg`;
  if (icons[iconPath]) {
    dynamicIcon.value = icons[iconPath];
  } else {
    console.error("SVG icon not found:", iconName);
    dynamicIcon.value = null;
  }
};

watch(() => props.name, loadSvgIcon, { immediate: true });

onMounted(() => {
  loadSvgIcon(props.name);
});
</script>