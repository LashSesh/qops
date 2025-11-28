<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  interface CalibrationStep {
    step: number;
    signature: { psi: number; rho: number; omega: number };
    accepted: boolean;
    cri_triggered: boolean;
  }

  interface CalibrationResult {
    steps: CalibrationStep[];
    final_signature: { psi: number; rho: number; omega: number };
    accepted_count: number;
  }

  interface QuantumWalkResult {
    time_points: number[];
    center_probabilities: number[];
    hex_probabilities: number[];
    cube_probabilities: number[];
  }

  let calibrationResult: CalibrationResult | null = null;
  let walkResult: QuantumWalkResult | null = null;
  let loading = false;
  let error: string | null = null;

  // Calibration params
  let calibrationSteps = 20;
  let calibrationTarget = 0.85;

  // Quantum walk params
  let walkTimes = '0.5,1.0,2.0,3.0,5.0';

  async function runCalibration() {
    loading = true;
    error = null;
    try {
      calibrationResult = await invoke<CalibrationResult>('run_calibration', {
        steps: calibrationSteps,
        target: calibrationTarget,
      });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function runQuantumWalk() {
    loading = true;
    error = null;
    try {
      const times = walkTimes.split(',').map(s => parseFloat(s.trim())).filter(n => !isNaN(n));
      walkResult = await invoke<QuantumWalkResult>('run_quantum_walk', { times });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function getResonanceClass(value: number): string {
    if (value >= 0.7) return 'text-quantum-success';
    if (value >= 0.4) return 'text-quantum-warning';
    return 'text-quantum-error';
  }
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  <!-- Header -->
  <div class="p-4 border-b border-surface-700">
    <h1 class="text-xl font-bold text-white">Resonance Analyzer</h1>
    <p class="text-slate-400 text-sm mt-1">Seraphic calibration and quantum walk analysis</p>
  </div>

  <div class="flex-1 p-6 overflow-y-auto">
    {#if error}
      <div class="bg-quantum-error/20 border border-quantum-error text-quantum-error p-4 rounded-lg mb-4">
        {error}
      </div>
    {/if}

    <div class="grid grid-cols-2 gap-6">
      <!-- Seraphic Calibration -->
      <div class="card">
        <h2 class="text-lg font-semibold text-white mb-4">Seraphic Calibration</h2>

        <div class="grid grid-cols-2 gap-4 mb-4">
          <div>
            <label class="block text-sm text-slate-400 mb-2">Steps</label>
            <input type="number" bind:value={calibrationSteps} min="5" max="100" class="input w-full" />
          </div>
          <div>
            <label class="block text-sm text-slate-400 mb-2">Target Resonance</label>
            <input type="number" bind:value={calibrationTarget} min="0" max="1" step="0.05" class="input w-full" />
          </div>
        </div>

        <button on:click={runCalibration} class="btn-primary w-full mb-4" disabled={loading}>
          Run Calibration
        </button>

        {#if calibrationResult}
          <div class="bg-surface-700 p-4 rounded-lg mb-4">
            <h3 class="text-sm font-semibold text-white mb-2">Final Signature</h3>
            <div class="grid grid-cols-3 gap-2">
              <div class="text-center">
                <div class="text-xs text-slate-400">psi</div>
                <div class="text-xl font-mono {getResonanceClass(calibrationResult.final_signature.psi)}">
                  {calibrationResult.final_signature.psi.toFixed(3)}
                </div>
              </div>
              <div class="text-center">
                <div class="text-xs text-slate-400">rho</div>
                <div class="text-xl font-mono {getResonanceClass(calibrationResult.final_signature.rho)}">
                  {calibrationResult.final_signature.rho.toFixed(3)}
                </div>
              </div>
              <div class="text-center">
                <div class="text-xs text-slate-400">omega</div>
                <div class="text-xl font-mono {getResonanceClass(calibrationResult.final_signature.omega)}">
                  {calibrationResult.final_signature.omega.toFixed(3)}
                </div>
              </div>
            </div>
            <div class="text-center mt-2 text-sm text-slate-400">
              Accepted: {calibrationResult.accepted_count}/{calibrationResult.steps.length}
            </div>
          </div>

          <div class="max-h-48 overflow-y-auto">
            <table class="w-full text-sm">
              <thead class="text-slate-400 sticky top-0 bg-surface-800">
                <tr>
                  <th class="text-left py-1">Step</th>
                  <th class="text-right py-1">psi</th>
                  <th class="text-right py-1">rho</th>
                  <th class="text-right py-1">omega</th>
                  <th class="text-center py-1">Accept</th>
                </tr>
              </thead>
              <tbody>
                {#each calibrationResult.steps as step}
                  <tr class="border-t border-surface-700">
                    <td class="py-1 text-slate-300">{step.step}</td>
                    <td class="py-1 text-right font-mono">{step.signature.psi.toFixed(3)}</td>
                    <td class="py-1 text-right font-mono">{step.signature.rho.toFixed(3)}</td>
                    <td class="py-1 text-right font-mono">{step.signature.omega.toFixed(3)}</td>
                    <td class="py-1 text-center">
                      {#if step.accepted}
                        <span class="text-quantum-success">Y</span>
                      {:else}
                        <span class="text-slate-500">-</span>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <!-- Quantum Walk -->
      <div class="card">
        <h2 class="text-lg font-semibold text-white mb-4">Quantum Walk (Cube-13)</h2>

        <div class="mb-4">
          <label class="block text-sm text-slate-400 mb-2">Time Points (comma-separated)</label>
          <input type="text" bind:value={walkTimes} class="input w-full" placeholder="0.5, 1.0, 2.0" />
        </div>

        <button on:click={runQuantumWalk} class="btn-accent w-full mb-4" disabled={loading}>
          Run Quantum Walk
        </button>

        {#if walkResult}
          <div class="bg-surface-700 p-4 rounded-lg">
            <h3 class="text-sm font-semibold text-white mb-3">Probability Distribution Over Time</h3>
            <table class="w-full text-sm">
              <thead class="text-slate-400">
                <tr>
                  <th class="text-left py-1">t</th>
                  <th class="text-right py-1">P(center)</th>
                  <th class="text-right py-1">P(hex)</th>
                  <th class="text-right py-1">P(cube)</th>
                </tr>
              </thead>
              <tbody>
                {#each walkResult.time_points as t, i}
                  <tr class="border-t border-surface-600">
                    <td class="py-2 font-mono text-slate-300">{t.toFixed(1)}</td>
                    <td class="py-2 text-right">
                      <div class="flex items-center justify-end gap-2">
                        <div class="w-16 h-2 bg-surface-600 rounded overflow-hidden">
                          <div class="h-full bg-quantum-primary" style="width: {walkResult.center_probabilities[i] * 100}%"></div>
                        </div>
                        <span class="font-mono text-xs w-12">{walkResult.center_probabilities[i].toFixed(3)}</span>
                      </div>
                    </td>
                    <td class="py-2 text-right">
                      <div class="flex items-center justify-end gap-2">
                        <div class="w-16 h-2 bg-surface-600 rounded overflow-hidden">
                          <div class="h-full bg-quantum-secondary" style="width: {walkResult.hex_probabilities[i] * 100}%"></div>
                        </div>
                        <span class="font-mono text-xs w-12">{walkResult.hex_probabilities[i].toFixed(3)}</span>
                      </div>
                    </td>
                    <td class="py-2 text-right">
                      <div class="flex items-center justify-end gap-2">
                        <div class="w-16 h-2 bg-surface-600 rounded overflow-hidden">
                          <div class="h-full bg-quantum-accent" style="width: {walkResult.cube_probabilities[i] * 100}%"></div>
                        </div>
                        <span class="font-mono text-xs w-12">{walkResult.cube_probabilities[i].toFixed(3)}</span>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>

          <div class="mt-4 flex gap-4 text-sm">
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded-full bg-quantum-primary"></div>
              <span class="text-slate-400">Center (1 node)</span>
            </div>
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded-full bg-quantum-secondary"></div>
              <span class="text-slate-400">Hexagon (6 nodes)</span>
            </div>
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded-full bg-quantum-accent"></div>
              <span class="text-slate-400">Cube (6 nodes)</span>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
