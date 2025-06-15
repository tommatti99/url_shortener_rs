<template>
    <div>
        Redirecionando...
    </div>
</template>

<script setup lang="ts">
    import { useQuasar } from 'quasar';
    import { api } from 'src/boot/axios';
    import { onMounted } from 'vue';
    import { useRouter } from 'vue-router';

    const GET_FULL_LINK_API = import.meta.env.VITE_GET_FULL_LINK_API?.replace(/"/g, '') ?? '';
    const router = useRouter();
    const $q = useQuasar()

    onMounted( async () => {
        await goToFullLink();
    })


    async function goToFullLink() {
        try {
            const payload = {
                short_link: router.currentRoute.value.path.replace(/[^a-zA-Z0-9]/g, '')
            }
            const response = await api.post(GET_FULL_LINK_API, payload );

            if (response.status == 200) {
                window.location.href = response.data.data.original_link;
            }
        } catch(error) {
            window.location.href = window.location.origin
        }
    }
</script>

<style>
</style>