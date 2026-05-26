<script setup lang="ts">
import { ref, watch, nextTick } from "vue";

const props = defineProps<{
  logs: { timestamp: string; message: string }[];
}>();

const container = ref<HTMLElement | null>(null);

watch(
  () => props.logs.length,
  async () => {
    await nextTick();
    if (container.value) {
      container.value.scrollTop = container.value.scrollHeight;
    }
  }
);
</script>

<template>
  <div ref="container" class="log-view">
    <div v-if="logs.length === 0" class="log-empty">暂无日志</div>
    <div v-for="(log, i) in logs" :key="i" class="log-line">
      <span class="log-time">{{ log.timestamp }}</span>
      {{ log.message }}
    </div>
  </div>
</template>

<style scoped>
.log-view {
  background: #1e1e1e;
  color: #d4d4d4;
  font-family: "SF Mono", "Monaco", "Menlo", monospace;
  font-size: 12px;
  line-height: 1.6;
  padding: 10px 12px;
  border-radius: 6px;
  height: 100%;
  overflow-y: auto;
  border: 1px solid #333;
}

.log-empty {
  color: #666;
  text-align: center;
  padding-top: 100px;
}

.log-line {
  white-space: pre-wrap;
  word-break: break-all;
}

.log-time {
  color: #6a9955;
  margin-right: 8px;
}
</style>
