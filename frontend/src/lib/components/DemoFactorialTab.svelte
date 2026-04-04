<script lang="ts">
  import { onMount } from 'svelte';
  import init, { factorial_mod as factorialModWasm } from '../wasm/wasm_js_bench.js';
  import { jsFactorialMod } from '../logic/factorialMod.js';

  export const componentName = 'FactorialBenchmarkTab';

  let wasmReady = false;

  let n: number = 500_000;
  let runs: number = 3;

  let jsTime: string | null = null;
  let wasmTime: string | null = null;
  let winner: 'js' | 'wasm' | 'tie' | null = null;
  let winnerMultiplier: string | null = null;

  let jsResult: number | null = null;
  let wasmResult: number | null = null;

  let isRunning = false;

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
    if (!wasmReady || isRunning) return;

    isRunning = true;
    jsTime = null;
    wasmTime = null;
    winner = null;
    winnerMultiplier = null;
    jsResult = null;
    wasmResult = null;

    try {
      const numericN = Number(n);
      const numericRuns = Number(runs);

      const js = bench(jsFactorialMod, numericN, numericRuns);
      jsTime = js.ms.toFixed(2);
      jsResult = js.result;

      const wasm = bench(factorialModWasm as (arg: number) => number, numericN, numericRuns);
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
    } finally {
      isRunning = false;
    }
  }
</script>

<div class="space-y-4">
  <p class="text-sm text-slate-300">
    We compute <code class="px-1 rounded bg-slate-800 text-indigo-300">n! mod 1&nbsp;000&nbsp;000&nbsp;007</code>.
    JavaScript uses <code class="px-1 rounded bg-slate-800 text-indigo-300">BigInt</code> for precision;
    Rust uses native 64-bit integers compiled to WebAssembly. Same math, same result — WASM wins by a large margin.
  </p>

  <div class="grid grid-cols-2 gap-4">
    <div>
      <label for="fact-n" class="block text-xs font-semibold mb-1.5 text-slate-300 uppercase tracking-wide">n (factorial argument)</label>
      <input
        id="fact-n"
        class="w-full rounded-xl bg-slate-800 border border-slate-700 px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        type="number"
        bind:value={n}
        min="10000"
        max="1000000"
        step="10000"
        disabled={isRunning}
      />
      <p class="mt-1 text-xs text-slate-500">Complexity O(n). BigInt in JS slows down as n grows.</p>
    </div>

    <div>
      <label for="fact-runs" class="block text-xs font-semibold mb-1.5 text-slate-300 uppercase tracking-wide">Runs (averaging)</label>
      <input
        id="fact-runs"
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
    {#if !wasmReady}Loading WASM…{:else if isRunning}Running factorial benchmark…{:else}Run factorial benchmark{/if}
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
      <div class="text-xs uppercase tracking-widest text-slate-400 mb-2 font-semibold">JavaScript (BigInt)</div>
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
