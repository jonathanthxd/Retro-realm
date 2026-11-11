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
    'window.__RETRO_EXPORT={SLR,OG,zHE2,zKF,CHD,ZT,ZB,ZG,ZRK,ZM,ZSG,ZGV,ZSTUMP,ZLOG,ZFENCE,FL,FL2,zDecoBuild,ZCB};' + marker);
  const base = new URL('file://' + root + '/').href;
  await page.setContent(instrumented.replace('<head>', '<head><base href="' + base + '">'), {
    waitUntil: 'domcontentloaded',
    timeout: 60000,
  });
  await page.waitForFunction(() => typeof HF === 'function' && typeof ES !== 'undefined', null, {
    timeout: 30000,
  });
  const payload = await page.evaluate(() => {
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
    extra.CHD.forEach((sprite, index) => add('chest_' + index, sprite));
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
    add('enemy_62_0',extra.zKF('i',0));
    add('enemy_62_1',extra.zKF('i',1));
    if (typeof window.zSetTiles === 'function') {
      window.zSetTiles(1);
      for (let j=0; j<4; j++) {
        add('dark_grass_' + j,TT[j]);
        add('dark_soil_' + j,TD[j]);
      }
    }
    // Vegetación, rocas, señales, cabañas y capas de bosque extraídas
    // directamente de los Canvas originales. Los mapas usan las posiciones
    // producidas por el generador determinista zDecoBuild(m).
    const names=new Map();
    const register=(prefix, entries)=>{
      for (const [name,canvas] of entries) {
        const key='decor_'+prefix+'_'+name;
        add(key,canvas);names.set(canvas,key);
      }
    };
    register('tree',Object.entries(extra.ZT));
    register('bush',Object.entries(extra.ZB));
    register('grass',extra.ZG.map((canvas,index)=>[String(index),canvas]));
    register('rock',extra.ZRK.map((canvas,index)=>[String(index),canvas]));
    register('mush',extra.ZM.map((canvas,index)=>[String(index),canvas]));
    register('sign',extra.ZSG.map((canvas,index)=>[String(index),canvas]));
    register('grave',extra.ZGV.map((canvas,index)=>[String(index),canvas]));
    register('misc',[['stump',extra.ZSTUMP],['log',extra.ZLOG],['fence',extra.ZFENCE]]);
    register('cabin',extra.ZCB.map((canvas,index)=>[String(index),canvas]));
    for(let map=0;map<2;map++)for(let layer=0;layer<3;layer++){
      add('forest_layer_'+map+'_'+layer,(map===0?extra.FL:extra.FL2)[layer]);
    }
    const decorLayout=[];
    for(let map=0;map<2;map++){
      const layout=extra.zDecoBuild(map);
      for(const item of layout){
        let key=names.get(item.s);
        if(!key){
          key='decor_extra_'+map+'_'+decorLayout.length;
          add(key,item.s);
          names.set(item.s,key);
        }
        const scale=item.cab?1:(item.tr?1.35:1.5);
        const layer=item.cab?0:(item.tr?1:2);
        const ax=Number(item.s?.ax || item.s?.width/2 || 0);
        const ay=Number(item.s?.ay || item.s?.height || 0);
        decorLayout.push([map,Math.round(item.x*10)/10,key,ax,ay,scale,
          Number(Boolean(item.gl)),layer].join(','));
      }
    }
    return {images:out,decorLayout};
  });
  const images=payload.images;
  if (images.length < 30) throw Error('Exportación incompleta: ' + images.length + ' sprites');
  const index = {};
  for (const item of images) {
    const {name, width, height, ax, ay, base64} = item;
    await writeFile(path.join(dest, name + '.png'), Buffer.from(base64, 'base64'));
    index[name] = { width, height, ax, ay };
  }
  await writeFile(path.join(dest, 'index.json'), JSON.stringify(index, null, 2) + '\n');
  await writeFile(path.join(dest,'decor_layout.csv'),payload.decorLayout.join('\n')+'\n');
  console.log('Exportados ' + images.length + ' sprites originales a ' + dest);
} finally {
  await browser.close();
}
