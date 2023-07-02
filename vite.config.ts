import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";
import AutoImport from "unplugin-auto-import/vite";
import AutoImportComponents from "unplugin-vue-components/vite";

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        AutoImport({
            imports: ["vue", "@vueuse/core", "vue-i18n"],
            dirs: ["src/hooks"],
            dts: "src/auto-import.d.ts",
        }),
        AutoImportComponents({
            // 指定组件位置，默认是src/components
            // dirs: ["src/components", "src/views"],
            // Glob patterns to match file names to be detected as components.
            // When specified, the `dirs` and `extensions` options will be ignored.
            globs: ["src/components/*.{vue}"],
            // ui库解析器
            // resolvers: [ElementPlusResolver()],
            extensions: ["vue"],
            // 配置文件生成位置
            dts: "src/components.d.ts",
            version: 3,
        }),
        vue(),
    ],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
    },
    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ["VITE_", "TAURI_"],
    build: {
        // Tauri supports es2021
        target:
            process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari14",
        // don't minify for debug builds
        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        // produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_DEBUG,
        rollupOptions: {
            treeshake: {
                tryCatchDeoptimization: false,
            },
        },
    },
    resolve: {
        extensions: [".tsx", ".ts", ".jsx", ".js"],
        alias: {
            "@": path.resolve(__dirname, "src"),
        },
    },
    css: {
        preprocessorOptions: {
            scss: {
                // additionalData: ``,
            },
        },
    },
});
