<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import {
  FolderOpen, Monitor, Trash2, Download, Layout,
  FileText, Calculator, Paintbrush, Scissors, Pencil,
  Activity, SlidersHorizontal, Settings, Cpu, Terminal
} from '@lucide/vue'

const apps = [
  { name: '文件资源管理器', icon: FolderOpen,       cmd: 'explorer.exe' },
  { name: '我的电脑',       icon: Monitor,           cmd: 'explorer.exe ::{20D04FE0-3AEA-1069-A2D8-08002B30309D}' },
  { name: '回收站',         icon: Trash2,            cmd: 'explorer.exe ::{645FF040-5081-101B-9F08-00AA002F954E}' },
  { name: '下载',           icon: Download,          cmd: '%USERPROFILE%\\Downloads' },
  { name: '桌面',           icon: Layout,            cmd: '%USERPROFILE%\\Desktop' },
  { name: '记事本',         icon: FileText,          cmd: 'notepad.exe' },
  { name: '计算器',         icon: Calculator,        cmd: 'calc.exe' },
  { name: '画图',           icon: Paintbrush,        cmd: 'mspaint.exe' },
  { name: '截图工具',       icon: Scissors,          cmd: 'SnippingTool.exe' },
  { name: '写字板',         icon: Pencil,            cmd: 'wordpad.exe' },
  { name: '任务管理器',     icon: Activity,          cmd: 'taskmgr.exe' },
  { name: '控制面板',       icon: SlidersHorizontal, cmd: 'control.exe' },
  { name: 'Windows 设置',   icon: Settings,          cmd: 'ms-settings:' },
  { name: '设备管理器',     icon: Cpu,               cmd: 'devmgmt.msc' },
  { name: '命令提示符',     icon: Terminal,          cmd: 'cmd.exe' },
]

const launchApp = async (cmd: string) => {
  try {
    await invoke('launch_app', { app: cmd })
  } catch (e) {
    console.error('启动失败:', e)
  }
}
</script>

<template>
  <div class="grid grid-cols-8 gap-2">
    <button
      v-for="app in apps"
      :key="app.name"
      @click="launchApp(app.cmd)"
      class="flex flex-col items-center gap-1.5 p-2.5 rounded-lg hover:bg-accent transition-colors cursor-pointer group"
    >
      <component :is="app.icon" class="w-5 h-5 text-muted-foreground group-hover:text-foreground transition-colors" />
      <span class="text-[11px] text-muted-foreground group-hover:text-foreground text-center leading-tight transition-colors">{{ app.name }}</span>
    </button>
  </div>
</template>
