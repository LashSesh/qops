<script lang="ts">
  let animationsEnabled = true;
  let precisionDigits = 4;
</script>

<div class="flex-1 p-6 overflow-y-auto">
  <h1 class="text-2xl font-bold text-white mb-6">Settings</h1>

  <div class="max-w-2xl">
    <div class="card mb-6">
      <h2 class="text-lg font-semibold text-white mb-4">Display</h2>

      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div>
            <div class="text-white">Enable Animations</div>
            <div class="text-sm text-slate-400">Show animated transitions in the UI</div>
          </div>
          <button
            on:click={() => animationsEnabled = !animationsEnabled}
            class="w-12 h-6 rounded-full transition-colors {animationsEnabled ? 'bg-quantum-primary' : 'bg-surface-600'}"
          >
            <div
              class="w-5 h-5 bg-white rounded-full shadow transform transition-transform {animationsEnabled ? 'translate-x-6' : 'translate-x-0.5'}"
            ></div>
          </button>
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="text-white">Precision Digits</div>
            <div class="text-sm text-slate-400">Number of decimal places for values</div>
          </div>
          <input
            type="number"
            bind:value={precisionDigits}
            min="1"
            max="10"
            class="input w-20"
          />
        </div>
      </div>
    </div>

    <div class="card mb-6">
      <h2 class="text-lg font-semibold text-white mb-4">Simulation</h2>

      <div class="space-y-4">
        <div>
          <label class="block text-white mb-2">Default Shots</label>
          <input type="number" value="1000" min="100" max="10000" class="input w-40" />
        </div>

        <div>
          <label class="block text-white mb-2">Max Qubits</label>
          <input type="number" value="15" min="5" max="20" class="input w-40" />
        </div>
      </div>
    </div>

    <div class="card">
      <h2 class="text-lg font-semibold text-white mb-4">About</h2>

      <div class="space-y-2 text-slate-400">
        <p><strong class="text-white">QOPS</strong> - Quantum Operator Processing System</p>
        <p>Version: 0.1.0</p>
        <p>License: MIT</p>
        <p>
          <a href="https://github.com/LashSesh/qops" class="text-quantum-accent hover:underline">
            GitHub Repository
          </a>
        </p>
      </div>
    </div>
  </div>
</div>
