import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  server: {
    hmr: {
      port: 5174
    }
  },
  define: {
    __version__: JSON.stringify(process.env.npm_package_version)
  }
});
