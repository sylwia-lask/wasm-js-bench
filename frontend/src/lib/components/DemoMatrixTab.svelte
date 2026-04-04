<script lang="ts">
  import { onMount } from 'svelte';
  import init, { matmul_sum as matmulSumWasm } from '../wasm/wasm_js_bench.js';
  import { jsMatmulSum } from '../logic/matmulSum.js';

  let wasmReady = false;
  let isRunning = false;

  let n: number = 180;
  let runs: number = 3;

  let jsTime: string | null = null;
  let wasmTime: string | null = null;
  let winner: 'js' | 'wasm' | 'tie' | null = null;
  let winnerMultiplier: string | null = null;

  let jsResult: number | null = null;
  let wasmResult: number | null = null;

  onMount(async () => {
    await init();
    wasmReady = true;
  });

  type BenchResult = { ms: number; result: number };

  function bench(fn: (arg: number) => number, arg: number, runs: number): BenchResult {
    const t0 = performance.now();
    let result = 0;
    for (let i = 0; i < runs; i++) {
      result = fn(arg);
    }
    const t1 = performance.now();
    return { ms: (t1 - t0) / runs, result };
  }

  async function runBenchmark() {
    if (isRunning) return;
    isRunning = true;

    jsTime = null;
    wasmTime = null;
    winner = null;
    winnerMultiplier = null;
    jsResult = null;
    wasmResult = null;

    const numericN = Number(n);
    const numericRuns = Number(runs);

    await Promise.resolve();

    const js = bench(jsMatmulSum, numericN, numericRuns);
    jsTime = js.ms.toFixed(2);
    jsResult = js.result;

    if (wasmReady) {
      const wasm = bench(matmulSumWasm as (n: number) => number, numericN, numericRuns);
      wasmTime = wasm.ms.toFixed(2);
      wasmResult = wasm.result;

      const ratio = js.ms / wasm.ms;
      if (ratio < 0.95) {
        winner = 'js';
        winnerMultiplier = (1 / ratio).toFixed(1);
      } else if (ratio > 1.05) {
        winner = 'wasm';
        winnerMultiplier = ratio.toFixed(1);
      } else {
        winner = 'tie';
        winnerMultiplier = ratio.toFixed(2);
      }
    }

    isRunning = false;
  }
</script>

<div class="space-y-4">
  <p class="text-sm text-slate-300">
    An <code class="px-1 rounded bg-slate-800 text-indigo-300">n × n</code> matrix multiply returning a single number (mod 1&nbsp;000&nbsp;000&nbsp;007).
    Both sides run the same O(n³) algorithm — this is where the JavaScript JIT often keeps pace with WASM.
  </p>

  <div class="grid grid-cols-2 gap-4">
    <div>
      <label for="matrix-n" class="block text-xs font-semibold mb-1.5 text-slate-300 uppercase tracking-wide">n (matrix size, n × n)</label>
      <input
        id="matrix-n"
        class="w-full rounded-xl bg-slate-800 border border-slate-700 px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        type="number"
        bind:value={n}
        min="50"
        max="400"
        step="10"
        disabled={isRunning}
      />
      <p class="mt-1 text-xs text-slate-500">Complexity O(n³). Bigger n = heavier work.</p>
    </div>

    <div>
      <label for="matrix-runs" class="block text-xs font-semibold mb-1.5 text-slate-300 uppercase tracking-wide">Runs (averaging)</label>
      <input
        id="matrix-runs"
        class="w-full rounded-xl bg-slate-800 border border-slate-700 px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        type="number"
        bind:value={runs}
        min="1"
        max="10"
        disabled={isRunning}
      />
    </div>
  </div>

  <button
    class="w-full py-3.5 rounded-2xl bg-indigo-600 hover:bg-indigo-500 text-lg font-bold transition disabled:opacity-50 shadow-lg"
    on:click={runBenchmark}
    disabled={!wasmReady || isRunning}
  >
    {#if !wasmReady}Loading WASM…{:else if isRunning}Running benchmark…{:else}Run matrix benchmark{/if}
  </button>

  <div class="grid grid-cols-2 gap-4">
    <div class={`p-5 rounded-2xl border-2 relative transition-all ${
      winner === 'js' ? 'bg-emerald-950 border-emerald-400 shadow-lg shadow-emerald-900/30'
      : winner === 'tie' ? 'bg-amber-950/40 border-amber-500'
      : 'bg-slate-800 border-slate-700'
    }`}>
      {#if winner === 'js'}
        <div class="absolute -top-3.5 left-1/2 -translate-x-1/2 bg-emerald-400 text-slate-900 text-xs font-black px-3 py-0.5 rounded-full uppercase tracking-widest">WINNER</div>
      {/if}
      <div class="text-xs uppercase tracking-widest text-slate-400 mb-2 font-semibold">JavaScript</div>
      <div class="text-5xl font-black font-mono">
        {#if jsTime !== null}{jsTime}<span class="text-xl font-normal text-slate-400 ml-1">ms</span>{:else}–{/if}
      </div>
      {#if jsResult !== null}
        <div class="mt-2 text-xs text-slate-500 font-mono">result: {jsResult}</div>
      {/if}
    </div>

    <div class={`p-5 rounded-2xl border-2 relative transition-all ${
      winner === 'wasm' ? 'bg-emerald-950 border-emerald-400 shadow-lg shadow-emerald-900/30'
      : winner === 'tie' ? 'bg-amber-950/40 border-amber-500'
      : 'bg-slate-800 border-slate-700'
    }`}>
      {#if winner === 'wasm'}
        <div class="absolute -top-3.5 left-1/2 -translate-x-1/2 bg-emerald-400 text-slate-900 text-xs font-black px-3 py-0.5 rounded-full uppercase tracking-widest">WINNER</div>
      {/if}
      <div class="text-xs uppercase tracking-widest text-slate-400 mb-2 font-semibold">Rust (WASM)</div>
      <div class="text-5xl font-black font-mono">
        {#if wasmTime !== null}{wasmTime}<span class="text-xl font-normal text-slate-400 ml-1">ms</span>{:else}–{/if}
      </div>
      {#if wasmResult !== null}
        <div class="mt-2 text-xs text-slate-500 font-mono">result: {wasmResult}</div>
      {/if}
    </div>
  </div>

  {#if winner !== null}
    <div class={`py-4 px-6 rounded-2xl text-center border-2 flex items-center justify-center gap-6 ${
      winner === 'tie' ? 'bg-amber-950/30 border-amber-500'
      : 'bg-emerald-950/50 border-emerald-500'
    }`}>
      {#if winner === 'tie'}
        <div class="text-3xl font-black text-amber-300">Essentially tied!</div>
        <div class="text-slate-400 text-base">ratio: {winnerMultiplier}× — within noise</div>
      {:else}
        <div class={`text-6xl font-black font-mono ${winner === 'js' ? 'text-blue-300' : 'text-emerald-300'}`}>{winnerMultiplier}×</div>
        <div class="text-xl font-bold text-slate-200">{winner === 'js' ? 'JavaScript' : 'Rust (WASM)'} is faster</div>
      {/if}
    </div>
  {/if}
</div>
