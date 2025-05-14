<script lang="ts">
	import check from '$lib/icons/check.svg';
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
					class="grid gap-8 rounded-md border border-gray-500 bg-white px-6 py-8 md:h-[500px] md:grid-cols-2"
				>
					{#if feature.imagePosition === 'left'}
						<div class="relative h-full w-full overflow-hidden">
							<img
								src={feature.image}
								alt={feature.imageAlt}
								class="inset-0 h-full max-h-[200px] w-full rounded-md object-cover md:absolute md:max-h-full"
							/>
						</div>
					{/if}

					<div class="overflow-auto">
						<p class="text-2xl font-bold">{feature.title}</p>
						<ul class="mt-4 flex flex-col gap-2">
							{#each feature.points as point}
								<li class="flex items-center gap-2">
									<img src={check} alt="check" class="inline-block w-6" />
									<p>{point}</p>
								</li>
							{/each}
						</ul>
					</div>

					{#if feature.imagePosition === 'right'}
						<div class="relative h-full w-full overflow-hidden">
							<img
								src={feature.image}
								alt={feature.imageAlt}
								class="inset-0 h-full max-h-[200px] w-full rounded-md object-cover md:absolute md:max-h-full"
							/>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</section>
