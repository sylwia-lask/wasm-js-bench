<script lang="ts">
  import DemoFactorialTab from './lib/components/DemoFactorialTab.svelte';
  import DemoImageTab from './lib/components/DemoImageTab.svelte';
  import DemoMatrixTab from './lib/components/DemoMatrixTab.svelte';

  type TabId = 'js-strong' | 'wasm-strong' | 'image';

  const tabs: { id: TabId; label: string; subtitle: string }[] = [
    { id: 'js-strong', label: 'JS is already fast', subtitle: 'JS wins or ties' },
    { id: 'wasm-strong', label: 'WASM shines', subtitle: 'Rust clearly wins' },
    { id: 'image', label: 'Real-world demo', subtitle: 'Image processing' }
  ];

  let activeTab: TabId = 'js-strong';
</script>

<main class="h-screen overflow-hidden flex flex-col bg-slate-950 text-slate-100 p-4">
  <div class="flex-1 flex flex-col min-h-0 max-w-5xl w-full mx-auto rounded-3xl bg-slate-900 shadow-2xl border border-slate-800 overflow-hidden">

    <!-- Compact header -->
    <div class="px-8 pt-5 pb-4 border-b border-slate-800 shrink-0">
      <div class="flex items-center justify-between mb-4">
        <h1 class="text-3xl font-black tracking-tight">Rust + WebAssembly vs JavaScript</h1>
        <p class="text-sm text-slate-400 text-right max-w-xs leading-snug">
          Where JS holds its own, where WASM dominates, and a real-world UI demo.
        </p>
      </div>

      <div class="flex gap-3">
        {#each tabs as tab}
          <button
            class={`flex-1 px-5 py-3 rounded-2xl border-2 transition-all
              ${activeTab === tab.id
                ? 'bg-indigo-600 border-indigo-400 text-white shadow-lg shadow-indigo-900/50'
                : 'bg-slate-800 border-slate-700 text-slate-200 hover:bg-slate-700 hover:border-slate-500'}`}
            on:click={() => (activeTab = tab.id)}
          >
            <div class="font-bold text-base">{tab.label}</div>
            <div class="text-xs text-slate-300/70 mt-0.5">{tab.subtitle}</div>
          </button>
        {/each}
      </div>
    </div>

    <!-- Scrollable content area -->
    <div class="flex-1 min-h-0 overflow-y-auto px-8 py-6">
      {#if activeTab === 'js-strong'}
        <DemoMatrixTab />
      {:else if activeTab === 'wasm-strong'}
        <DemoFactorialTab />
      {:else}
        <DemoImageTab />
      {/if}
    </div>

  </div>
</main>
