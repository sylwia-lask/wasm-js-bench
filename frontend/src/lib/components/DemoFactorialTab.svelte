<script lang="ts">
  import { onMount } from 'svelte';
  import init, { factorial_mod as factorialModWasm } from '../wasm/wasm_js_bench.js';
  import { jsFactorialMod } from '../logic/factorialMod.js';

  let wasmReady = false;

  let n: number = 500_000; 
  let runs: number = 3;   

  let jsTime: string | null = null;
  let wasmTime: string | null = null;
  let speedup: string | null = null;

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
    jsTime = null;
    wasmTime = null;
    speedup = null;

    jsResult = null;
    wasmResult = null;

    const numericN = Number(n);
    const numericRuns = Number(runs);

    const js = bench(jsFactorialMod, numericN, numericRuns);
    jsTime = js.ms.toFixed(2);
    jsResult = js.result;

    if (!wasmReady) return;

    const wasm = bench(factorialModWasm as (arg: number) => number, numericN, numericRuns);
    wasmTime = wasm.ms.toFixed(2);
    wasmResult = wasm.result;

    speedup = (js.ms / wasm.ms).toFixed(1);
  }
</script>

<div class="space-y-4">
  <h2 class="text-lg font-semibold">
    Tab 2 – when WASM really shines
  </h2>
  <p class="text-sm text-slate-300">
    Here we compute <code class="px-1 rounded bg-slate-800">n! mod 1&nbsp;000&nbsp;000&nbsp;007</code>.
    JavaScript uses <code>BigInt</code> to stay exact, while Rust uses native 64-bit integers compiled to WebAssembly.
    Same math, same result – but WASM usually finishes several times faster.
  </p>

  <div class="grid grid-cols-2 gap-4 mb-2">
    <div>
      <label class="block text-xs font-medium mb-1">n (factorial argument)</label>
      <input
        class="w-full rounded-md bg-slate-800 border border-slate-700 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        type="number"
        bind:value={n}
        min="10_000"
        max="1_000_000"
        step="10_000"
      />
      <p class="mt-1 text-[11px] text-slate-500">
        Complexity is O(n). BigInt in JS becomes noticeably slower as n grows.
      </p>
    </div>

    <div>
      <label class="block text-xs font-medium mb-1">Runs (averaging)</label>
      <input
        class="w-full rounded-md bg-slate-800 border border-slate-700 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
        type="number"
        bind:value={runs}
        min="1"
        max="10"
      />
    </div>
  </div>

  <button
    class="w-full py-2.5 rounded-md bg-indigo-500 hover:bg-indigo-400 text-sm font-semibold transition disabled:opacity-50"
    on:click={runBenchmark}
    disabled={!wasmReady}
  >
    {#if wasmReady}
      Run factorial benchmark
    {:else}
      Loading WASM…
    {/if}
  </button>

  <div class="mt-4 grid grid-cols-2 gap-4 text-sm">
    <div class="p-3 rounded-lg bg-slate-800 border border-slate-700">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">JavaScript (BigInt)</div>
      <div class="text-lg font-mono">
        {#if jsTime !== null}{jsTime} ms{:else}–{/if}
      </div>
    </div>

    <div class="p-3 rounded-lg bg-slate-800 border border-slate-700">
      <div class="text-xs uppercase tracking-wide text-slate-400 mb-1">Rust (WASM)</div>
      <div class="text-lg font-mono">
        {#if wasmTime !== null}{wasmTime} ms{:else}(waiting…){/if}
      </div>
    </div>
  </div>

  <div class="mt-4 grid grid-cols-1 gap-2 text-xs text-slate-400">
    {#if jsResult !== null}
      <div>
        JS result: <span class="font-mono text-slate-100">{jsResult}</span>
      </div>
    {/if}

    {#if wasmResult !== null}
      <div>
        WASM result: <span class="font-mono text-slate-100">{wasmResult}</span>
      </div>
    {/if}
  </div>

  {#if speedup}
    <div class="mt-3 text-xs text-emerald-400">
      Rust is about <span class="font-semibold">{speedup}×</span> faster than JS.
    </div>
  {/if}
</div>
