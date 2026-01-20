const path = require("path");
const fs = require("fs");
const esbuild = require("esbuild");

const root = path.join(__dirname, "..");
const srcDir = path.join(root, "src", "web", "ts");
const outDir = path.join(root, "src", "web", "assets");
const entries = ["app.ts", "home.ts", "note.ts", "signin.ts", "network.ts"].map((file) =>
  path.join(srcDir, file),
);

const cleanOutputs = () => {
  if (!fs.existsSync(outDir)) {
    fs.mkdirSync(outDir, { recursive: true });
    return;
  }
  fs.readdirSync(outDir)
    .filter((file) => file.endsWith(".js"))
    .forEach((file) => fs.unlinkSync(path.join(outDir, file)));
};

cleanOutputs();

esbuild
  .build({
    entryPoints: entries,
    bundle: true,
    format: "iife",
    platform: "browser",
    minify: true,
    target: "es2019",
    outdir: outDir,
  })
  .catch(() => process.exit(1));
