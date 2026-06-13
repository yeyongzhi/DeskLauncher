<script setup lang="ts">
import { Loader2 } from '@lucide/vue'

defineProps<{
  text?: string
}>()
</script>

<template>
  <div class="w-full h-full flex items-center justify-center bg-background/80 backdrop-blur-sm z-10">
    <div class="flex flex-col items-center gap-3 text-muted-foreground">
      <Loader2 class="w-8 h-8 animate-spin" />
      <span class="text-sm">{{ text ?? '加载中…' }}</span>
    </div>
  </div>
</template>
