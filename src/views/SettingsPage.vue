<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { loadCredentials, saveCredentials } from "@/native/store";
import { refreshStatus, setAutoLoginPaused, setIgnoreSsid } from "@/native/network_state";
import { tryLogin } from "@/native/login";
import type { NetworkStatus, LoginResult } from "@/native/types";

const username = ref("");
const password = ref("");
const networkType = ref("校园网");
const saved = ref(false);
const paused = ref(false);
const ignoreSsid = ref(false);
const loading = ref(false);
const networkStatus = ref<NetworkStatus | null>(null);
const loginResult = ref<LoginResult | null>(null);

let pollTimer: ReturnType<typeof setInterval> | null = null;

async function refreshStatusLoop() {
  const status = await refreshStatus();
  paused.value = status.paused;
  ignoreSsid.value = status.ignoreSsid;
  networkStatus.value = status.networkStatus;
}

async function initCredentials() {
  const creds = await loadCredentials();
  if (creds) {
    username.value = creds.username;
    password.value = creds.password;
    networkType.value = creds.network_type || "校园网";
    saved.value = true;
  }
}

async function handleSave() {
  await saveCredentials(username.value, password.value, networkType.value);
  saved.value = true;
}

async function handleLogin() {
  if (!username.value || !password.value) {
    loginResult.value = {
      success: false,
      message: "请先输入用户名和密码",
    };
    return;
  }
  loading.value = true;
  loginResult.value = null;
  try {
    loginResult.value = await tryLogin(
      username.value,
      password.value,
      networkType.value,
    );
  } catch (e: any) {
    loginResult.value = {
      success: false,
      message: typeof e === "string" ? e : "登录请求异常",
    };
  } finally {
    loading.value = false;
  }
}

async function togglePause() {
  paused.value = !paused.value;
  await setAutoLoginPaused(paused.value);
}

async function toggleIgnoreSsid() {
  ignoreSsid.value = !ignoreSsid.value;
  await setIgnoreSsid(ignoreSsid.value);
}

onMounted(async () => {
  await initCredentials();
  await refreshStatusLoop();
  pollTimer = setInterval(async () => {
    await refreshStatusLoop();
  }, 3000);
});

onUnmounted(() => {
  if (pollTimer) clearInterval(pollTimer);
});
</script>

<template>
  <div class="app">
    <h1 class="title">天科大校园网自动登录</h1>

    <!-- Network Status Bar -->
    <div class="status-bar" v-if="networkStatus">
      <span class="status-dot" :class="{ online: networkStatus.is_tust_network }"></span>
      <span v-if="networkStatus.wifi_ssid">{{ networkStatus.wifi_ssid }}</span>
      <span v-else class="dim">未检测到WiFi</span>
      <span class="sep">|</span>
      <span v-if="networkStatus.local_ipv4">{{ networkStatus.local_ipv4 }}</span>
      <span v-else class="dim">无IP</span>
      <span v-if="networkStatus.is_tust_network" class="tag tust">校园网</span>
    </div>

    <!-- Credentials Form -->
    <div class="section">
      <label class="label">用户名</label>
      <input v-model="username" type="text" class="input" placeholder="学号/工号" autocomplete="username" />

      <label class="label">密码</label>
      <input v-model="password" type="password" class="input" placeholder="校园网密码" autocomplete="current-password" />

      <label class="label">网络类型</label>
      <select v-model="networkType" class="input">
        <option value="校园网">校园网</option>
        <option value="中国联通">中国联通</option>
      </select>

      <div class="btn-row">
        <button class="btn primary" @click="handleSave">保存凭据</button>
        <span v-if="saved" class="saved-hint">已保存</span>
      </div>
    </div>

    <!-- Actions -->
    <div class="section">
      <div class="btn-row">
        <button class="btn" :class="{ primary: !loading }" @click="handleLogin" :disabled="loading">
          {{ loading ? "登录中..." : "手动登录" }}
        </button>
        <button class="btn" :class="{ active: paused }" @click="togglePause">
          {{ paused ? "恢复自动登录" : "暂停自动登录" }}
        </button>
        <button class="btn" :class="{ active: ignoreSsid }" @click="toggleIgnoreSsid">
          {{ ignoreSsid ? "恢复SSID检测" : "忽略SSID" }}
        </button>
      </div>
    </div>

    <!-- Login Result -->
    <div v-if="loginResult" class="result" :class="{ success: loginResult.success, error: !loginResult.success }">
      {{ loginResult.message }}
    </div>
  </div>
</template>

<style scoped>
.app {
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

.status-bar {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: #f0f0f0;
  border-radius: 6px;
  font-size: 13px;
  margin-bottom: 16px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #ccc;
}

.status-dot.online {
  background: #4caf50;
}

.sep {
  color: #ccc;
}

.tag {
  font-size: 11px;
  padding: 1px 6px;
  border-radius: 4px;
  margin-left: auto;
}

.tag.tust {
  background: #e3f2fd;
  color: #1976d2;
}

.dim {
  color: #999;
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
  display: block;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 4px;
  color: #555;
}

.input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 14px;
  margin-bottom: 10px;
  box-sizing: border-box;
  outline: none;
  transition: border-color 0.2s;
}

.input:focus {
  border-color: #1976d2;
}

select.input {
  appearance: none;
  -webkit-appearance: none;
  background: #fff url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='8'%3E%3Cpath d='M1 1l5 5 5-5' stroke='currentColor' stroke-width='1.5' fill='none'/%3E%3C/svg%3E") no-repeat right 10px center;
  padding-right: 30px;
  cursor: pointer;
  color: #333;
}

@media (prefers-color-scheme: dark) {
  select.input {
    background-color: #333;
    color: #ccc;
    border-color: #555;
  }
}

.btn-row {
  display: flex;
  gap: 8px;
  align-items: center;
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

.btn.primary {
  background: #1976d2;
  color: #fff;
  border-color: #1976d2;
}

.btn.primary:hover {
  background: #1565c0;
}

.btn.active {
  background: #ff9800;
  color: #fff;
  border-color: #ff9800;
}

.btn.sm {
  padding: 4px 10px;
  font-size: 12px;
}

.btn:disabled {
  opacity: 0.6;
  cursor: default;
}

.saved-hint {
  font-size: 12px;
  color: #4caf50;
}

.result {
  padding: 10px 14px;
  border-radius: 6px;
  font-size: 13px;
  margin-bottom: 16px;
}

.result.success {
  background: #e8f5e9;
  color: #2e7d32;
}

.result.error {
  background: #ffebee;
  color: #c62828;
}
</style>
