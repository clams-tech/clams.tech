<script lang="ts">
	import { API_DOCS_URL } from '$lib/constants';

	const freeTier = [
		{
			category: 'Transactions',
			features: ['21K']
		},
		{
			category: 'Cost per Report',
			features: ['Cap Gains - $75', 'Quickbooks Export - $150']
		}
	];

	const paidTier1 = [
		{
			category: 'Transactions',
			features: ['210K']
		},
		{
			category: 'Cost per Report',
			features: ['Cap Gains - $70', 'Quickbooks Export - $135']
		}
	];

	const paidTier2 = [
		{
			category: 'Transactions',
			features: ['21M']
		},
		{
			category: 'Cost per Report',
			features: ['Cap Gains - $50', 'Quickbooks Export - $120']
		}
	];
</script>

<section class="mx-auto mt-24 max-w-5xl lg:px-6">
	<h2 class="text-center text-4xl font-bold">Pricing</h2>

	<div class="mx-auto mt-12 flex max-w-5xl flex-wrap justify-between gap-4">
		<!-- Free tier -->
		<section class="w-full p-8 md:flex-1">
			<h3 id="tier-basic" class="text-sm font-semibold leading-6 text-gray-900 dark:text-white">
				Starter
			</h3>
			<p class="mt-2 flex items-baseline gap-x-1 text-gray-900 dark:text-white">
				<span class="text-4xl font-bold">FREE</span>
				<span class="text-sm font-semibold">/forever</span>
			</p>
			<a
				href={API_DOCS_URL}
				rel="noopener noreferrer"
				target="_blank"
				aria-describedby="tier-basic"
				class="text-md mt-8 block rounded-md px-3 py-2 text-center font-semibold leading-6 text-light-purple ring-1 ring-inset ring-indigo-200 hover:ring-indigo-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:text-dark-purple"
				>Get Started</a
			>
			<ul role="list" class="mt-10 space-y-4 text-sm leading-6 text-gray-900 dark:text-white">
				{#each freeTier as { category, features }}
					<li class="flex flex-col gap-2">
						<span class="font-bold">{category}</span>
						<ul
							role="list"
							class="ml-10d space-y-4 text-sm leading-6 text-gray-900 dark:text-white"
						>
							{#each features as feature}
								<li class="flex gap-x-3">
									<svg
										class="h-6 w-5 flex-none text-light-purple dark:text-dark-purple"
										viewBox="0 0 20 20"
										fill="currentColor"
										aria-hidden="true"
									>
										<path
											fill-rule="evenodd"
											d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
											clip-rule="evenodd"
										/>
									</svg>
									<span>{feature}</span>
								</li>
							{/each}
						</ul>
					</li>
				{/each}
			</ul>
		</section>
		<!-- Paid Tier 1 -->
		<section class="w-full rounded-xl bg-gray-400/5 p-8 ring-1 ring-inset ring-gray-200 md:flex-1">
			<h3 id="tier-pro" class="text-sm font-semibold leading-6 text-gray-900 dark:text-white">
				Tier 1
			</h3>
			<p class="mt-2 flex items-baseline gap-x-1 text-gray-900 dark:text-white">
				<span class="text-4xl font-bold">$500</span>
				<span class="text-sm font-semibold">/month</span>
			</p>
			<div
				aria-describedby="tier-pro"
				class="text-md pointer-events-none mt-8 block rounded-md bg-indigo-600 px-3 py-2 text-center font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
			>
				Coming Soon...
			</div>
			<ul role="list" class="mt-10 space-y-4 text-sm leading-6 text-gray-900 dark:text-white">
				{#each paidTier1 as { category, features }}
					<li class="flex flex-col gap-2">
						<span class="font-bold">{category}</span>
						<ul
							role="list"
							class="ml-10d space-y-4 text-sm leading-6 text-gray-900 dark:text-white"
						>
							{#each features as feature}
								<li class="flex gap-x-3">
									<svg
										class="h-6 w-5 flex-none text-light-purple dark:text-dark-purple"
										viewBox="0 0 20 20"
										fill="currentColor"
										aria-hidden="true"
									>
										<path
											fill-rule="evenodd"
											d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
											clip-rule="evenodd"
										/>
									</svg>
									<span>{feature}</span>
								</li>
							{/each}
						</ul>
					</li>
				{/each}
			</ul>
		</section>
		<!-- Paid Tier 2 -->
		<section class="w-full rounded-xl bg-gray-400/5 p-8 md:flex-1">
			<h3 id="tier-pro" class="text-sm font-semibold leading-6 text-gray-900 dark:text-white">
				Tier 2
			</h3>
			<p class="mt-2 flex items-baseline gap-x-1 text-gray-900 dark:text-white">
				<span class="text-4xl font-bold">$1500</span>
				<span class="text-sm font-semibold">/month</span>
			</p>
			<div
				aria-describedby="tier-pro"
				class="text-md pointer-events-none mt-8 block rounded-md bg-indigo-600 px-3 py-2 text-center font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
			>
				Coming Soon...
			</div>
			<ul role="list" class="mt-10 space-y-4 text-sm leading-6 text-gray-900 dark:text-white">
				{#each paidTier2 as { category, features }}
					<li class="flex flex-col gap-2">
						<span class="font-bold">{category}</span>
						<ul
							role="list"
							class="ml-10d space-y-4 text-sm leading-6 text-gray-900 dark:text-white"
						>
							{#each features as feature}
								<li class="flex gap-x-3">
									<svg
										class="h-6 w-5 flex-none text-light-purple dark:text-dark-purple"
										viewBox="0 0 20 20"
										fill="currentColor"
										aria-hidden="true"
									>
										<path
											fill-rule="evenodd"
											d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
											clip-rule="evenodd"
										/>
									</svg>
									<span>{feature}</span>
								</li>
							{/each}
						</ul>
					</li>
				{/each}
			</ul>
		</section>
	</div>
</section>
