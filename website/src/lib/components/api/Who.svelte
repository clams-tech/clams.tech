<script lang="ts">
	import WalletIcon from '$lib/icons/wallet.js';
	import KeysIcon from '$lib/icons/keys.js';
	import LightningIcon from '$lib/icons/lightning-outline.js';
	import { darkMode } from '$lib/stores';

	// Reactive subscription to dark mode store
	$: iconColor = $darkMode ? 'white' : 'black';

	// Feature list
	const features = [
		{
			title: 'Wallets',
			description: 'Offer automated accounting without leaving your app.',
			icon: WalletIcon
		},
		{
			title: 'Exchanges',
			description: 'Simplify tax season with capital gains reports.',
			icon: KeysIcon
		},
		{
			title: 'Payment Platforms',
			description: 'Empower merchants with profit and loss statements.',
			icon: LightningIcon
		}
	];
</script>

<section class="mx-auto max-w-5xl lg:px-6">
	<h2 class="text-center text-4xl font-bold">Who is it for?</h2>

	<div class="mt-12 grid grid-cols-1 gap-8 md:grid-cols-3">
		{#each features as { title, description, icon }}
			<div class="rounded-lg border border-gray-600 p-6 text-center shadow-md dark:border-gray-500">
				<div class="m-auto w-16">
					{@html `<div style="stroke: ${iconColor}">${icon}</div>`}
				</div>
				<h3 class="mt-4 text-2xl font-semibold">{title}</h3>
				<p class="mt-2 text-gray-600 dark:text-gray-300">{description}</p>
			</div>
		{/each}
	</div>
</section>
