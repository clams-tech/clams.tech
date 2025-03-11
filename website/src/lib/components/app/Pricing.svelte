<script>
	import ContentContainer from '../shared/ContentContainer.svelte';
	import { TipModal } from 'tip-modal';
	import 'tip-modal/app.css';

	// const basic = [
	// 	{
	// 		category: 'Connections',
	// 		features: ['Unlimited']
	// 	},
	// 	{
	// 		category: 'Dashboard',
	// 		features: ['Balance tracker', 'Cost basis']
	// 	}
	// ];

	// const pro = [
	// 	{
	// 		category: 'Connections',
	// 		features: ['Unlimited']
	// 	},
	// 	{
	// 		category: 'Dashboard',
	// 		features: ['Balance tracker', 'Cost basis']
	// 	},
	// 	{
	// 		category: 'Reports',
	// 		features: ['Cap gains', 'Profit and loss']
	// 	},
	// 	{
	// 		category: 'Operational',
	// 		features: [
	// 			'Secure data sharing',
	// 			'Multi-user access',
	// 			'Cross-device sync',
	// 			'Encrypted data backups'
	// 		]
	// 	}
	// ];

	let showModal = false;

	function openModal() {
		showModal = true;
	}

	function closeModal() {
		showModal = false;
	}
</script>

<ContentContainer title="Pricing" scrollTo="pricing">
	<div
		class="my-12 flex flex-col items-center text-center text-lg leading-8 text-gray-600 dark:text-gray-100"
	>
		<p class="space-y-4">While the app is in beta, we're adopting a Value for Value (V4V) model.</p>
		<p class="space-y-4">
			All features are free, and you can support our efforts using the button below.
		</p>
		<p class="space-y-4">
			Once we're out of beta, we’ll introduce a yearly subscription, with beta testers receiving a
			discount.
		</p>
		<TipModal {showModal} {openModal} {closeModal}>
			<!-- svelte-ignore a11y_click_events_have_key_events -->
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div on:click={openModal} slot="button" class="mt-8 flex justify-center">
				<button
					class="text-md rounded-md bg-indigo-600 px-3.5 py-2.5 font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
				>
					Donate
				</button>
			</div>
		</TipModal>
	</div>
	<!-- <div class="mx-auto flex max-w-5xl flex-wrap justify-between gap-4">
		<section class="w-full p-8 md:flex-1">
			<h3 id="tier-basic" class="text-sm font-semibold leading-6 text-gray-900 dark:text-white">
				Basic
			</h3>
			<p class="mt-2 flex items-baseline gap-x-1 text-gray-900 dark:text-white">
				<span class="text-4xl font-bold">FREE</span>
				<span class="text-sm font-semibold">/forever</span>
			</p>
			<a
				href="/downloads"
				aria-describedby="tier-basic"
				class="text-md mt-8 block rounded-md px-3 py-2 text-center font-semibold leading-6 text-light-purple ring-1 ring-inset ring-indigo-200 hover:ring-indigo-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 dark:text-dark-purple"
				>Get Started</a
			>
			<ul role="list" class="mt-10 space-y-4 text-sm leading-6 text-gray-900 dark:text-white">
				{#each basic as { category, features }}
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
		<section class="w-full rounded-xl bg-gray-400/5 p-8 ring-1 ring-inset ring-gray-200 md:flex-1">
			<h3 id="tier-pro" class="text-sm font-semibold leading-6 text-gray-900 dark:text-white">
				Pro
			</h3>
			<p class="mt-2 flex items-baseline gap-x-1 text-gray-900 dark:text-white">
				<span class="text-4xl font-bold">$TBD</span>
				<span class="text-sm font-semibold">/month</span>
			</p>
			<div
				aria-describedby="tier-pro"
				class="text-md pointer-events-none mt-8 block rounded-md bg-indigo-600 px-3 py-2 text-center font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
			>
				Coming Soon...
			</div>
			<ul role="list" class="mt-10 space-y-4 text-sm leading-6 text-gray-900 dark:text-white">
				{#each pro as { category, features }}
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
	</div> -->
</ContentContainer>
