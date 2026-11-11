/**
 * Baseline visual del HTML original (no altera el archivo del repositorio).
 * npm install --no-save playwright-core && node scripts/capture-reference.mjs
 */
import { chromium } from 'playwright-core';
import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const out=path.join(root,'reference-captures');
await mkdir(out,{recursive:true});
const browser=await chromium.launch({
  channel:'chrome',headless:true,
  args:['--no-sandbox','--disable-dev-shm-usage','--autoplay-policy=no-user-gesture-required']
});
try {
  const page=await browser.newPage({viewport:{width:960,height:540},deviceScaleFactor:1});
  const errors=[];
  page.on('pageerror',err=>errors.push(err.message));
  const html=await readFile(path.join(root,'Retro Realm.html'),'utf8');
  const marker='function zRain(){';
  if(!html.includes(marker))throw Error('El HTML cambió: revisar la instrumentación');
  const inject='window.__RETRO_REF={setMap:function(m){window.MAP=m;p.x=70;p.y=gAt(70);cam=0;en.length=0;}};';
  const instrumented=html.replace(marker,inject+marker);
  const base=new URL('file://'+root+'/').href;
  await page.setContent(instrumented.replace('<head>','<head><base href="'+base+'">'),
    {waitUntil:'domcontentloaded',timeout:60000});
  await page.waitForFunction(
    ()=>document.querySelector('#pl') && typeof window.__RETRO_REF?.setMap==='function',
    null,{timeout:20000}
  );
  await page.waitForTimeout(700);
  await page.screenshot({path:path.join(out,'01-menu-original.png')});
  await page.locator('#pl').click({force:true});
  await page.waitForTimeout(1800);
  await page.screenshot({path:path.join(out,'02-bosque-nocturno-original.png')});
  await page.evaluate(()=>window.__RETRO_REF.setMap(1));
  await page.waitForTimeout(850);
  await page.screenshot({path:path.join(out,'03-bosque-oscuro-original.png')});
  await writeFile(path.join(out,'README.txt'),[
    'HTML original de Retro Realm; Chrome; 960x540; DPR 1.',
    '01: menu original',
    '02: Bosque Nocturno en inicio de partida',
    '03: Bosque Oscuro (solo instrumentacion temporal para captura)',
    'El HTML real del repositorio no se ha editado.',
    'Animaciones, particulas y aleatoriedad pueden variar entre capturas.',
    'Errores de JS: '+errors.length,
    ...errors.slice(0,8)
  ].join('\n')+'\n');
  console.log('Referencias guardadas en '+out);
} finally {
  await browser.close();
}
