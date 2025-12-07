export function processImageJs(
  width: number,
  height: number,
  data: Uint8ClampedArray
): Uint8ClampedArray {
  const w = width;
  const h = height;

  const img = new Uint8ClampedArray(data);
  const tmp = new Uint8ClampedArray(img.length);

  boxBlurJs(img, tmp, w, h);
  gaussianBlurJs(tmp, img, w, h);
  grayscaleJs(img, w, h);

  const downW = Math.max(1, Math.floor(w / 2));
  const downH = Math.max(1, Math.floor(h / 2));
  const down = new Uint8ClampedArray(downW * downH * 4);
  resizeNearestJs(img, w, h, down, downW, downH);

  const up = new Uint8ClampedArray(w * h * 4);
  resizeNearestJs(down, downW, downH, up, w, h);

  const waved = new Uint8ClampedArray(w * h * 4);
  waveJs(up, waved, w, h);

  const baseAfterWave = new Uint8ClampedArray(waved);

  let current = waved;
  for (let pass = 0; pass < 4; pass++) {
    const edges = new Uint8ClampedArray(w * h * 4);
    sobelEdgesJs(current, edges, w, h);
    nonlinearEdgesJs(edges, w, h);
    const blended = new Uint8ClampedArray(w * h * 4);
    blendEdgesJs(current, edges, blended, w, h);
    current = blended;
  }

  intensityHashPassJs(current, w, h);
  finalBlendWithBaseJs(current, baseAfterWave, w, h);

  return current;
}

function getPixelJs(
  src: Uint8ClampedArray,
  w: number,
  h: number,
  x: number,
  y: number
): [number, number, number, number] {
  const xx = Math.min(Math.max(x, 0), w - 1);
  const yy = Math.min(Math.max(y, 0), h - 1);
  const idx = (yy * w + xx) * 4;
  return [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]];
}

function convolve3x3Js(
  src: Uint8ClampedArray,
  dst: Uint8ClampedArray,
  w: number,
  h: number,
  kernel: number[],
  norm: number
) {
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      let r = 0,
        g = 0,
        b = 0,
        a = 0;
      let k = 0;
      for (let ky = -1; ky <= 1; ky++) {
        for (let kx = -1; kx <= 1; kx++) {
          const [pr, pg, pb, pa] = getPixelJs(src, w, h, x + kx, y + ky);
          const kv = kernel[k++];
          r += kv * pr;
          g += kv * pg;
          b += kv * pb;
          a += kv * pa;
        }
      }
      const idx = (y * w + x) * 4;
      dst[idx] = clamp255(r * norm);
      dst[idx + 1] = clamp255(g * norm);
      dst[idx + 2] = clamp255(b * norm);
      dst[idx + 3] = clamp255(a * norm);
    }
  }
}

function clamp255(v: number): number {
  return (v < 0 ? 0 : v > 255 ? 255 : v) | 0;
}

function boxBlurJs(
  src: Uint8ClampedArray,
  dst: Uint8ClampedArray,
  w: number,
  h: number
) {
  const kernel = [1, 1, 1, 1, 1, 1, 1, 1, 1];
  convolve3x3Js(src, dst, w, h, kernel, 1 / 9);
}

function gaussianBlurJs(
  src: Uint8ClampedArray,
  dst: Uint8ClampedArray,
  w: number,
  h: number
) {
  const kernel = [1, 2, 1, 2, 4, 2, 1, 2, 1];
  convolve3x3Js(src, dst, w, h, kernel, 1 / 16);
}

function grayscaleJs(img: Uint8ClampedArray, w: number, h: number) {
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;
      const r = img[idx];
      const g = img[idx + 1];
      const b = img[idx + 2];
      const gray = clamp255(0.299 * r + 0.587 * g + 0.114 * b);
      img[idx] = gray;
      img[idx + 1] = gray;
      img[idx + 2] = gray;
    }
  }
}

function resizeNearestJs(
  src: Uint8ClampedArray,
  wIn: number,
  hIn: number,
  dst: Uint8ClampedArray,
  wOut: number,
  hOut: number
) {
  for (let y = 0; y < hOut; y++) {
    const srcY = Math.floor((y * hIn) / hOut);
    for (let x = 0; x < wOut; x++) {
      const srcX = Math.floor((x * wIn) / wOut);
      const srcIdx = (srcY * wIn + srcX) * 4;
      const dstIdx = (y * wOut + x) * 4;
      dst[dstIdx] = src[srcIdx];
      dst[dstIdx + 1] = src[srcIdx + 1];
      dst[dstIdx + 2] = src[srcIdx + 2];
      dst[dstIdx + 3] = src[srcIdx + 3];
    }
  }
}

function waveJs(
  src: Uint8ClampedArray,
  dst: Uint8ClampedArray,
  w: number,
  h: number
) {
  const amplitude = 10;
  const frequency = (2 * Math.PI) / 80;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const offsetFloat = amplitude * Math.sin(y * frequency);
      const offset =
        offsetFloat < 0 ? Math.ceil(offsetFloat) : Math.floor(offsetFloat);

      const sx = x + offset;
      const sy = y;
      const [r, g, b, a] = getPixelJs(src, w, h, sx, sy);
      const idx = (y * w + x) * 4;
      dst[idx] = r;
      dst[idx + 1] = g;
      dst[idx + 2] = b;
      dst[idx + 3] = a;
    }
  }
}

function sobelEdgesJs(
  src: Uint8ClampedArray,
  dst: Uint8ClampedArray,
  w: number,
  h: number
) {
  const gx = [-1, 0, 1, -2, 0, 2, -1, 0, 1];
  const gy = [-1, -2, -1, 0, 0, 0, 1, 2, 1];

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      let sx = 0;
      let sy = 0;
      let k = 0;

      for (let ky = -1; ky <= 1; ky++) {
        for (let kx = -1; kx <= 1; kx++) {
          const [r, g, b] = getPixelJs(src, w, h, x + kx, y + ky);
          const lum = (0.299 * r + 0.587 * g + 0.114 * b) | 0;

          sx += gx[k] * lum;
          sy += gy[k] * lum;
          k++;
        }
      }

      const mag = clamp255(Math.sqrt(sx * sx + sy * sy));
      const idx = (y * w + x) * 4;
      dst[idx] = mag;
      dst[idx + 1] = mag;
      dst[idx + 2] = mag;
      dst[idx + 3] = 255;
    }
  }
}

function blendEdgesJs(
  base: Uint8ClampedArray,
  edges: Uint8ClampedArray,
  dst: Uint8ClampedArray,
  w: number,
  h: number
) {
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;

      const br = base[idx];
      const bg = base[idx + 1];
      const bb = base[idx + 2];

      const er = edges[idx];
      const eg = edges[idx + 1];
      const eb = edges[idx + 2];

      const r = clamp255(0.7 * br + 0.8 * er);
      const g = clamp255(0.7 * bg + 0.8 * eg);
      const b = clamp255(0.7 * bb + 0.8 * eb);

      dst[idx] = r;
      dst[idx + 1] = g;
      dst[idx + 2] = b;
      dst[idx + 3] = base[idx + 3];
    }
  }
}

function nonlinearEdgesJs(
  edges: Uint8ClampedArray,
  w: number,
  h: number
) {
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;
      let v = edges[idx] / 255;

      let acc = v;
      for (let i = 0; i < 8; i++) {
        const s = Math.abs(Math.sin(acc * 12.0));
        acc = Math.sqrt(acc * 0.85 + s);
        if (!Number.isFinite(acc)) {
          acc = 0;
          break;
        }
        if (acc > 1.0) acc = 1.0;
      }

      const out = clamp255(acc * 255);
      edges[idx] = out;
      edges[idx + 1] = out;
      edges[idx + 2] = out;
    }
  }
}

function pixelHashMixJs(seed: number): number {
  const MOD = 1000000007n;
  const ROUNDS = 64n;

  let acc = (BigInt(seed) + 1n) % MOD;
  const s = BigInt(seed) % MOD;

  for (let i = 1n; i <= ROUNDS; i++) {
    const factor = (i + s) % MOD;
    acc = (acc * factor + i * 17n) % MOD;
  }

  return Number(acc & 0xffn);
}

function intensityHashPassJs(
  img: Uint8ClampedArray,
  w: number,
  h: number
) {
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;

      const r = img[idx];
      const g = img[idx + 1];
      const b = img[idx + 2];

      const seed = (r + (g << 8) + (b << 16)) % 1000;
      const m = pixelHashMixJs(seed);

      const scale = 128 + m;

      const nr = clamp255((r * scale) / 255);
      const ng = clamp255((g * scale) / 255);
      const nb = clamp255((b * scale) / 255);

      img[idx] = nr;
      img[idx + 1] = ng;
      img[idx + 2] = nb;
    }
  }
}

function finalBlendWithBaseJs(
  img: Uint8ClampedArray,
  base: Uint8ClampedArray,
  w: number,
  h: number
) {
  const alpha = 0.6;
  const beta = 1.0 - alpha;

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;

      const br = base[idx];
      const bg = base[idx + 1];
      const bb = base[idx + 2];

      const pr = img[idx];
      const pg = img[idx + 1];
      const pb = img[idx + 2];

      img[idx] = clamp255(alpha * br + beta * pr);
      img[idx + 1] = clamp255(alpha * bg + beta * pg);
      img[idx + 2] = clamp255(alpha * bb + beta * pb);
    }
  }
}
