const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '..');
const srcHtml = path.join(root, 'index.html');
const srcJs = path.join(root, 'halo.js');
const distDir = path.join(root, 'dist');

function ensureFileExists(filePath) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`Missing source file: ${filePath}`);
  }
}

function inlineScript(html, scriptContent) {
  const scriptTag = '<script src="halo.js"></script>';
  if (!html.includes(scriptTag)) {
    throw new Error('Expected <script src="halo.js"></script> placeholder in index.html');
  }
  return html.replace(
    scriptTag,
    `<script>\n// Bundled by scripts/build.js\n${scriptContent}\n</script>`
  );
}

function main() {
  ensureFileExists(srcHtml);
  ensureFileExists(srcJs);

  const html = fs.readFileSync(srcHtml, 'utf8');
  const script = fs.readFileSync(srcJs, 'utf8');

  if (!fs.existsSync(distDir)) {
    fs.mkdirSync(distDir, { recursive: true });
  }

  const bundledHtml = inlineScript(html, script);
  const distHtml = path.join(distDir, 'index.html');
  const distScript = path.join(distDir, 'halo.js');

  fs.writeFileSync(distHtml, bundledHtml, 'utf8');
  fs.writeFileSync(distScript, script, 'utf8');

  console.log('Built dist/index.html (inlined halo.js)');
  console.log('Copied dist/halo.js for debugging');
}

main();
