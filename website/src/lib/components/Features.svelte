<script lang="ts">
	import { fade, slide } from 'svelte/transition';

	interface FeatureCategory {
		title: string;
		features: Feature[];
	}

	interface Feature {
		name: string;
		description: string;
		expanded: boolean;
	}

	interface UserType {
		title: string;
		description: string;
		categories: FeatureCategory[];
		icon: string;
		expanded: boolean;
	}

	let userTypes: UserType[] = [
		{
			title: 'Individual',
			description:
				'Tired of guessing your holdings or dreading tax season? Clams turns stress into clarity and gives you full control.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" /></svg>`,
			expanded: true,
			categories: [
				{
					title: 'Taxes',
					features: [
						{
							name: 'Cost Basis Tracking',
							description:
								'Automatically calculate cost basis, realized gains, and losses for all transactions.',
							expanded: false
						},
						{
							name: 'Privacy-Preserving Reports',
							description: 'Generate audit-ready tax reports without exposing sensitive data.',
							expanded: false
						},
						{
							name: 'Accountant-Friendly',
							description: 'Provide what your accountant needs without the panic or hassle.',
							expanded: false
						}
					]
				},
				{
					title: 'Spending & Transaction Clarity',
					features: [
						{
							name: 'Informed Spending',
							description: 'Know when to spend Bitcoin with clear tax impact and capital gains.',
							expanded: false
						},
						{
							name: 'Transaction Management',
							description: 'Identify and categorize all Bitcoin transactions with ease.',
							expanded: false
						}
					]
				},
				{
					title: 'Income & Events',
					features: [
						{
							name: 'Income Tracking',
							description:
								'Track Bitcoin from income, mining, gifts, or services with annotations.',
							expanded: false
						},
						{
							name: 'Family Bitcoin Management',
							description: 'Track Bitcoin delegated to family with full visibility.',
							expanded: false
						}
					]
				},
				{
					title: 'Visibility',
					features: [
						{
							name: 'Unified Portfolio Management',
							description: 'See all wallets in one dashboard—no more guessing your position.',
							expanded: false
						},
						{
							name: 'Shared Wallet Visibility',
							description: 'Track family holdings securely across shared wallets.',
							expanded: false
						},
						{
							name: 'Performance Insights',
							description:
								'Turn confusing data into clear financial position and performance metrics.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'Business Owner',
			description:
				'Make Bitcoin a business advantage with real-time financial visibility and tax-ready reports.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 21h19.5m-18-18v18m10.5-18v18m6-13.5V21M6.75 6.75h.75m-.75 3h.75m-.75 3h.75m3-6h.75m-.75 3h.75m-.75 3h.75M6.75 21v-3.375c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21M3 3h12m-.75 4.5H21m-3.75 3.75h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Accounting & Tax Compliance',
					features: [
						{
							name: 'Bitcoin-Optimized Accounting',
							description: 'QuickBooks-like experience designed specifically for Bitcoin.',
							expanded: false
						},
						{
							name: 'Compliant Financial Management',
							description: 'Accurate, compliant records for balance sheet management.',
							expanded: false
						},
						{
							name: 'Stakeholder Reporting',
							description: 'Clean, investor-ready reports for all stakeholders.',
							expanded: false
						}
					]
				},
				{
					title: 'Payments & Operations',
					features: [
						{
							name: 'Transaction Management',
							description: 'Streamline and reconcile all Bitcoin payments and expenses.',
							expanded: false
						},
						{
							name: 'Documentation Suite',
							description: 'Automated record-keeping for vendor and customer transactions.',
							expanded: false
						},
						{
							name: 'Operational Integration',
							description: 'Support Bitcoin payroll, refunds, and cold storage strategies.',
							expanded: false
						}
					]
				},
				{
					title: 'Treasury Management',
					features: [
						{
							name: 'Unified Bitcoin Position',
							description: 'See complete position with realized vs. unrealized gains at a glance.',
							expanded: false
						},
						{
							name: 'Strategic Fund Allocation',
							description: 'Segment Bitcoin by business use for proper financial management.',
							expanded: false
						}
					]
				},
				{
					title: 'Performance & Analytics',
					features: [
						{
							name: 'Performance Dashboard',
							description: 'Monitor Bitcoin holdings and performance across wallets and teams.',
							expanded: false
						},
						{
							name: 'Business Impact Analysis',
							description: "Measure Bitcoin's effect on your bottom line with clear metrics.",
							expanded: false
						},
						{
							name: 'Revenue Intelligence',
							description: 'Track Bitcoin revenue percentage and usage patterns.',
							expanded: false
						},
						{
							name: 'Customer Insights',
							description: 'Analyze transaction values, frequency, and customer behavior.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'Accountant',
			description:
				'No more Bitcoin headaches at tax time. Track, reconcile, and explain client activity with ease.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Tax & Regulatory Compliance',
					features: [
						{
							name: 'Tax Automation',
							description:
								'Accurately calculate cost basis and capital gains, even for partial sales.',
							expanded: false
						},
						{
							name: 'Standards Compliance',
							description: 'Stay current with FASB and regulatory expectations.',
							expanded: false
						},
						{
							name: 'Complex Event Handling',
							description: 'Process forks, gifts, and multi-signature arrangements correctly.',
							expanded: false
						}
					]
				},
				{
					title: 'Client Service Excellence',
					features: [
						{
							name: 'Bitcoin Translation',
							description: 'Turn technical Bitcoin concepts into familiar accounting terms.',
							expanded: false
						},
						{
							name: 'Methodical Approach',
							description: 'Follow clear, established practices with confidence.',
							expanded: false
						},
						{
							name: 'Client Confidentiality',
							description: 'Maintain privacy while generating necessary reports.',
							expanded: false
						}
					]
				},
				{
					title: 'Advanced Client Management',
					features: [
						{
							name: 'Bitcoin Infrastructure',
							description:
								'Handle Lightning nodes, multiple wallets, and various transaction types.',
							expanded: false
						},
						{
							name: 'Portfolio Organization',
							description: 'Organize client portfolios with intuitive tools.',
							expanded: false
						},
						{
							name: 'Longitudinal Records',
							description: 'Track activity across fiscal years with blockchain verification.',
							expanded: false
						},
						{
							name: 'Multi-Client Efficiency',
							description: 'Batch-process reporting while maintaining individual client attention.',
							expanded: false
						}
					]
				},
				{
					title: 'Reporting & Audit Readiness',
					features: [
						{
							name: 'Tailored Reporting',
							description:
								'Generate custom reports for individuals, businesses, or international clients.',
							expanded: false
						},
						{
							name: 'Audit Support',
							description: 'Support audits with comprehensive transaction histories.',
							expanded: false
						},
						{
							name: 'Documentation Automation',
							description: 'Create proper audit trails with automated tools.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'CFO',
			description:
				'Get the audit-ready data and strategic insights you need to manage Bitcoin with confidence.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Compliance & Reporting',
					features: [
						{
							name: 'Financial Integrity',
							description: 'Produce auditable data that properly reflects Bitcoin holdings.',
							expanded: false
						},
						{
							name: 'Streamlined Reporting',
							description:
								'Generate reports compliant with GAAP and accounting standards for stakeholders and regulators.',
							expanded: false
						}
					]
				},
				{
					title: 'Security & Governance',
					features: [
						{
							name: 'Data Protection',
							description: 'Enterprise-grade security with no external data sharing.',
							expanded: false
						},
						{
							name: 'Policy Framework',
							description: 'Inform robust Bitcoin policies for organizational alignment.',
							expanded: false
						},
						{
							name: 'Executive Communication',
							description: 'Present clear Bitcoin financials to boards and investors.',
							expanded: false
						}
					]
				},
				{
					title: 'Treasury Management',
					features: [
						// {
						// 	name: 'Strategic Asset Control',
						// 	description: 'Gain complete control over Bitcoin assets and allocation decisions.',
						// 	expanded: false
						// },
						{
							name: 'Performance Intelligence',
							description: 'Measure Bitcoin treasury performance with precise metrics.',
							expanded: false
						},
						{
							name: 'Balance Sheet Integration',
							description: "Understand Bitcoin's impact on financial ratios and risk exposure.",
							expanded: false
						}
					]
				},
				{
					title: 'Strategic Planning',
					features: [
						// {
						// 	name: 'Scenario Modeling',
						// 	description: 'Model different Bitcoin allocation approaches with analytical tools.',
						// 	expanded: false
						// },
						{
							name: 'Cash Flow Forecasting',
							description:
								'Forecast liquidity needs and preservation strategies across market cycles.',
							expanded: false
						},
						{
							name: 'Decision Support',
							description: 'Make confident decisions backed by comprehensive data and metrics.',
							expanded: false
						}
					]
				}
			]
		}
	];

	function toggleUserType(index: number) {
		userTypes = userTypes.map((type, i) => ({
			...type,
			expanded: i === index ? !type.expanded : false
		}));
	}

	function toggleFeature(userIndex: number, categoryIndex: number, featureIndex: number) {
		userTypes = userTypes.map((user, ui) => {
			if (ui !== userIndex) return user;

			return {
				...user,
				categories: user.categories.map((cat, ci) => {
					if (ci !== categoryIndex) return cat;

					return {
						...cat,
						features: cat.features.map((feat, fi) => {
							if (fi !== featureIndex) return feat;
							return { ...feat, expanded: !feat.expanded };
						})
					};
				})
			};
		});
	}
</script>

<section class="px-4 py-16">
	<div class="m-auto flex max-w-5xl flex-col gap-8">
		<div class="mb-8 flex flex-col gap-4">
			<h1 class="text-center text-6xl">Powerful Solutions for Real Problems</h1>
			<p class="mt-4 text-center text-2xl font-bold">
				Choose your role to learn how Clams can help you
			</p>
		</div>

		<!-- User Type Cards -->
		<div class="grid grid-cols-2 gap-4 md:grid-cols-2 lg:grid-cols-4">
			{#each userTypes as userType, userIndex}
				<button
					class="flex flex-col items-center rounded-lg border border-gray-500 p-6 transition-all duration-200 {userType.expanded
						? 'bg-[#212420] text-white'
						: 'bg-white hover:shadow-md'}"
					on:click={() => toggleUserType(userIndex)}
				>
					<div class="mb-4 flex h-16 w-16 items-center justify-center rounded-full">
						{@html userType.icon}
					</div>
					<p class="text-xl font-bold">{userType.title}</p>
				</button>
			{/each}
		</div>

		<!-- Expanded Content -->
		{#each userTypes as userType, userIndex}
			{#if userType.expanded}
				<div
					in:fade={{ duration: 200 }}
					out:fade={{ duration: 200 }}
					class="mt-8 rounded-lg border border-gray-500 bg-white p-6 shadow-sm"
				>
					<div class="mb-6">
						<p class="text-center text-4xl font-bold">{userType.title}</p>
						<p class="m-auto mt-2 max-w-3xl text-center italic">{userType.description}</p>
					</div>

					<div class="mt-8 grid gap-8 md:grid-cols-2">
						{#each userType.categories as category, categoryIndex}
							<div class="rounded-lg p-4">
								<p class="mb-4 font-bold">{category.title}</p>

								<div class="space-y-4">
									{#each category.features as feature, featureIndex}
										<div class="border-b pb-4 last:border-0 last:pb-0">
											<button
												class="flex w-full items-center justify-between text-left font-medium transition-colors hover:text-blue-600"
												on:click={() => toggleFeature(userIndex, categoryIndex, featureIndex)}
											>
												<p>{feature.name}</p>
												<svg
													class="h-5 w-5 transition-transform {feature.expanded
														? 'rotate-180'
														: ''}"
													fill="none"
													viewBox="0 0 24 24"
													stroke="currentColor"
												>
													<path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="2"
														d="M19 9l-7 7-7-7"
													/>
												</svg>
											</button>

											{#if feature.expanded}
												<p transition:slide={{ duration: 200 }} class="mt-2">
													{feature.description}
												</p>
											{/if}
										</div>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{/each}
	</div>
</section>
