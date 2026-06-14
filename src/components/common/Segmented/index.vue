<script setup lang="ts">
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs'

interface Option {
    label: string
    value: string
}

const props = defineProps<{
    options: Option[]
}>()

const model = defineModel<string>()
</script>

<template>
    <Tabs v-model="model">
        <TabsList>
            <TabsTrigger
                class="px-4"
                v-for="opt in options"
                :key="opt.value"
                :value="opt.value"
            >
                {{ opt.label }}
            </TabsTrigger>
        </TabsList>
    </Tabs>
</template>
