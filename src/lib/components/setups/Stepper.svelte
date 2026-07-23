<script lang="ts">
	let {
		currentStep = $bindable(),
		steps
	}: {
		currentStep: string;
		steps: { id: string; label: string; icon: string }[];
	} = $props();

	function stepIndex(s: string) {
		return steps.findIndex((x) => x.id === s);
	}
</script>

<div class="flex items-center justify-center mb-8" role="list">
	{#each steps as step, i}
		<div class="flex flex-col items-center gap-2" role="listitem">
			<div class="w-10 h-10 rounded-full flex items-center justify-center text-base transition-all duration-300 border-2
				{currentStep === step.id ? 'bg-blue-500/15 border-blue-500 ring-4 ring-blue-500/10' : 
				stepIndex(currentStep) > i ? 'bg-blue-500 border-blue-500' : 
				'bg-white/5 border-white/10'}"
			>
				{#if stepIndex(currentStep) > i}
					<svg width="14" height="14" viewBox="0 0 14 14" fill="none">
						<path d="M2.5 7l3 3 6-6" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
					</svg>
				{:else}
					{step.icon}
				{/if}
			</div>
			<span class="text-xs font-medium transition-colors duration-200
				{currentStep === step.id ? 'text-blue-500' : 
				stepIndex(currentStep) > i ? 'text-blue-400' : 'text-slate-500'}"
			>
				{step.label}
			</span>
		</div>
		{#if i < steps.length - 1}
			<div class="flex-1 h-0.5 min-w-10 sm:min-w-16 mb-6 transition-colors duration-300
				{stepIndex(currentStep) > i ? 'bg-blue-500' : 'bg-white/10'}"
			></div>
		{/if}
	{/each}
</div>
