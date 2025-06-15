<template>
  <q-page class="modern-bg min-h-screen relative overflow-hidden">

    <FloatingCircles />
    
    <div class="flex flex-center min-h-screen q-pa-lg relative z-10">
      <div class="max-w-lg w-full">
        <div class="text-center q-mb-xl">
          <div class="hero-icon q-mb-md">
            <q-icon name="link" size="64px" color="white" />
          </div>
          <h1 class="hero-title q-mb-sm">LinkCut</h1>
          <p class="hero-subtitle">
            Transforme URLs longas em links poderosos e elegantes
          </p>
        </div>

        <q-card class="main-card shadow-24 q-mb-lg">
          <q-card-section class="q-pa-xl">
            <q-form @submit="shortenUrl" class="q-gutter-lg">
              <div class="input-container">
                <q-input
                  v-model="originalUrl"
                  type="url"
                  placeholder="https://exemplo.com/sua-url-muito-longa-aqui"
                  class="modern-input"
                  outlined
                  :rules="[val => !!val || 'URL é obrigatória', isValidUrl]"
                >
                  <template v-slot:prepend>
                    <q-icon name="language" color="primary" />
                  </template>
                  <template v-slot:append v-if="originalUrl && isUrlValid">
                    <q-icon name="check_circle" color="positive" />
                  </template>
                </q-input>
              </div>

              <q-btn
                type="submit"
                :loading="loading"
                :disable="!originalUrl || !isUrlValid"
                class="modern-btn full-width"
                size="lg"
                no-caps
              >
                <q-icon name="compress" class="q-mr-sm" />
                {{ loading ? 'Encurtando...' : 'Encurtar URL' }}
              </q-btn>
            </q-form>
          </q-card-section>
        </q-card>

        <q-card v-if="showResult" class="result-card shadow-24" v-animate-css="'fadeInUp'">
          <q-card-section class="q-pa-xl">
            <div class="result-header q-mb-lg text-center">
              <div class="success-icon q-mb-sm">
                <q-icon name="check_circle" size="32px" color="white" />
              </div>
              <h3 class="result-title">URL Encurtada!</h3>
              <p class="result-subtitle">Sua nova URL está pronta para uso</p>
            </div>

            <div class="url-display q-mb-lg">
              <div class="url-item q-mb-md">
                <div class="url-label">Original</div>
                <div class="url-value original-url">{{ truncateUrl(originalUrl, 60) }}</div>
              </div>
              <div class="url-item">
                <div class="url-label">Encurtada</div>
                <div class="url-value shortened-url">{{ shortenedUrl }}</div>
              </div>
            </div>

            <div class="action-buttons q-gutter-sm">
              <q-btn
                @click="copyToClipboard"
                class="action-btn primary-btn"
                no-caps
                :ripple="{ color: 'white' }"
              >
                <q-icon name="content_copy" class="q-mr-xs" />
                Copiar Link
              </q-btn>
              <q-btn
                @click="openUrl"
                class="action-btn secondary-btn"
                no-caps
                :ripple="{ color: 'brown-8' }"
              >
                <q-icon name="open_in_new" class="q-mr-xs" />
                Abrir
              </q-btn>
              <q-btn
                @click="shareUrl"
                class="action-btn tertiary-btn"
                no-caps
                :ripple="{ color: 'white' }"
              >
                <q-icon name="share" class="q-mr-xs" />
                Compartilhar
              </q-btn>
            </div>

            <div class="stats-container q-mt-lg">
              <div class="stats-grid">
                <div class="stat-item">
                  <div class="stat-number">{{ stats.saves }}</div>
                  <div class="stat-label">Economia</div>
                </div>
                <div class="stat-item">
                  <div class="stat-number">{{ stats.percentage }}%</div>
                  <div class="stat-label">Redução</div>
                </div>
              </div>
            </div>
          </q-card-section>
        </q-card>
      </div>
    </div>
  </q-page>
</template>

<script setup>
import { ref, computed, nextTick } from 'vue'
import { useQuasar } from 'quasar'
import FloatingCircles from 'src/components/FloatingCircles.vue';

const CREATE_LINK_API = import.meta.env.VITE_CREATE_LINK_API?.replace(/"/g, '') ?? '';

const $q = useQuasar()

const originalUrl = ref('')
const shortenedUrl = ref('')
const loading = ref(false)
const showResult = ref(false)

const stats = ref({
  saves: '0 chars',
  percentage: 0
})

const isUrlValid = computed(() => {
  return originalUrl.value.trim() && isValidUrl(originalUrl.value.trim())
})

const isValidUrl = (url) => {
  if (!url) return 'URL é obrigatória'
  try {
    const urlObj = new URL(url)
    return ['https:'].includes(urlObj.protocol) || 'URL deve começar com https://'
  } catch {
    return 'Formato de URL inválido'
  }
}

const truncateUrl = (url, maxLength) => {
  if (!url || url.length <= maxLength) return url
  return url.substring(0, maxLength) + '...'
}

async function getShortUrl() {
  try {
    const payload = {
      original_lnk: originalUrl
    }

    const response = await api.get(CREATE_LINK_API, {params: payload});

    if (response.status == 200) {
      return response.data.short_lnk
    }
    $q.notify({
      message: `Ops... Ocorreu um erro ao encurtar essa URL. Por favor tente novamente mais tarde`,
      type: 'negative',
      position: 'top-right',
      timeout: 4000,
      actions: [{ icon: 'close', color: 'white' }]
    }) 
  } catch {
    $q.notify({
      message: `Ops... Ocorreu um erro ao encurtar essa URL. Por favor tente novamente mais tarde`,
      type: 'negative',
      position: 'top-right',
      timeout: 4000,
      actions: [{ icon: 'close', color: 'white' }]
    })
  }
}

const shortenUrl = async () => {
  if (!isUrlValid.value) return

  loading.value = true
  
  try {
    const newShortUrl = await getShortUrl()
    
    shortenedUrl.value = newShortUrl
    
    const savedChars = originalUrl.value.length - newShortUrl.length
    stats.value = {
      saves: `${savedChars} chars`,
      percentage: Math.round((savedChars / originalUrl.value.length) * 100)
    }

    showResult.value = true
    await nextTick()
    
    $q.notify({
      message: 'URL encurtada com sucesso!',
      type: 'positive',
      position: 'top-right',
      timeout: 3000,
      actions: [{ icon: 'close', color: 'white' }]
    })

  } catch (error) {
    $q.notify({
      message: `Ops! Essa URL é inválida`,
      type: 'negative',
      position: 'top-right',
      timeout: 4000,
      actions: [{ icon: 'close', color: 'white' }]
    })
  } finally {
    loading.value = false
  }
}

const copyToClipboard = async () => {
  try {
    await navigator.clipboard.writeText(shortenedUrl.value)
    $q.notify({
      message: 'Link copiado!',
      type: 'positive',
      position: 'top-right',
      timeout: 2000,
      actions: [{ icon: 'close', color: 'white' }]
    })
  } catch {
    $q.notify({
      message: 'Erro ao copiar o link',
      type: 'negative',
      position: 'top-right',
      timeout: 3000
    })
  }
}

const openUrl = () => {
  window.open(shortenedUrl.value, '_blank')
}

const shareUrl = async () => {
  if (navigator.share) {
    try {
      await navigator.share({
        title: 'Link Encurtado',
        url: shortenedUrl.value
      })
    } catch {
      copyToClipboard()
    }
  } else {
    copyToClipboard()
  }
}
</script>

<style scoped>
.modern-bg {
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 25%, #334155 50%, #475569 100%);
  position: relative;
}

.hero-icon {
  background: linear-gradient(45deg, #3b82f6, #8b5cf6, #06b6d4);
  width: 100px;
  height: 100px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto;
  box-shadow: 0 20px 40px rgba(59, 130, 246, 0.3);
}

.hero-title {
  font-size: 3.5rem;
  font-weight: 800;
  background: linear-gradient(45deg, #fff, #3b82f6, #06b6d4);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  margin: 0;
  letter-spacing: -2px;
}

.hero-subtitle {
  color: rgba(255, 255, 255, 0.9);
  font-size: 1.2rem;
  font-weight: 300;
  margin: 0;
}

.main-card, .result-card {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 24px;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.modern-input :deep(.q-field__control) {
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  border: 2px solid transparent;
  transition: all 0.3s ease;
}

.modern-input :deep(.q-field__control):hover {
  border-color: #3b82f6;
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(59, 130, 246, 0.25);
}

.modern-input :deep(.q-field--focused .q-field__control) {
  border-color: #3b82f6;
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(59, 130, 246, 0.35);
}

.modern-btn {
  background: linear-gradient(45deg, #3b82f6, #8b5cf6);
  border-radius: 16px;
  font-weight: 600;
  font-size: 1.1rem;
  color: white;
  transition: all 0.3s ease;
}

.modern-btn:hover {
  transform: translateY(-3px);
  box-shadow: 0 15px 35px rgba(59, 130, 246, 0.5);
  background: linear-gradient(45deg, #2563eb, #7c3aed);
}

.success-icon {
  background: linear-gradient(45deg, #10b981, #059669);
  width: 60px;
  height: 60px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto;
}

.result-title {
  font-size: 2rem;
  font-weight: 700;
  color: #1f2937;
  margin: 0;
}

.result-subtitle {
  color: #6b7280;
  font-size: 1rem;
  margin: 0;
}

.url-display {
  background: #f8fafc;
  border-radius: 16px;
  padding: 24px;
  border: 1px solid #e2e8f0;
}

.url-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.url-label {
  font-size: 0.875rem;
  font-weight: 600;
  color: #4b5563;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.url-value {
  font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
  font-size: 0.95rem;
  padding: 12px 16px;
  border-radius: 12px;
  word-break: break-all;
}

.original-url {
  background: #fef2f2;
  color: #dc2626;
  border: 1px solid #fecaca;
}

.shortened-url {
  background: #f0fdf4;
  color: #16a34a;
  border: 1px solid #bbf7d0;
  font-weight: 600;
}

.action-buttons {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.action-btn {
  border-radius: 12px;
  font-weight: 600;
  padding: 12px 24px;
  transition: all 0.3s ease;
  flex: 1;
  min-width: 120px;
}

.primary-btn {
  background: linear-gradient(45deg, #3b82f6, #1d4ed8);
  color: white;
}

.secondary-btn {
  background: linear-gradient(45deg, #f59e0b, #d97706);
  color: white;
}

.tertiary-btn {
  background: linear-gradient(45deg, #10b981, #059669);
  color: white;
}

.action-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
}

.stats-container {
  background: linear-gradient(45deg, #1f2937, #374151);
  border-radius: 16px;
  padding: 24px;
  color: white;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
  text-align: center;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.stat-number {
  font-size: 2rem;
  font-weight: 800;
  color: #3b82f6;
  line-height: 1;
}

.stat-label {
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.8);
  margin-top: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.flex {
  display: flex;
}

.flex-center {
  justify-content: center;
  align-items: center;
}

.min-h-screen {
  min-height: 100vh;
}

.max-w-lg {
  max-width: 32rem;
}

.w-full {
  width: 100%;
}

.text-center {
  text-align: center;
}

.relative {
  position: relative;
}

.z-10 {
  z-index: 10;
}

.full-width {
  width: 100%;
}

.shadow-24 {
  box-shadow: 0 24px 48px rgba(0, 0, 0, 0.15);
}

.overflow-hidden {
  overflow: hidden;
}

@media (max-width: 768px) {
  .hero-title {
    font-size: 2.5rem;
  }
  
  .hero-subtitle {
    font-size: 1rem;
  }
  
  .action-buttons {
    flex-direction: column;
  }
  
  .action-btn {
    min-width: unset;
  }
  
  .stats-grid {
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
  }
  
  .stat-number {
    font-size: 1.5rem;
  }
}
</style>