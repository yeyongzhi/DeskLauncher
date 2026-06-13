<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import dayjs from 'dayjs'
import 'dayjs/locale/zh-cn'

dayjs.locale('zh-cn')

const now = ref(dayjs())
let timer: ReturnType<typeof setInterval>

onMounted(() => { timer = setInterval(() => { now.value = dayjs() }, 1000) })
onUnmounted(() => clearInterval(timer))

const timeStr = computed(() => now.value.format('HH:mm:ss'))
const WEEK = ['日', '一', '二', '三', '四', '五', '六']
const dateStr = computed(() => `${now.value.format('YYYY年MM月DD日')} 周${WEEK[now.value.day()]}`)

</script>

<template>
    <div>
        <p class="text-4xl font-bold tabular-nums tracking-tight">{{ timeStr }}</p>
        <p class="text-base text-muted-foreground mt-1">{{ dateStr }}</p>
    </div>
</template>
