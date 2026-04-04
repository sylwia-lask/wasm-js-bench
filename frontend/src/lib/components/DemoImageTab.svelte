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
  let winner: 'js' | 'wasm' | null = null;
  let winnerMultiplier: string | null = null;

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
    winner = null;
    winnerMultiplier = null;
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
    winner = null;
    winnerMultiplier = null;

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
        if (jsMs > wasmMs) {
          winner = 'wasm';
          winnerMultiplier = (jsMs / wasmMs).toFixed(1);
        } else {
          winner = 'js';
          winnerMultiplier = (wasmMs / jsMs).toFixed(1);
        }
      }
    } finally {
      running = false;
    }
  }
</script>

<div class="space-y-4">
  <p class="text-sm text-slate-300">
    A <strong>heavy</strong> pixel pipeline — convolution, Gaussian blur, grayscale, wave distortion,
    Sobel edge detection, nonlinear enhancement, and final blending — run in
    <strong>TypeScript</strong> and <strong>Rust → WebAssembly</strong>.
  </p>

  <div class="flex items-center gap-4">
    <div class="flex items-center gap-2">
      <span class="text-xs text-slate-400">Source:</span>
      {#if !imageLoaded}
        <span class="text-xs text-slate-500">none</span>
      {:else if currentSource === 'sample'}
        <span class="text-xs text-emerald-400 font-semibold">sample image</span>
      {:else if currentSource === 'uploaded'}
        <span class="text-xs text-amber-400 font-semibold">uploaded image</span>
      {/if}
      {#if imageLoaded}
        <span class="text-xs text-slate-500">({originalWidth} × {originalHeight})</span>
      {/if}
    </div>

    <button
      class="px-4 py-1.5 rounded-lg bg-slate-800 border border-slate-700 hover:bg-slate-700 text-xs font-medium transition disabled:opacity-50"
      on:click={loadSampleImage}
      disabled={!wasmReady || running}
    >
      Load sample
    </button>

    <label for="img-upload" class="px-4 py-1.5 rounded-lg bg-slate-800 border border-slate-700 hover:bg-slate-700 text-xs font-medium transition cursor-pointer">
      Upload image
    </label>
    <input
      id="img-upload"
      class="hidden"
      type="file"
      accept="image/*"
      on:change={handleFileChange}
      disabled={running}
    />
  </div>

  <div class="grid grid-cols-3 gap-3">
    <div>
      <div class="text-xs uppercase tracking-widest text-slate-400 mb-1.5 font-semibold">Input</div>
      <canvas bind:this={inputCanvas} class="w-full border border-slate-700 rounded-xl"></canvas>
    </div>
    <div class={`rounded-xl p-1 transition-all ${winner === 'js' ? 'ring-2 ring-emerald-400 bg-emerald-950/20' : ''}`}>
      <div class="flex items-center gap-2 mb-1.5 px-1">
        <div class="text-xs uppercase tracking-widest text-slate-400 font-semibold">JS output</div>
        {#if winner === 'js'}
          <span class="bg-emerald-400 text-slate-900 text-xs font-black px-2 py-0.5 rounded-full uppercase tracking-wider">WINNER</span>
        {/if}
      </div>
      <canvas bind:this={outputJsCanvas} class="w-full border border-slate-700 rounded-lg"></canvas>
    </div>
    <div class={`rounded-xl p-1 transition-all ${winner === 'wasm' ? 'ring-2 ring-emerald-400 bg-emerald-950/20' : ''}`}>
      <div class="flex items-center gap-2 mb-1.5 px-1">
        <div class="text-xs uppercase tracking-widest text-slate-400 font-semibold">WASM output</div>
        {#if winner === 'wasm'}
          <span class="bg-emerald-400 text-slate-900 text-xs font-black px-2 py-0.5 rounded-full uppercase tracking-wider">WINNER</span>
        {/if}
      </div>
      <canvas bind:this={outputWasmCanvas} class="w-full border border-slate-700 rounded-lg"></canvas>
    </div>
  </div>

  <button
    class="w-full py-3.5 rounded-2xl bg-indigo-600 hover:bg-indigo-500 text-lg font-bold transition disabled:opacity-50 shadow-lg"
    on:click={runBoth}
    disabled={!wasmReady || !imageLoaded || running}
  >
    {#if !wasmReady}Loading WASM…{:else if running}Running pipelines…{:else}Run JS & WASM pipelines{/if}
  </button>

  <div class="grid grid-cols-3 gap-4">
    <div class={`p-4 rounded-2xl border-2 transition-all ${winner === 'js' ? 'bg-emerald-950 border-emerald-400' : 'bg-slate-800 border-slate-700'}`}>
      <div class="text-xs uppercase tracking-widest text-slate-400 mb-2 font-semibold">JS runtime</div>
      <div class="text-4xl font-black font-mono">
        {#if jsTime !== null}{jsTime}<span class="text-base font-normal text-slate-400 ml-1">ms</span>{:else}–{/if}
      </div>
    </div>

    <div class={`p-4 rounded-2xl border-2 transition-all ${winner === 'wasm' ? 'bg-emerald-950 border-emerald-400' : 'bg-slate-800 border-slate-700'}`}>
      <div class="text-xs uppercase tracking-widest text-slate-400 mb-2 font-semibold">WASM runtime</div>
      <div class="text-4xl font-black font-mono">
        {#if wasmTime !== null}{wasmTime}<span class="text-base font-normal text-slate-400 ml-1">ms</span>{:else}–{/if}
      </div>
    </div>

    <div class={`p-4 rounded-2xl border-2 flex items-center justify-center gap-4 transition-all ${winner !== null ? 'bg-emerald-950/50 border-emerald-500' : 'bg-slate-800 border-slate-700'}`}>
      {#if winner !== null}
        <div class="text-4xl font-black font-mono text-emerald-300">{winnerMultiplier}×</div>
        <div class="text-sm font-bold text-slate-200">{winner === 'wasm' ? 'Rust (WASM)' : 'JavaScript'} is faster</div>
      {:else}
        <div class="text-xs uppercase tracking-widest text-slate-400 font-semibold">Speed ratio</div>
      {/if}
    </div>
  </div>
</div>
