<script lang="ts">
  /**
   * StageIndicator - Animated stage progress indicator for Genesis pipeline
   *
   * Shows the multi-stage progress with:
   * - Animated transitions between stages
   * - Color coding (Blue -> Violet -> Gold -> Green)
   * - Pulse effects for current stage
   */

  export let currentStage: 'discovery' | 'kosmokrator' | 'chronokrator' | 'pfauenthron' | 'finalized' = 'discovery';
  export let animating = false;
  export let showLabels = true;
  export let compact = false;

  const stages = [
    { id: 'discovery', name: 'Discovery', shortName: 'DIS', color: 'blue', icon: '1' },
    { id: 'kosmokrator', name: 'Kosmokrator', shortName: 'KOS', color: 'blue', icon: '2' },
    { id: 'chronokrator', name: 'Chronokrator', shortName: 'CHR', color: 'violet', icon: '3' },
    { id: 'pfauenthron', name: 'Pfauenthron', shortName: 'PFA', color: 'amber', icon: '4' },
    { id: 'finalized', name: 'Monolith', shortName: 'MON', color: 'emerald', icon: '5' },
  ] as const;

  function getStageIndex(stage: string): number {
    return stages.findIndex(s => s.id === stage);
  }

  function isStageActive(stage: string): boolean {
    return getStageIndex(currentStage) >= getStageIndex(stage);
  }

  function isStageComplete(stage: string): boolean {
    return getStageIndex(currentStage) > getStageIndex(stage);
  }

  function isCurrent(stage: string): boolean {
    return currentStage === stage;
  }

  function getColorClass(stage: typeof stages[number], type: 'bg' | 'text' | 'border'): string {
    const colorMap = {
      blue: { bg: 'bg-blue-500', text: 'text-blue-400', border: 'border-blue-500' },
      violet: { bg: 'bg-violet-500', text: 'text-violet-400', border: 'border-violet-500' },
      amber: { bg: 'bg-amber-500', text: 'text-amber-400', border: 'border-amber-500' },
      emerald: { bg: 'bg-emerald-500', text: 'text-emerald-400', border: 'border-emerald-500' },
    };
    return colorMap[stage.color][type];
  }

  $: progress = ((getStageIndex(currentStage) + 1) / stages.length) * 100;
</script>

<div class="stage-indicator" class:compact>
  <!-- Progress bar -->
  <div class="h-1.5 bg-surface-700 rounded-full mb-3 overflow-hidden">
    <div
      class="h-full transition-all duration-700 ease-out bg-gradient-to-r from-blue-500 via-violet-500 to-amber-500"
      style="width: {progress}%"
    ></div>
  </div>

  <!-- Stage dots/icons -->
  <div class="flex justify-between items-start">
    {#each stages as stage, idx}
      <div class="flex flex-col items-center flex-1">
        <!-- Circle/Icon -->
        <div class="relative">
          <!-- Pulse animation for current stage -->
          {#if isCurrent(stage.id) && animating}
            <div
              class="absolute inset-0 rounded-full animate-ping opacity-30 {getColorClass(stage, 'bg')}"
              style="animation-duration: 1.5s;"
            ></div>
          {/if}

          <div
            class="relative z-10 rounded-full flex items-center justify-center transition-all duration-300 {compact ? 'w-8 h-8 text-sm' : 'w-10 h-10'}"
            class:bg-surface-600={!isStageActive(stage.id)}
            class:scale-110={isCurrent(stage.id)}
          >
            {#if isStageActive(stage.id)}
              <div class="absolute inset-0 {getColorClass(stage, 'bg')} rounded-full opacity-90"></div>
            {/if}

            <span class="relative z-10 font-bold text-white">
              {#if isStageComplete(stage.id)}
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
              {:else}
                {compact ? stage.shortName[0] : stage.icon}
              {/if}
            </span>
          </div>
        </div>

        <!-- Label -->
        {#if showLabels}
          <div class="mt-2 text-center">
            <div
              class="text-xs font-medium transition-colors duration-300 {isStageActive(stage.id) ? getColorClass(stage, 'text') : 'text-slate-500'}"
            >
              {compact ? stage.shortName : stage.name}
            </div>
          </div>
        {/if}
      </div>

      <!-- Connector line between stages -->
      {#if idx < stages.length - 1}
        <div
          class="flex-1 h-0.5 mt-4 mx-1 transition-all duration-500"
          class:bg-surface-600={!isStageComplete(stage.id)}
        >
          {#if isStageComplete(stage.id)}
            <div class="h-full w-full bg-gradient-to-r {idx === 0 ? 'from-blue-500 to-blue-400' : idx === 1 ? 'from-blue-500 to-violet-500' : idx === 2 ? 'from-violet-500 to-amber-500' : 'from-amber-500 to-emerald-500'}"></div>
          {/if}
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  .compact .stage-indicator {
    @apply text-xs;
  }

  @keyframes ping {
    75%, 100% {
      transform: scale(1.5);
      opacity: 0;
    }
  }

  .animate-ping {
    animation: ping 1.5s cubic-bezier(0, 0, 0.2, 1) infinite;
  }
</style>
