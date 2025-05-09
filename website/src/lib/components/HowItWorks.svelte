<script>
	import SyncIcon from '$lib/icons/sync';
	import EnhanceIcon from '$lib/icons/enhance';
	import VisualizeIcon from '$lib/icons/visualize';
	import ReportIcon from '$lib/icons/report';
	import ContentContainer from '../shared/ContentContainer.svelte';

	const tiles = [
		{
			title: 'Sync',
			description:
				'Connect unlimited wallets and automatically import transactions into a private local hub.',
			icon: SyncIcon
		},
		{
			title: 'Enhance',
			description:
				'Auto-convert raw transaction data into structured, audit-ready financial records.',
			icon: EnhanceIcon
		},
		{
			title: 'Visualize',
			description: 'Gain real-time insights with customizable, interactive dashboards.',
			icon: VisualizeIcon
		},
		{
			title: 'Report',
			description: 'Generate audit-ready reports for tax and financial planning.',
			icon: ReportIcon
		}
	];
</script>

<ContentContainer title="How it works">
	<div class="grid grid-cols-1 gap-8 md:grid-cols-2">
		{#each tiles as { title, description, icon }}
			<div class="m-auto flex w-full flex-col gap-2 rounded-lg border p-4">
				<h3 class="inline text-2xl font-semibold text-gray-900 dark:text-white">{title}</h3>
				<p class="text-gray-900 dark:text-white">{description}</p>
				<div class="m-auto mt-4 flex justify-center sm:w-40 md:w-60">
					{@html icon}
				</div>
			</div>
		{/each}
	</div>
	<div class="mt-12 flex items-center justify-center gap-x-6">
		<a
			href="/downloads"
			class="text-md rounded-md bg-indigo-600 px-3.5 py-2.5 font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
			>Get Started</a
		>
	</div>
</ContentContainer>
