/**
 * Extrae PNGs sin modificar ni redibujar la ilustración original.
 * Se ejecuta el HTML heredado en Chromium y se utiliza HF()/ES[] de su Canvas.
 * Dependencia solo de herramientas: npm install --no-save playwright-core.
 */
import { chromium } from 'playwright-core';
import { writeFile, mkdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const dest = path.join(root, 'apps', 'retro-realm', 'assets', 'sprites');
await mkdir(dest, { recursive: true });
const browser = await chromium.launch({
  channel: 'chrome',
  headless: true,
  args: ['--no-sandbox', '--disable-dev-shm-usage', '--autoplay-policy=no-user-gesture-required'],
});
try {
  const page = await browser.newPage({ viewport: { width: 960, height: 540 } });
  page.on('pageerror', error => console.log('[HTML original]', error.message.slice(0, 180)));
  await page.goto(new URL('file://' + path.join(root, 'Retro Realm.html')).href, {
    waitUntil: 'domcontentloaded',
    timeout: 60000,
  });
  await page.waitForFunction(() => typeof HF === 'function' && typeof ES !== 'undefined', null, {
    timeout: 30000,
  });
  const images = await page.evaluate(() => {
    const out = [];
    const add = (name, canvas) => {
      if (!canvas || typeof canvas.toDataURL !== 'function') return;
      const data = canvas.toDataURL('image/png');
      out.push({
        name,
        width: canvas.width,
        height: canvas.height,
        ax: Number(canvas.ax || 0),
        ay: Number(canvas.ay || 0),
        base64: data.split(',')[1],
      });
    };
    // 3 personajes, poses y variantes de habilidad.
    for (let cl = 0; cl < 3; cl++) {
      for (let phase = 0; phase < 9; phase++) {
        add('hero_' + cl + '_' + phase, HF(cl, phase, 0, 0, 0));
      }
      for (let slash = 0; slash < 3; slash++) {
        add('hero_attack_' + cl + '_' + slash, HF(cl, 9, 0, 2, slash));
      }
    }
    // Primeras criaturas. Las variantes avanzadas se exportarán por separado.
    for (let k = 0; k < ES.length; k++) {
      if (!Array.isArray(ES[k])) continue;
      for (let f = 0; f < ES[k].length; f++) add('enemy_' + k + '_' + f, ES[k][f]);
    }
    for (let j = 0; j < Math.min(4, TT.length); j++) add('terrain_grass_' + j, TT[j]);
    for (let j = 0; j < Math.min(4, TD.length); j++) add('terrain_soil_' + j, TD[j]);
    return out;
  });
  if (images.length < 30) throw Error('Exportación incompleta: ' + images.length + ' sprites');
  const index = {};
  for (const item of images) {
    const {name, width, height, ax, ay, base64} = item;
    await writeFile(path.join(dest, name + '.png'), Buffer.from(base64, 'base64'));
    index[name] = { width, height, ax, ay };
  }
  await writeFile(path.join(dest, 'index.json'), JSON.stringify(index, null, 2) + '\n');
  console.log('Exportados ' + images.length + ' sprites originales a ' + dest);
} finally {
  await browser.close();
}
