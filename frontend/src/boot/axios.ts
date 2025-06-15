import { defineBoot } from '#q-app/wrappers';
import axios, { type AxiosInstance } from 'axios';

const BASE_URL = 'https://urlshortenerbackend-production.up.railway.app'; //import.meta.env.VITE_BASE_URL; 

declare module 'vue' {
  interface ComponentCustomProperties {
    $axios: AxiosInstance;
    $api: AxiosInstance;
  }
}

const api = axios.create({ baseURL: BASE_URL });

export default defineBoot(({ app }) => {
  app.config.globalProperties.$axios = axios;
  app.config.globalProperties.$api = api;
});

export { api };
