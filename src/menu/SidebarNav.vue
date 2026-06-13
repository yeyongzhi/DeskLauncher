<script setup lang="ts">
import { Home, Cpu } from '@lucide/vue'
import { RouterLink, useRoute } from 'vue-router'
import {
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from '@/components/ui/sidebar'

const route = useRoute()

const items = [
    { to: '/', name: 'home', icon: Home, label: '主页' },
    { to: '/hardware', name: 'hardware', icon: Cpu, label: '硬件信息' },
]
</script>

<template>
    <SidebarMenu>
        <SidebarMenuItem v-for="item in items" :key="item.name">
            <SidebarMenuButton
                as-child
                class="justify-center px-0"
                :class="{ 'bg-sidebar-accent': route.name === item.name }"
                :title="item.label"
            >
                <RouterLink :to="item.to">
                    <component :is="item.icon" />
                </RouterLink>
            </SidebarMenuButton>
        </SidebarMenuItem>
    </SidebarMenu>
</template>
