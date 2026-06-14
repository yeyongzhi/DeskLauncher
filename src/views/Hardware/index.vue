<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { RefreshCw, Copy, Check } from '@lucide/vue'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import Segmented from '@/components/common/Segmented/index.vue'
import { Table, TableBody, TableCell, TableRow } from '@/components/ui/table'
import { ScrollArea } from '@/components/ui/scroll-area'
import Loading from '@/components/common/loading/index.vue'

// ── Overview ──────────────────────────────────
interface HardwareInfo {
    computer_name: string
    computer_model: string; os: string; cpu: string
    motherboard: string; memory: string; main_disk: string
    gpu: string; monitor: string; sound_card: string; network_card: string
}

// ── Detail ────────────────────────────────────
interface GpuItem   { name: string; vendor: string; vram: string; video_processor: string; driver_version: string; current_resolution: string; refresh_rate: string }
interface MemStick  { name: string; vendor: string; size: string; speed: string; slot: string; data_width: string }
interface CpuDetail { name: string; vendor: string; cores: string; max_speed: string; ext_clock: string; current_speed: string; l2_cache: string; l3_cache: string; voltage: string; processor_id: string; data_width: string; temperature: string }
interface MbDetail  { name: string; vendor: string; serial: string; bios_vendor: string; bios_version: string; bios_date: string; computer_model: string; computer_vendor: string; install_date: string; last_boot: string }
interface DiskItem  { name: string; vendor: string; size: string; is_primary: string; serial: string }
interface MonItem   { name: string; vendor: string; code: string; size: string; manufacture_date: string }
interface NetItem   { name: string; vendor: string; mac: string }
interface SndItem   { name: string; vendor: string }
interface HardwareDetail { gpus: GpuItem[]; memory_sticks: MemStick[]; cpu: CpuDetail; motherboard: MbDetail; disks: DiskItem[]; monitors: MonItem[]; networks: NetItem[]; sounds: SndItem[] }

// ── State ─────────────────────────────────────
const info      = ref<HardwareInfo | null>(null)
const detail    = ref<HardwareDetail | null>(null)
const loading   = ref(true)
const copied    = ref(false)
const activeTab = ref('overview')

const tabDefs = [
    { value: 'overview',    label: '总览'  },
    { value: 'gpu',         label: '显卡'  },
    { value: 'memory',      label: '内存'  },
    { value: 'cpu',         label: 'CPU'   },
    { value: 'motherboard', label: '主板'  },
    { value: 'disk',        label: '硬盘'  },
    { value: 'monitor',     label: '显示器' },
    { value: 'network',     label: '网卡'  },
    { value: 'sound',       label: '声卡'  },
]

const currentTabName = computed(() => tabDefs.find(t => t.value === activeTab.value)?.label ?? '')

const overviewItems: { key: keyof Omit<HardwareInfo, 'computer_name'>; label: string }[] = [
    { key: 'computer_model', label: '电脑型号' }, { key: 'os',          label: '操作系统' },
    { key: 'cpu',            label: 'CPU'      }, { key: 'motherboard', label: '主板'     },
    { key: 'memory',         label: '内存'      }, { key: 'main_disk',   label: '主硬盘'   },
    { key: 'gpu',            label: '显卡'      }, { key: 'monitor',     label: '显示器'   },
    { key: 'sound_card',     label: '声卡'      }, { key: 'network_card',label: '网卡'     },
]

type Row = [string, string | undefined]
function clean(pairs: Row[]): Row[] {
    return pairs.filter(([, v]) => v && v !== 'N/A' && v !== '')
}

// ── Rows for detail tabs ───────────────────────
const detailRows = computed<Row[]>(() => {
    const d = detail.value
    if (!d) return []
    switch (activeTab.value) {
        case 'gpu':
            return d.gpus.flatMap((g, i): Row[] => [
                [`显卡 ${i + 1}`, undefined],
                ['显卡名称', g.name], ['厂商', g.vendor], ['显存', g.vram],
                ['内核', g.video_processor], ['驱动版本', g.driver_version],
                ['当前分辨率', g.current_resolution], ['刷新频率', g.refresh_rate],
            ])
        case 'memory':
            return d.memory_sticks.flatMap((m, i): Row[] => [
                [`内存 ${i + 1}`, undefined],
                ['名称', m.name], ['厂商', m.vendor], ['大小', m.size],
                ['频率', m.speed], ['插槽', m.slot], ['数据宽度', m.data_width],
            ])
        case 'cpu':
            return clean([
                ['CPU名称', d.cpu.name], ['温度', d.cpu.temperature], ['厂商', d.cpu.vendor],
                ['核心数', d.cpu.cores], ['默认频率', d.cpu.max_speed], ['外频', d.cpu.ext_clock],
                ['当前频率', d.cpu.current_speed], ['二级缓存', d.cpu.l2_cache],
                ['三级缓存', d.cpu.l3_cache], ['CPU电压', d.cpu.voltage],
                ['CPU序列号', d.cpu.processor_id], ['数据宽度', d.cpu.data_width],
            ])
        case 'motherboard':
            return clean([
                ['主板名称', d.motherboard.name], ['主板厂商', d.motherboard.vendor],
                ['主板序列号', d.motherboard.serial], ['BIOS厂商', d.motherboard.bios_vendor],
                ['BIOS版本', d.motherboard.bios_version], ['BIOS日期', d.motherboard.bios_date],
                ['电脑型号', d.motherboard.computer_model], ['电脑厂商', d.motherboard.computer_vendor],
                ['系统安装日期', d.motherboard.install_date], ['最近启动', d.motherboard.last_boot],
            ])
        case 'disk':
            return d.disks.flatMap((dk, i): Row[] => [
                [`硬盘 ${i + 1}`, undefined],
                ['名称', dk.name], ['厂商', dk.vendor], ['大小', dk.size],
                ['主硬盘', dk.is_primary], ['序列号', dk.serial],
            ])
        case 'monitor':
            return d.monitors.flatMap((m, i): Row[] => [
                [`显示器 ${i + 1}`, undefined],
                ['名称', m.name], ['厂商', m.vendor], ['代号', m.code],
                ['尺寸', m.size], ['制造日期', m.manufacture_date],
            ])
        case 'network':
            return d.networks.flatMap((n, i): Row[] => [
                [`网卡 ${i + 1}`, undefined],
                ['名称', n.name], ['厂商', n.vendor], ['MAC地址', n.mac],
            ])
        case 'sound':
            return d.sounds.flatMap((s, i): Row[] => [
                [`声卡 ${i + 1}`, undefined],
                ['名称', s.name], ['厂商', s.vendor],
            ])
        default:
            return []
    }
})

// ── Actions ───────────────────────────────────
async function fetchInfo() {
    loading.value = true
    try {
        const [i, d] = await Promise.all([
            invoke<HardwareInfo>('get_hardware_info'),
            invoke<HardwareDetail>('get_hardware_detail'),
        ])
        info.value = i
        detail.value = d
    } finally {
        loading.value = false
    }
}

async function copyInfo() {
    if (!info.value) return
    const lines = [
        `主机名称        ${info.value.computer_name}`,
        ...overviewItems.map(it => `${it.label.padEnd(8)}    ${info.value![it.key] || '-'}`),
    ]
    await navigator.clipboard.writeText(lines.join('\n'))
    copied.value = true
    setTimeout(() => { copied.value = false }, 2000)
}

onMounted(fetchInfo)
</script>

<template>
    <div class="flex flex-col h-full gap-4">
        <!-- Top bar -->
        <div class="flex items-center justify-between shrink-0">
            <div class="flex flex-col items-start gap-3">
                <div v-if="info" class="text-2xl font-bold text-foreground">🖥️ {{ info.computer_name }}</div>
                <Segmented v-model="activeTab" :options="tabDefs" />
            </div>
            <div class="flex items-center gap-2">
                <Button variant="ghost" size="sm" :disabled="loading" @click="fetchInfo">
                    <RefreshCw class="w-3.5 h-3.5" :class="loading ? 'animate-spin' : ''" />
                    重新检测
                </Button>
                <Button variant="outline" size="sm" :disabled="loading" @click="copyInfo">
                    <component :is="copied ? Check : Copy" class="w-3.5 h-3.5" :class="copied ? 'text-green-500' : ''" />
                    {{ copied ? '已复制' : '复制信息' }}
                </Button>
            </div>
        </div>

        <!-- Content Card -->
        <Card class="relative flex-1 min-h-0 w-full flex flex-col">
            <CardHeader class="shrink-0 border-b">
                <CardTitle class="text-xl">{{ currentTabName }}</CardTitle>
            </CardHeader>
            <CardContent class="flex-1 min-h-0 p-0">
                <Loading v-if="loading" text="正在读取硬件信息…" />
                <ScrollArea v-else-if="info" class="h-full">
                    <Table>
                        <TableBody>
                            <!-- 总览 -->
                            <template v-if="activeTab === 'overview'">
                                <TableRow>
                                    <TableCell class="w-36 text-muted-foreground pl-6">主机名称</TableCell>
                                    <TableCell class="font-medium">{{ info.computer_name }}</TableCell>
                                </TableRow>
                                <TableRow v-for="item in overviewItems" :key="item.key">
                                    <TableCell class="w-36 text-muted-foreground pl-6">{{ item.label }}</TableCell>
                                    <TableCell class="font-medium">{{ info[item.key] || '-' }}</TableCell>
                                </TableRow>
                            </template>

                            <!-- 详细 tabs -->
                            <template v-else>
                                <template v-for="(row, i) in detailRows" :key="i">
                                    <TableRow v-if="row[1] === undefined" class="hover:bg-transparent border-0">
                                        <TableCell colspan="2" :class="['text-xs font-semibold text-muted-foreground pl-6', i > 0 ? 'pt-5' : 'pt-3']">
                                            {{ row[0] }}
                                        </TableCell>
                                    </TableRow>
                                    <TableRow v-else>
                                        <TableCell class="w-36 text-muted-foreground pl-6">{{ row[0] }}</TableCell>
                                        <TableCell class="font-medium">{{ row[1] }}</TableCell>
                                    </TableRow>
                                </template>
                            </template>
                        </TableBody>
                    </Table>
                </ScrollArea>
            </CardContent>
        </Card>
    </div>
</template>
