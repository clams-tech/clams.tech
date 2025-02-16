<script>
	import CheckIcon from '$lib/icons/check.svelte';
	import BitcoinSymbolIcon from '$lib/icons/bitcoin-symbol';
	import PlusIcon from '$lib/icons/plus';
	import Button from '$lib/elements/Button.svelte';
	import { REMOTE_APP_URL } from '$lib/constants';

	const communityFeatures = [
		'Unlimited Wallet Connections',
		'Onchain Payments',
		'Lightning Payments',
		'Channel Management',
		'BOLT12 Offers',
		'Forwards Dashboard',
		'Premium Features'
	];

	const premiumFeatures = [
		'All Community Features',
		'Lightning Addresses',
		'Cost Basis Calculations',
		'Profit and Loss Statements',
		'Sync Data Across Devices',
		'Hosted Payment Pages',
		'Advanced Notifications'
	];
</script>

<div class="grid grid-cols-1 gap-10 md:grid-cols-2">
	<!-- Community -->
	<div
		class="w-full rounded-3xl bg-white p-8 text-center shadow-xl dark:bg-black dark:shadow-indigo-500/50"
	>
		<h1 class="text-2xl font-semibold">Community</h1>
		<div class="flex justify-center gap-1 pt-2 tracking-wide">
			<span class="flex justify-center text-3xl font-semibold">
				<div class="w-6">
					{@html BitcoinSymbolIcon}
				</div>
				0
			</span>
			<span class="mt-3 font-medium text-gray-400 dark:text-gray-100">/ forever</span>
		</div>
		<hr class="border-1 mt-4" />
		<div class="flex flex-col justify-between gap-3 pt-8">
			{#each communityFeatures as feature, i}
				<div class="flex items-center gap-2">
					{#if i === communityFeatures.length - 1}
						<div class="w-8 rotate-45 transform fill-red-500 stroke-red-500">
							{@html PlusIcon}
						</div>
					{:else}
						<div class="w-8">
							<CheckIcon />
						</div>
					{/if}
					<span>{feature}</span>
				</div>
			{/each}
			<a class="mt-3 flex" href={REMOTE_APP_URL} target="_blank" rel="noopener noreferrer">
				<Button primary text="LFG!" />
			</a>
		</div>
	</div>
	<!-- Premium -->
	<div
		class="w-full rounded-3xl bg-white p-8 text-center shadow-xl dark:bg-black dark:shadow-indigo-500/50"
	>
		<h1 class="text-2xl font-semibold">Premium</h1>
		<div class="flex justify-center gap-1 pt-2 tracking-wide">
			<span class="flex justify-center text-3xl font-semibold">
				<div class="w-6">
					{@html BitcoinSymbolIcon}
				</div>
				TBD
			</span>
			<span class="mt-3 font-medium text-gray-400 dark:text-gray-100">/ year</span>
		</div>
		<hr class="border-1 mt-4" />
		<div class="flex flex-col justify-between gap-3 pt-8">
			{#each premiumFeatures as feature}
				<div class="flex items-center gap-2">
					<div class="w-8">
						<CheckIcon />
					</div>
					<span>{feature}</span>
				</div>
			{/each}
			<div class="mt-3 flex">
				<Button disabled text="Coming Soon..." />
			</div>
		</div>
	</div>
</div>
