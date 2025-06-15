<template>
    <div>
        Redirecionando...
    </div>
</template>

<script setup lang="ts">
    import { api } from 'src/boot/axios';
    import { onMounted } from 'vue';
    import { useRouter } from 'vue-router';

    const GET_FULL_LINK_API = import.meta.env.VITE_GET_FULL_LINK_API?.replace(/"/g, '') ?? '';
    const router = useRouter();

    onMounted( async () => {
        const fullLink: string = await getFullLink();

        window.location.href = fullLink;
    })


    async function getFullLink() {
        try {
            const payload = {
                short_lnk_url: router.currentRoute.value.path.replace(/[^a-zA-Z0-9]/g, '')
            }

            const response = await api.get(GET_FULL_LINK_API, {params: payload} );

            if (response.status == 200) {
                return response.data.short_lnk
            }
            return 'router.currentRoute.value.query'

        } catch {
            return 'router.currentRoute.value.query'
        }
    }
</script>

<style>
</style>