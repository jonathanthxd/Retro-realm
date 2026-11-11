/**
 * Extrae PNGs sin modificar ni redibujar la ilustración original.
 * Se ejecuta el HTML heredado en Chromium y se utiliza HF()/ES[] de su Canvas.
 * Dependencia solo de herramientas: npm install --no-save playwright-core.
 */
import { chromium } from 'playwright-core';
import { writeFile, mkdir, readFile } from 'node:fs/promises';
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
  // Instrumentación exclusiva del exportador, NO del HTML versionado.
  // Así podemos capturar también los Canvas de slimes, humanos y jefes cuyos
  // generadores viven dentro de una IIFE privada.
  const source = await readFile(path.join(root, 'Retro Realm.html'), 'utf8');
  const marker = 'function zKingPose(e){';
  if (!source.includes(marker)) throw Error('No se encontró el ancla de recursos del Rey');
  const instrumented = source.replace(marker,
    'window.__RETRO_EXPORT={SLR,OG,zHE2,zKF};' + marker);
  const base = new URL('file://' + root + '/').href;
  await page.setContent(instrumented.replace('<head>', '<head><base href="' + base + '">'), {
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
    // Exposición temporal dentro del navegador. Mantiene el HTML real intacto.
    const extra = window.__RETRO_EXPORT;
    if (!extra) throw Error('No se pudo acceder a los generadores de enemigos finales');
    extra.SLR.forEach((variants, index) => variants.forEach((sprite, frame) => {
      add('enemy_' + (20+index) + '_' + frame, sprite);
    }));
    for (let v = 0; v < 2; v++) {
      for (let f = 0; f < 4; f++) add('enemy_' + (60+v) + '_' + f,extra.OG[v].w[f]);
      for (let p of ['a', 's', 'r']) add('ogre_' + (60+v) + '_' + p,extra.OG[v][p]);
    }
    for (let k = 40; k <= 49; k++) {
      for (let f = 0; f < 9; f++) add('enemy_' + k + '_' + f,
        extra.zHE2(k,f,0,0,0));
      for (let f = 0; f < 3; f++) add('humanoid_attack_' + k + '_' + f,
        extra.zHE2(k,9,2,f,0));
    }
    for (const pose of ['i','w','s1','s2','s3','a','j','f','d','sl']) {
      for (let frame = 0; frame < (pose==='w'?4:2); frame++) {
        add('king_' + pose + '_' + frame, extra.zKF(pose, frame));
      }
    }
    if (typeof window.zSetTiles === 'function') {
      window.zSetTiles(1);
      for (let j=0; j<4; j++) {
        add('dark_grass_' + j,TT[j]);
        add('dark_soil_' + j,TD[j]);
      }
    }
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
