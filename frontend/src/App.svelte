<script lang="ts">
  import DemoFactorialTab from './lib/components/DemoFactorialTab.svelte';
  import DemoImageTab from './lib/components/DemoImageTab.svelte';
  import DemoMatrixTab from './lib/components/DemoMatrixTab.svelte';

  type TabId = 'js-strong' | 'wasm-strong' | 'image';

  const tabs: { id: TabId; label: string; subtitle: string }[] = [
    { id: 'js-strong', label: 'JS is already fast', subtitle: 'A case where JS wins or ties' },
    { id: 'wasm-strong', label: 'WASM shines', subtitle: 'A case where Rust clearly wins' },
    { id: 'image', label: 'Real-world demo', subtitle: 'Image processing with WASM' }
  ];

  let activeTab: TabId = 'js-strong';
</script>

<main class="min-h-screen bg-slate-950 text-slate-100 flex items-center justify-center">
  <div class="max-w-5xl w-full mx-4 p-6 rounded-2xl bg-slate-900 shadow-xl border border-slate-800">
    <h1 class="text-2xl font-semibold mb-2 text-center">
      Rust + WebAssembly vs JavaScript
    </h1>
    <p class="text-sm text-slate-300 mb-6 text-center">
      Three small benchmarks to show where JavaScript is already great, where WASM shines, 
      and how this looks in a real-world UI.
    </p>

    <div class="flex gap-2 mb-6 overflow-x-auto">
      {#each tabs as tab}
        <button
          class={`px-3 py-2 rounded-xl text-sm border transition 
            ${activeTab === tab.id 
              ? 'bg-indigo-500 border-indigo-400 text-white' 
              : 'bg-slate-800 border-slate-700 text-slate-200 hover:bg-slate-700'}`}
          on:click={() => (activeTab = tab.id)}
        >
          <div class="font-medium">{tab.label}</div>
          <div class="text-[11px] text-slate-300/80">{tab.subtitle}</div>
        </button>
      {/each}
    </div>

    {#if activeTab === 'js-strong'}
      <DemoMatrixTab />
    {:else if activeTab === 'wasm-strong'}
      <DemoFactorialTab />
    {:else}
      <DemoImageTab />
    {/if}
  </div>
</main>
