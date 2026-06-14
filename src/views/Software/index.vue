<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { RefreshCw, Search, Package2, Calendar, HardDrive, Building, Folder } from '@lucide/vue'
import { InputGroup, InputGroupAddon, InputGroupInput } from '@/components/ui/input-group'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'
import Loading from '@/components/common/loading/index.vue'
import Segmented from '@/components/common/Segmented/index.vue'
import { Item, ItemContent, ItemDescription, ItemMedia, ItemTitle, ItemActions } from '@/components/ui/item'

interface SoftwareInfo {
    name: string
    version: string
    publisher: string
    install_location: string
    install_date: string
    size_kb: number
    icon: string
}

const apps = ref<SoftwareInfo[]>([])
const loading = ref(true)
const search = ref('')
const sortBy = ref<'name' | 'date' | 'size'>('name')

const AVATAR_COLORS = [
    '#6366f1', '#8b5cf6', '#ec4899', '#f97316',
    '#10b981', '#3b82f6', '#f59e0b', '#14b8a6',
    '#ef4444', '#a855f7',
]

function avatarColor(name: string): string {
    let hash = 0
    for (let i = 0; i < name.length; i++) hash = name.charCodeAt(i) + ((hash << 5) - hash)
    return AVATAR_COLORS[Math.abs(hash) % AVATAR_COLORS.length]
}

function avatarText(name: string): string {
    const trimmed = name.trim()
    // Try to get first letter of first two words
    const words = trimmed.split(/\s+/).filter(w => /[a-zA-Z一-龥]/.test(w))
    if (words.length >= 2) return (words[0][0] + words[1][0]).toUpperCase()
    return trimmed.slice(0, 2).toUpperCase()
}

function formatSize(kb: number): string {
    if (!kb || kb === 0) return ''
    if (kb < 1024) return `${kb} KB`
    if (kb < 1024 * 1024) return `${(kb / 1024).toFixed(1)} MB`
    return `${(kb / 1024 / 1024).toFixed(2)} GB`
}

const filtered = computed(() => {
    let list = apps.value
    const q = search.value.trim().toLowerCase()
    if (q) {
        list = list.filter(a =>
            a.name.toLowerCase().includes(q) ||
            a.publisher.toLowerCase().includes(q)
        )
    }
    return [...list].sort((a, b) => {
        if (sortBy.value === 'name') return a.name.toLowerCase().localeCompare(b.name.toLowerCase())
        if (sortBy.value === 'date') return (b.install_date || '').localeCompare(a.install_date || '')
        if (sortBy.value === 'size') return (b.size_kb || 0) - (a.size_kb || 0)
        return 0
    })
})

const totalWithDate = computed(() => apps.value.filter(a => a.install_date).length)
const totalWithSize = computed(() => apps.value.filter(a => a.size_kb > 0).length)

async function fetchApps() {
    loading.value = true
    try {
        apps.value = await invoke<SoftwareInfo[]>('get_installed_software')
        console.log(apps.value)
    } finally {
        loading.value = false
    }
}

onMounted(fetchApps)
</script>

<template>
    <div class="flex flex-col h-full gap-4">
        <!-- Top bar -->
        <div class="flex items-center justify-between shrink-0">
            <div class="flex items-center gap-4">
                <div class="flex items-center gap-2">
                    <Package2 class="text-primary" />
                    <span class="text-xl font-bold text-foreground">系统软件</span>
                </div>
                <div v-if="!loading" class="flex items-center">
                    <Badge>{{ apps.length }} 个</Badge>
                </div>
            </div>

            <Button variant="outline" :disabled="loading" @click="fetchApps">
                <RefreshCw :class="loading ? 'animate-spin' : ''" />
                重新扫描
            </Button>
        </div>

        <!-- Search + Sort -->
        <div class="flex items-center gap-2 shrink-0">
            <!-- Search -->
            <InputGroup>
                <InputGroupInput v-model="search" placeholder="输入关键字搜索" />
                <InputGroupAddon>
                    <Search />
                </InputGroupAddon>
            </InputGroup>

            <!-- Sort -->
            <Segmented v-model="sortBy"
                :options="[{ value: 'name', label: '名称' }, { value: 'date', label: '安装时间' }, { value: 'size', label: '大小' }]" />
        </div>

        <!-- List Card -->
        <Card class="relative flex-1 min-h-0 flex flex-col">
            <CardContent class="flex-1 min-h-0 p-0">
                <Loading v-if="loading" text="正在扫描已安装软件…" />
                <template v-else>
                    <!-- Empty search result -->
                    <div v-if="filtered.length === 0"
                        class="flex flex-col items-center justify-center h-full gap-3 text-muted-foreground">
                        <Package2 class="w-12 h-12 opacity-20" />
                        <p class="text-sm">未找到匹配的软件</p>
                    </div>

                    <ScrollArea v-else class="h-full">
                        <Item v-for="app in filtered" :key="app.name" size="sm"
                            class="hover:bg-accent/40 cursor-pointer">
                            <ItemMedia class="size-12 rounded-lg overflow-hidden"
                                :style="!app.icon ? { backgroundColor: avatarColor(app.name) } : {}">
                                <img v-if="app.icon" :src="app.icon" class="size-full object-contain" />
                                <span v-else
                                    class="size-full flex items-center justify-center text-white text-xs font-bold select-none">
                                    {{ avatarText(app.name) }}
                                </span>
                            </ItemMedia>

                            <ItemContent>
                                <ItemTitle>
                                    <span class="text-base font-bold">{{ app.name }}</span>
                                    <Badge v-if="app.version" variant="secondary">
                                        V {{ app.version }}
                                    </Badge>
                                </ItemTitle>
                                <ItemDescription v-if="app.publisher || app.install_location" class="flex gap-x-2">
                                    <Badge variant="outline" v-if="app.publisher">
                                        <Building />
                                        <span>{{ app.publisher }}</span>
                                    </Badge>
                                    <Badge variant="outline" v-if="app.install_location">
                                        <Folder />
                                        <span>{{ app.install_location }}</span>
                                    </Badge>
                                </ItemDescription>
                            </ItemContent>
                            <!-- <ItemContent class="text-muted-foreground gap-1">
                                
                            </ItemContent> -->
                            <ItemActions>
                                <!-- <Button variant="outline" size="sm">
                                    Action
                                </Button> -->
                            </ItemActions>
                        </Item>
                        <p class="px-4 py-2 border-t flex items-center gap-4 text-sm text-muted-foreground">
                            <span>共 {{ filtered.length }} 个软件{{ search ? `（过滤自 ${apps.length} 个）` : '' }}</span>
                            <span v-if="totalWithDate > 0">{{ totalWithDate }} 个有安装日期</span>
                            <span v-if="totalWithSize > 0">{{ totalWithSize }} 个有大小信息</span>
                        </p>
                    </ScrollArea>
                </template>
            </CardContent>
        </Card>
    </div>
</template>
