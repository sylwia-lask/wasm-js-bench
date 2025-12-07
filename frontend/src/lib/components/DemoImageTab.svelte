<script lang="ts">
  import { onMount } from 'svelte';
  import initWasm, { process_image_wasm as processImageWasm } from '../wasm/wasm_js_bench.js';
  import sampleImageUrl from '../../assets/sample-image.jpg';
  import { processImageJs } from '../logic/imagePipeline';

  let wasmReady = false;
  let imageLoaded = false;

  let inputCanvas: HTMLCanvasElement;
  let outputJsCanvas: HTMLCanvasElement;
  let outputWasmCanvas: HTMLCanvasElement;

  let originalWidth = 0;
  let originalHeight = 0;

  let jsTime: string | null = null;
  let wasmTime: string | null = null;
  let speedup: string | null = null;

  let currentSource: 'sample' | 'uploaded' | null = null;
  let running = false;

  onMount(async () => {
    await initWasm();
    wasmReady = true;
    loadSampleImage();
  });

  function loadSampleImage() {
    if (running) return;
    const img = new Image();
    img.onload = () => {
      drawImageToInputCanvas(img);
      currentSource = 'sample';
    };
    img.src = sampleImageUrl;
  }

  function handleFileChange(event: Event) {
    if (running) return;
    const target = event.target as HTMLInputElement;
    if (!target.files || target.files.length === 0) return;
    const file = target.files[0];
    const img = new Image();
    img.onload = () => {
      drawImageToInputCanvas(img);
      currentSource = 'uploaded';
    };
    img.src = URL.createObjectURL(file);
  }

  function drawImageToInputCanvas(img: HTMLImageElement) {
    const ctx = inputCanvas.getContext('2d');
    if (!ctx) return;
    originalWidth = img.width;
    originalHeight = img.height;
    inputCanvas.width = originalWidth;
    inputCanvas.height = originalHeight;
    ctx.drawImage(img, 0, 0);
    imageLoaded = true;

    clearOutputCanvases();
    jsTime = null;
    wasmTime = null;
    speedup = null;
  }

  function clearOutputCanvases() {
    if (outputJsCanvas) {
      const jsCtx = outputJsCanvas.getContext('2d');
      if (jsCtx) {
        outputJsCanvas.width = originalWidth;
        outputJsCanvas.height = originalHeight;
        jsCtx.clearRect(0, 0, outputJsCanvas.width, outputJsCanvas.height);
      }
    }
    if (outputWasmCanvas) {
      const wasmCtx = outputWasmCanvas.getContext('2d');
      if (wasmCtx) {
        outputWasmCanvas.width = originalWidth;
        outputWasmCanvas.height = originalHeight;
        wasmCtx.clearRect(0, 0, outputWasmCanvas.width, outputWasmCanvas.height);
      }
    }
  }

  function getImageData(): ImageData | null {
    const ctx = inputCanvas.getContext('2d');
    if (!ctx) return null;
    return ctx.getImageData(0, 0, originalWidth, originalHeight);
  }

  function drawToCanvas(canvas: HTMLCanvasElement, pixels: Uint8ClampedArray) {
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    const outImg = new ImageData(pixels as any, originalWidth, originalHeight);
    canvas.width = originalWidth;
    canvas.height = originalHeight;
    ctx.putImageData(outImg, 0, 0);
  }

  async function runBoth() {
    if (!wasmReady || !imageLoaded || running) return;
    const imgData = getImageData();
    if (!imgData) return;

    running = true;
    jsTime = null;
    wasmTime = null;
    speedup = null;

    const inputPixels = imgData.data;

    try {
      const t0Wasm = performance.now();
      const wasmOut = processImageWasm(originalWidth, originalHeight, inputPixels);
      const t1Wasm = performance.now();
      const wasmMs = t1Wasm - t0Wasm;
      wasmTime = wasmMs.toFixed(2);
      drawToCanvas(outputWasmCanvas, wasmOut);

      const t0Js = performance.now();
      const jsOut = processImageJs(originalWidth, originalHeight, inputPixels);
      const t1Js = performance.now();
      const jsMs = t1Js - t0Js;
      jsTime = jsMs.toFixed(2);
      drawToCanvas(outputJsCanvas, jsOut);

      if (jsMs > 0 && wasmMs > 0) {
        const ratio = jsMs / wasmMs;
        if (ratio > 1) {
          speedup = `${ratio.toFixed(1)}× slower than WASM`;
        } else {
          speedup = `${(1 / ratio).toFixed(1)}× faster than JS`;
        }
      }
    } finally {
      running = false;
    }
  }
</script>

<div class="space-y-4">
  <h2 class="text-lg font-semibold">
    Tab X – when WASM crushes heavy pixel work
  </h2>

  <p class="text-sm text-slate-300">
    We run a <strong>very heavy</strong> image-processing pipeline (multiple convolution passes,
    nonlinear pixel transformations, wave distortions, edge detection and glow passes)
    in <strong>TypeScript</strong> and <strong>Rust → WebAssembly</strong>.
  </p>

  <div class="text-xs text-slate-400">
    Current source:
    {#if !imageLoaded}
      <span class="text-slate-500">none</span>
    {:else if currentSource === 'sample'}
      <span class="text-emerald-400 font-semibold">sample image</span>
    {:else if currentSource === 'uploaded'}
      <span class="text-amber-400 font-semibold">uploaded image</span>
    {/if}
    {#if imageLoaded}
      <span class="ml-2 text-slate-500">({originalWidth} × {originalHeight})</span>
    {/if}
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <div>
      <label class="block text-xs font-medium mb-1">Use sample image</label>
      <button
        class="w-full py-2 rounded-md bg-slate-800 border border-slate-700 hover:bg-slate-700 text-xs font-medium transition disabled:opacity-50"
        on:click={loadSampleImage}
        disabled={!wasmReady || running}
      >
        Load built-in sample
      </button>
    </div>

    <div>
      <label class="block text-xs font-medium mb-1">Or upload your own image</label>
      <input
        class="w-full text-xs file:text-xs file:px-3 file:py-1.5 file:rounded-md file:border-0
               file:bg-slate-800 file:text-slate-100 file:cursor-pointer
               bg-slate-900 border border-slate-700 rounded-md px-2 py-1"
        type="file"
        accept="image/*"
        on:change={handleFileChange}
        disabled={running}
      />
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-2">
    <div class="panel">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">Input</div>
      <canvas bind:this={inputCanvas} class="w-full border border-slate-800 rounded-md"></canvas>
    </div>
    <div class="panel">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">JS output</div>
      <canvas bind:this={outputJsCanvas} class="w-full border border-slate-800 rounded-md"></canvas>
    </div>
    <div class="panel">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">WASM output</div>
      <canvas bind:this={outputWasmCanvas} class="w-full border border-slate-800 rounded-md"></canvas>
    </div>
  </div>

  <button
    class="w-full py-2.5 rounded-md bg-indigo-500 hover:bg-indigo-400 text-sm font-semibold transition disabled:opacity-50"
    on:click={runBoth}
    disabled={!wasmReady || !imageLoaded || running}
  >
    {#if !wasmReady}
      Loading WASM…
    {:else if running}
      Running pipelines…
    {:else}
      Run JS & WASM pipelines
    {/if}
  </button>

  <div class="mt-4 grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
    <div class="p-3 rounded-lg bg-slate-800 border border-slate-700">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">JS runtime</div>
      <div class="text-lg font-mono">{jsTime ?? '–'} ms</div>
    </div>

    <div class="p-3 rounded-lg bg-slate-800 border border-slate-700">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">WASM runtime</div>
      <div class="text-lg font-mono">{wasmTime ?? '–'} ms</div>
    </div>

    <div class="p-3 rounded-lg bg-slate-800 border border-slate-700">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">Speed ratio</div>
      <div class="text-lg font-mono text-emerald-400">
        {speedup ?? '–'}
      </div>
    </div>
  </div>
</div>
