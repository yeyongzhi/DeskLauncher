<script setup lang="ts">
import { Search } from '@lucide/vue'
import { InputGroup, InputGroupAddon, InputGroupInput } from '@/components/ui/input-group'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import DateTime from './components/DateTime/index.vue'
import SystemApps from './components/SystemApps/index.vue'

const quote = '种一棵树最好的时间是十年前，其次是现在'
</script>

<template>
    <div class="flex flex-col gap-4">
        <!-- 顶部两卡 -->
        <div class="flex gap-4">
            <Card class="flex-1 py-4">
                <CardContent>
                    <DateTime />
                </CardContent>
            </Card>
            <Card class="flex-1 py-4">
                <CardContent>
                    <p class="text-sm text-muted-foreground leading-relaxed italic">" {{ quote }} "</p>
                </CardContent>
            </Card>
        </div>

        <!-- 搜索栏 -->
        <InputGroup>
            <InputGroupInput placeholder="输入关键字搜索" />
            <InputGroupAddon>
                <Search />
            </InputGroupAddon>
        </InputGroup>

        <!-- 系统应用 -->
        <Card>
            <CardHeader>
                <CardTitle class="text-sm">系统应用</CardTitle>
            </CardHeader>
            <CardContent>
                <SystemApps />
            </CardContent>
        </Card>

    </div>
</template>
