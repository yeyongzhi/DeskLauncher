<script setup lang="ts">
import { Search } from '@lucide/vue'
import { InputGroup, InputGroupAddon, InputGroupInput } from '@/components/ui/input-group'
import { Card, CardContent } from '@/components/ui/card'
import DateTime from './components/DateTime/index.vue'

const quote = '种一棵树最好的时间是十年前，其次是现在'
</script>

<template>
    <div class="flex flex-col gap-4">
        <!-- 顶部两卡 -->
        <div class="flex gap-4">
            <Card class="flex-1">
                <CardContent>
                    <DateTime />
                </CardContent>
            </Card>
            <Card class="flex-1">
                <CardContent>
                    <p class="text-sm text-muted-foreground leading-relaxed italic">" {{ quote }} "</p>
                </CardContent>
            </Card>
        </div>

        <!-- 搜索栏 -->
        <InputGroup>
            <InputGroupInput placeholder="Search..." />
            <InputGroupAddon>
                <Search />
            </InputGroupAddon>
        </InputGroup>
    </div>
</template>
