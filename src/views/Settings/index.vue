<script setup lang="ts">
import { ref } from 'vue'

const darkMode = ref(false)
const autoStart = ref(true)
const language = ref('zh-CN')
const gridSize = ref('medium')
</script>

<template>
  <div class="p-6 max-w-2xl">
    <div class="mb-6">
      <h1 class="text-xl font-semibold text-foreground">设置</h1>
      <p class="text-sm text-muted-foreground mt-0.5">管理你的应用偏好</p>
    </div>

    <div class="space-y-2">
      <!-- Appearance -->
      <section class="rounded-xl border border-border bg-card overflow-hidden">
        <div class="px-4 py-3 border-b border-border">
          <h2 class="text-sm font-medium text-foreground">外观</h2>
        </div>

        <div class="divide-y divide-border">
          <div class="flex items-center justify-between px-4 py-3">
            <div>
              <p class="text-sm text-foreground">深色模式</p>
              <p class="text-xs text-muted-foreground">切换深色/浅色主题</p>
            </div>
            <button
              @click="darkMode = !darkMode"
              :class="['relative w-10 h-6 rounded-full transition-colors', darkMode ? 'bg-primary' : 'bg-muted']"
            >
              <span
                :class="['absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform', darkMode ? 'translate-x-5' : 'translate-x-1']"
              />
            </button>
          </div>

          <div class="flex items-center justify-between px-4 py-3">
            <div>
              <p class="text-sm text-foreground">图标网格大小</p>
              <p class="text-xs text-muted-foreground">调整主页应用图标的显示大小</p>
            </div>
            <select
              v-model="gridSize"
              class="h-8 rounded-lg border border-border bg-background px-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-ring"
            >
              <option value="small">小</option>
              <option value="medium">中</option>
              <option value="large">大</option>
            </select>
          </div>
        </div>
      </section>

      <!-- General -->
      <section class="rounded-xl border border-border bg-card overflow-hidden">
        <div class="px-4 py-3 border-b border-border">
          <h2 class="text-sm font-medium text-foreground">通用</h2>
        </div>

        <div class="divide-y divide-border">
          <div class="flex items-center justify-between px-4 py-3">
            <div>
              <p class="text-sm text-foreground">开机自启动</p>
              <p class="text-xs text-muted-foreground">登录时自动启动 DeskLauncher</p>
            </div>
            <button
              @click="autoStart = !autoStart"
              :class="['relative w-10 h-6 rounded-full transition-colors', autoStart ? 'bg-primary' : 'bg-muted']"
            >
              <span
                :class="['absolute top-1 w-4 h-4 rounded-full bg-white shadow transition-transform', autoStart ? 'translate-x-5' : 'translate-x-1']"
              />
            </button>
          </div>

          <div class="flex items-center justify-between px-4 py-3">
            <div>
              <p class="text-sm text-foreground">语言</p>
              <p class="text-xs text-muted-foreground">选择界面显示语言</p>
            </div>
            <select
              v-model="language"
              class="h-8 rounded-lg border border-border bg-background px-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-ring"
            >
              <option value="zh-CN">简体中文</option>
              <option value="zh-TW">繁體中文</option>
              <option value="en-US">English</option>
            </select>
          </div>
        </div>
      </section>

      <!-- About -->
      <section class="rounded-xl border border-border bg-card overflow-hidden">
        <div class="px-4 py-3 border-b border-border">
          <h2 class="text-sm font-medium text-foreground">关于</h2>
        </div>
        <div class="flex items-center justify-between px-4 py-3">
          <div>
            <p class="text-sm text-foreground">DeskLauncher</p>
            <p class="text-xs text-muted-foreground">版本 0.1.0</p>
          </div>
          <button class="h-8 rounded-lg border border-border px-3 text-xs text-muted-foreground hover:bg-accent transition-colors">
            检查更新
          </button>
        </div>
      </section>
    </div>
  </div>
</template>
