<script lang="ts">
	// import { fade, slide } from 'svelte/transition';

	// interface FeatureCategory {
	// 	title: string;
	// 	features: Feature[];
	// }

	interface Feature {
		name: string;
		description: string;
		expanded: boolean;
	}

	interface UserType {
		title: string;
		description: string;
		features: Feature[];
		icon: string;
		expanded: boolean;
	}

	let userTypes: UserType[] = [
		{
			title: 'Individual',
			description:
				'Tired of guessing your holdings or dreading tax season? Clams turns stress into clarity.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" /></svg>`,
			expanded: true,
			features: [
				{
					name: 'Unified Balance',
					description: 'All your wallet balances in one dashboard.',
					expanded: false
				},
				{
					name: 'Cost Basis',
					description: 'View cost basis, realized gains, and losses for all transactions.',
					expanded: false
				},
				{
					name: 'Capital Gains',
					description: 'Provide your accountant exactly what he needs come tax season.',
					expanded: false
				},
				{
					name: 'Record Keeping',
					description: 'Identify, label and categorize all transactions with ease.',
					expanded: false
				}
			]
		},
		{
			title: 'Business Owner',
			description:
				'Make Bitcoin a business advantage with real-time financial visibility and tax-ready reports.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 21h19.5m-18-18v18m10.5-18v18m6-13.5V21M6.75 6.75h.75m-.75 3h.75m-.75 3h.75m3-6h.75m-.75 3h.75m-.75 3h.75M6.75 21v-3.375c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21M3 3h12m-.75 4.5H21m-3.75 3.75h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008z" /></svg>`,
			expanded: false,
			features: [
				{
					name: 'Accounting',
					description: 'Manage Bitcoin transactions alongside traditional finances.',
					expanded: false
				},
				{
					name: 'Tax Compliance',
					description: 'Automatically generate accurate reports for tax filings.',
					expanded: false
				},
				{
					name: 'Customer Insights',
					description: 'Track Bitcoin payment patterns and optimize your revenue.',
					expanded: false
				},
				{
					name: 'Operational Support',
					description: 'Inform payroll, refunds, and savings strategies.',
					expanded: false
				}
			]
		},
		{
			title: 'Accountant',
			description:
				'No more Bitcoin headaches at tax time. Track, reconcile, and explain client activity with ease.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" /></svg>`,
			expanded: false,
			features: [
				{
					name: 'Journal Entries',
					description: 'Auto-conversion of disparate Bitcoin data into unified journal entries.',
					expanded: false
				},
				{
					name: 'Reports',
					description: 'Generate clear reports that clients and tax agencies understand.',
					expanded: false
				},
				{
					name: 'Complex Transaction Handling',
					description: 'Process advanced Onchain and Lightning transactions with precise accuracy.',
					expanded: false
				},
				{
					name: 'Audit Protection',
					description: 'Back every filing with comprehensive transaction records.',
					expanded: false
				}
			]
		},
		{
			title: 'CFO',
			description:
				'Get the audit-ready data and strategic insights you need to manage Bitcoin with confidence.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" /></svg>`,
			expanded: false,
			features: [
				{
					name: 'Compliant Reporting',
					description: 'Create GAAP-compliant reports that satisfy auditors and regulators.',
					expanded: false
				},
				{
					name: 'Treasury Performance',
					description: 'Track Bitcoin holdings performance with precision metrics.',
					expanded: false
				},
				{
					name: 'Stakeholder Communication',
					description: 'Present clear Bitcoin financials to boards and investors.',
					expanded: false
				},
				{
					name: 'Decision Support',
					description: 'Make confident Bitcoin treasury decisions backed by data.',
					expanded: false
				}
			]
		}
	];
</script>

<section class="px-4 py-16">
	<div class="m-auto flex max-w-5xl flex-col gap-8">
		<div class="mb-8 flex flex-col gap-4">
			<h1 class="text-center text-6xl">Powerful Solutions for Real Problems</h1>
		</div>

		<!-- User Type Cards -->
		<div class="grid grid-cols-2 gap-8 md:grid-cols-2">
			{#each userTypes as { icon, title, description, features }}
				<div class="flex flex-col items-center rounded-lg border border-gray-500 px-6 py-8">
					<div class="mb-4 flex items-center justify-center rounded-full">
						{@html icon}
					</div>
					<p class="text-xl font-bold">{title}</p>
					<p class="mt-4 italic">{description}</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						{#each features as { name, description }}
							<li>
								<div class="mt-4">
									<p class="font-bold">{name}</p>
									<p>{description}</p>
								</div>
							</li>
						{/each}
					</ul>
				</div>
			{/each}
		</div>
	</div>
</section>
