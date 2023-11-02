import { build } from "esbuild/mod.js";
import { denoPlugins } from "esbuild_deno_loader/mod.ts";
import { resolve } from "path/mod.ts"

await build({
  plugins: denoPlugins({ configPath: resolve("deno.json") }),
  entryPoints: ["./src/index.jsx"],
  outdir: ".dist/",
  bundle: true,
  minify: true,
  jsx: "automatic",
  jsxImportSource: "npm:/preact"
});

Deno.exit(0);