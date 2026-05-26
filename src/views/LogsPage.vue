<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import LogView from "@/components/LogView.vue";
import { getLogs } from "@/native/log";
import type { LogEntry } from "@/native/types";

const logs = ref<LogEntry[]>([]);
let pollTimer: ReturnType<typeof setInterval> | null = null;

async function refreshLogs() {
  logs.value = await getLogs();
}

onMounted(async () => {
  await refreshLogs();
  pollTimer = setInterval(refreshLogs, 3000);
});

onUnmounted(() => {
  if (pollTimer) clearInterval(pollTimer);
});
</script>

<template>
  <div class="logs-page">
    <h1 class="title">日志</h1>
    <div class="section">
      <div class="section-header">
        <span class="label">实时日志</span>
        <button class="btn sm" @click="refreshLogs">刷新</button>
      </div>
      <div class="log-wrapper">
        <LogView :logs="logs" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.logs-page {
  padding: 20px 24px;
  max-width: 520px;
  margin: 0 auto;
}

.title {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 16px 0;
  text-align: center;
}

.section {
  margin-bottom: 16px;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.label {
  font-size: 13px;
  font-weight: 500;
  color: #555;
}

.log-wrapper {
  height: 380px;
}

.btn {
  padding: 8px 16px;
  border: 1px solid #ddd;
  border-radius: 6px;
  background: #fff;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
}

.btn:hover {
  background: #f5f5f5;
}

.btn.sm {
  padding: 4px 10px;
  font-size: 12px;
}

@media (prefers-color-scheme: dark) {
  .label {
    color: #aaa;
  }

  .btn {
    background: #333;
    color: #ccc;
    border-color: #555;
  }

  .btn:hover {
    background: #444;
  }
}
</style>
