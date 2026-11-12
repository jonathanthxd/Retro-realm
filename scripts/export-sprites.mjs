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
  const sceneAnchor = 'bgx=function(g,cx,tt){';
  if (!source.includes(sceneAnchor)) throw Error('No se encontraron las capas originales de bgx');
  const instrumented = source.replace(marker,
    'window.__RETRO_EXPORT={SLR,OG,zHE2,zKF,CHD,ZT,ZB,ZG,ZRK,ZM,ZSG,ZGV,ZSTUMP,ZLOG,ZFENCE,FL,FL2,zDecoBuild,ZCB};' + marker)
    .replace(sceneAnchor,'window.__RETRO_BG={ZLAY,ZMN,ZST,ZCD};' + sceneAnchor);
  const base = new URL('file://' + root + '/').href;
  await page.setContent(instrumented.replace('<head>', '<head><base href="' + base + '">'), {
    waitUntil: 'domcontentloaded',
    timeout: 60000,
  });
  await page.waitForFunction(() => typeof HF === 'function' && typeof ES !== 'undefined', null, {
    timeout: 30000,
  });
  const payload = await page.evaluate(async () => {
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
      // HF(cl, fase, idleFrame, arm, slash) — conservar poses reales.
      for (let phase of [0,1,2,3,4,5,6,7,8,9,11]) {
        add('hero_' + cl + '_' + phase, HF(cl, phase, 0, 0, 0));
      }
      for (let idle = 0; idle < 4; idle++) {
        add('hero_idle_' + cl + '_' + idle, HF(cl, 0, idle, 0, 0));
      }
      add('hero_cast_' + cl, HF(cl, 0, 0, 1, 0));
      for (let slash = 0; slash < 3; slash++) {
        add('hero_attack_' + cl + '_' + slash, HF(cl, 9, 0, 2, slash));
      }
    }
    // Primeras criaturas. Las variantes avanzadas se exportarán por separado.
    for (let k = 0; k < ES.length; k++) {
      if (!Array.isArray(ES[k])) continue;
      for (let f = 0; f < ES[k].length; f++) add('enemy_' + k + '_' + f, ES[k][f]);
    }
    // Al entrar al bosque el HTML sustituye TT/TD por zTile(…, ZPAL[MAP]).
    // Los assets anteriores se exportaban antes del primer zSetTiles(0).
    if(typeof window.zSetTiles === 'function') window.zSetTiles(0);
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
    // Los escenarios finales originales contienen mucho más que FL / FL2:
    // montaña frontal y lejana, cielo dithered, bosque procedural y nubes.
    // Estos Canvas proceden de zSky/zRidge/zForest/zCloud, no son arte nuevo.
    const sc=window.__RETRO_BG;
    if(!sc || sc.ZLAY.length<2) throw Error('No se pudo acceder a zSky/ZLAY');
    add('scene_moon',sc.ZMN);
    for(let scene=0;scene<2;scene++){
      const art=sc.ZLAY[scene];
      add('scene_sky_'+scene,art.sk);
      for(const key of ['mf','mn','ff','fm'])add('scene_'+key+'_'+scene,art[key]);
      for(let c=0;c<art.cl.length;c++)add('scene_cloud_'+scene+'_'+c,art.cl[c]);
    }
    // Retratos del HUD idénticos al original HTML (HTML ~1110).
    for(let cl=0;cl<3;cl++){
      const portrait=document.createElement('canvas');portrait.width=portrait.height=64;
      const g=portrait.getContext('2d');g.imageSmoothingEnabled=false;
      g.fillStyle='#244c5e';g.fillRect(0,0,64,64);
      const hero=HF(cl,0,0,0,0);
      g.drawImage(hero,8,0,24,26,2,4,60,62);
      add('ui_portrait_'+cl,portrait);
    }
    // Atlas PNG del tipo RetroPx incrustado en el HTML, sin redistribuir WOFF/TTF.
    // Reutiliza el rasterizado Canvas del navegador; 16 columnas x 8 filas.
    await document.fonts.load('16px RetroPx');
    const glyphs=Array.from({length:95},(_,i)=>String.fromCharCode(i+32))
      .concat(Array.from('ÁÉÍÓÚÜÑáéíóúüñ¡¿'));
    const cellW=18,cellH=20,cols=16;
    const fontCanvas=document.createElement('canvas');
    fontCanvas.width=cols*cellW;
    fontCanvas.height=Math.ceil(glyphs.length/cols)*cellH;
    const cg=fontCanvas.getContext('2d');
    cg.fillStyle='#ffffff';
    cg.font='16px RetroPx';
    cg.textAlign='left';
    cg.textBaseline='alphabetic';
    glyphs.forEach((ch,i)=>{
      cg.fillText(ch,(i%cols)*cellW+1,Math.floor(i/cols)*cellH+16);
    });
    add('ui_font_atlas',fontCanvas);
    // Pictogramas definitivos de los botones, tras aplicar CSS y clases del HTML.
    for(let cl=0;cl<3;cl++){
      p.cls=cl;icons();
      for(const id of ['s0','s1','s2','s3','bj','bm','bl','br']){
        const el=document.getElementById(id);
        if(!el)continue;
        const bg=getComputedStyle(el).backgroundImage;
        const src=(bg.match(/data:image\/png;base64,[A-Za-z0-9+/=]+/)||[])[0];
        if(!src)continue;
        const im=new Image();im.src=src;
        try{await im.decode()}catch{continue}
        const cv=document.createElement('canvas');cv.width=im.naturalWidth;cv.height=im.naturalHeight;
        cv.getContext('2d').drawImage(im,0,0);
        add('ui_'+id+'_'+cl,cv);
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
    const cloudLayout=sc.ZCD.map(q=>[q.x,q.y,q.sp,q.s,q.par,q.a].join(','));
    return {images:out,decorLayout,cloudLayout};
  });
  // Capture the FINAL original DOM interface at full 960x540 CSS resolution.
  // Preserve real gradients, metallic borders, illustrations and shadows.
  await page.addStyleTag({content:[
    'html,body,#app{background:transparent!important}',
    '#cv,#home,#menu,.fb,#ap{visibility:hidden!important}',
    '#ctl>*{visibility:hidden!important}',
    '#hd2{visibility:hidden!important;display:flex!important}',
    '#hd2 #h2n,#hd2 #h2l,#hd2 #h2mt,#hd2 #h2ht,',
    '#hd2 #h2g,#hd2 #h2d,#hd2 .fl{visibility:hidden!important}',
  ].join(' ')});
  const uiLayout=[];
  const hudLayout=[];
  const rects=await page.evaluate(()=>{
    const root=document.getElementById('hd2');
    if(!root)throw Error('Final HUD #hd2 not found');
    const targets={frame:'#hd2',name:'#h2n',level:'#h2l',
      hp:'#h2h',mp:'#h2m',hp_text:'#h2ht',mp_text:'#h2mt',
      gold:'#h2g',gems:'#h2d'};
    return Object.entries(targets).map(([key,selector])=>{
      const r=document.querySelector(selector)?.getBoundingClientRect();
      return [key,r?.x??0,r?.y??0,r?.width??0,r?.height??0];
    });
  });
  hudLayout.push(...rects.map(r=>r.join(',')));
  for(let cl=0;cl<3;cl++){
    await page.evaluate(async index=>{
      p.cls=index;
      if(typeof icons==='function')icons();
      if(typeof apIcon==='function')apIcon();
      if(typeof window.hud==='function')window.hud();
      const img=document.getElementById('h2p');
      if(img?.decode)await img.decode().catch(()=>{});
    },cl);
    const hud=page.locator('#hd2');
    await hud.evaluate(el=>el.style.visibility='visible');
    await hud.screenshot({
      path:path.join(dest,'ui_hd2_'+cl+'.png'),
      omitBackground:true,animations:'disabled'
    });
    await hud.evaluate(el=>el.style.visibility='hidden');
    for(const id of ['bl','br','s0','s1','s2','s3','bs','bj','bm','ap']){
      const el=page.locator('#'+id);
      const rect=await el.boundingBox();
      if(!rect||rect.width<10||rect.height<10)
        throw Error('Original DOM button lacks geometry: '+id);
      const pad=4;
      const clip={x:Math.max(0,Math.floor(rect.x-pad)),
        y:Math.max(0,Math.floor(rect.y-pad)),
        width:Math.ceil(rect.width+pad*2),
        height:Math.ceil(rect.height+pad*2)};
      clip.width=Math.min(clip.width,960-clip.x);
      clip.height=Math.min(clip.height,540-clip.y);
      await el.evaluate(node=>node.style.visibility='visible');
      await page.screenshot({
        path:path.join(dest,'ui_dom_'+id+'_'+cl+'.png'),
        clip,omitBackground:true,animations:'disabled'
      });
      await el.evaluate(node=>node.style.visibility='hidden');
      if(cl===0)uiLayout.push([id,clip.x,clip.y,clip.width,clip.height].join(','));
    }
  }
  const pause=page.locator('button.fb.gear').first();
  if(await pause.count()){
    await pause.evaluate(el=>{el.style.display='block';el.style.visibility='visible'});
    const rect=await pause.boundingBox();
    if(rect&&rect.width>0){
      const pad=4;
      const clip={x:Math.max(0,Math.floor(rect.x-pad)),
        y:Math.max(0,Math.floor(rect.y-pad)),
        width:Math.min(960-Math.max(0,Math.floor(rect.x-pad)),Math.ceil(rect.width+pad*2)),
        height:Math.min(540-Math.max(0,Math.floor(rect.y-pad)),Math.ceil(rect.height+pad*2))};
      await page.screenshot({path:path.join(dest,'ui_dom_pause.png'),clip,omitBackground:true});
      uiLayout.push(['pause',clip.x,clip.y,clip.width,clip.height].join(','));
    }
    await pause.evaluate(el=>el.style.visibility='hidden');
  }
  await writeFile(path.join(dest,'ui_dom_layout.csv'),uiLayout.join('\n')+'\n');
  await writeFile(path.join(dest,'ui_hud_layout.csv'),hudLayout.join('\n')+'\n');
  const images=payload.images;
  if (images.length < 30) throw Error('Exportación incompleta: ' + images.length + ' sprites');
  const index = {};
  for (const item of images) {
    const {name, width, height, ax, ay, base64} = item;
    await writeFile(path.join(dest, name + '.png'), Buffer.from(base64, 'base64'));
    index[name] = { width, height, ax, ay };
  }
  await writeFile(path.join(dest, 'index.json'), JSON.stringify(index, null, 2) + '\n');
  // El ancla Canvas (ax, ay) debe mantenerse. El alto del PNG no equivale
  // al punto de apoyo de todos los jefes ni personajes.
  await writeFile(path.join(dest,'sprite_meta.csv'),images.map(({name,ax,ay})=>
    [name,ax,ay].join(',')).join('\n')+'\n');
  await writeFile(path.join(dest,'decor_layout.csv'),payload.decorLayout.join('\n')+'\n');
  await writeFile(path.join(dest,'scene_cloud_layout.csv'),payload.cloudLayout.join('\n')+'\n');
  console.log('Exportados ' + images.length + ' sprites originales a ' + dest);
} finally {
  await browser.close();
}
