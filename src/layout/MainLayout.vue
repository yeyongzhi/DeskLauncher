<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarHeader,
    SidebarInset,
    SidebarProvider,
} from '@/components/ui/sidebar'
import { ScrollArea } from '@/components/ui/scroll-area'
import SidebarNav from '@/menu/SidebarNav.vue'

const sidebarOpen = ref(true)
const route = useRoute()
</script>

<template>
    <SidebarProvider v-model:open="sidebarOpen" style="--sidebar-width: 50px;" class="h-svh overflow-hidden">
        <Sidebar>
            <SidebarHeader class="items-center">
                <img src="/app-icon.png" class="w-8 h-8 rounded-lg" />
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupContent>
                        <SidebarNav />
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
        <SidebarInset class="overflow-hidden flex flex-col">
            <div class="flex-1 overflow-hidden p-4">
                <RouterView />
                <!-- <ScrollArea class="h-full p-4">
                    <RouterView />
                </ScrollArea> -->
            </div>
        </SidebarInset>
    </SidebarProvider>
</template>
