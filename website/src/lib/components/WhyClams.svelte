<script lang="ts">
	import wood from '$lib/images/wood.png';
	import marble from '$lib/images/marble.png';
	import leather from '$lib/images/leather.png';

	const features = [
		{
			title: 'Privacy First',
			image: wood,
			imageAlt: 'Wood',
			imagePosition: 'right',
			points: [
				'All data remains local — never uploaded, tracked, or shared.',
				'Zero third-party tracking or analytics.',
				'No account or personal information required to use the app.',
				'Audit-ready without compromising confidentiality.',
				'Connect personal Electrum server for maximum sovereignty.',
				'Encrypted backups, exports and support communications.'
			]
		},
		{
			title: 'Bitcoin Only',
			image: marble,
			imageAlt: 'Marble',
			imagePosition: 'left',
			points: [
				'First class support for on-chain, Lightning, and multi-signature setups.',
				'Seamless integration with most popular Bitcoin wallets.',
				'First to support new Bitcoin payment protocols.',
				'Pioneering the standard for Bitcoin-native accounting.',
				'Designed for those living on a Bitcoin standard.',
				'Created by long-time Bitcoiners who understand your needs.'
			]
		},
		{
			title: 'Elite Performance',
			image: leather,
			imageAlt: 'Leather',
			imagePosition: 'right',
			points: [
				'Native desktop app for lightning-fast performance.',
				'Handles millions of transactions without breaking a sweat.',
				'Instant insights with advanced caching.',
				'Automatically transforms complex Bitcoin data into actionable insights.',
				'Designed for reliability across high-volume, multi-profile environments.',
				'Robust offline capabilities for data resilience.'
			]
		}
	];
</script>

<section class="px-4 py-16">
	<div class="m-auto flex max-w-5xl flex-col gap-8">
		<div class="flex flex-col gap-4">
			<h1 class="text-center text-6xl">The Standard for Bitcoin Accounting</h1>
		</div>
		<div class="mt-8 grid gap-16">
			{#each features as feature}
				<div
					class="grid h-[400px] gap-8 rounded-md border border-gray-500 bg-white px-6 py-8 md:grid-cols-2"
				>
					{#if feature.imagePosition === 'left'}
						<div class="relative h-full w-full overflow-hidden">
							<img
								src={feature.image}
								alt={feature.imageAlt}
								class="absolute inset-0 h-full w-full rounded-md object-cover"
							/>
						</div>
					{/if}

					<div class="overflow-auto">
						<p class="text-2xl font-bold">{feature.title}</p>
						<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
							{#each feature.points as point}
								<li>{point}</li>
							{/each}
						</ul>
					</div>

					{#if feature.imagePosition === 'right'}
						<div class="relative h-full w-full overflow-hidden">
							<img
								src={feature.image}
								alt={feature.imageAlt}
								class="absolute inset-0 h-full w-full rounded-md object-cover"
							/>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</section>
