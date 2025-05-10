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
				'Tired of guessing your holdings, dreading tax season, or managing Bitcoin for your family? Clams turns stress into clarity and puts you in full control.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" /></svg>`,
			expanded: true,
			categories: [
				{
					title: 'Tax + Cost Basis',
					features: [
						{
							name: 'Automatic Purchase & Cost Basis Tracking',
							description:
								'Track every purchase with precision and automatically calculate cost basis, realized gains, and losses for any transaction.',
							expanded: false
						},
						{
							name: 'Tax Prediction & Planning',
							description:
								"Understand exactly what you'll owe come tax time before you spend or sell.",
							expanded: false
						},
						{
							name: 'Privacy-Preserving Reports',
							description:
								'Generate clean, audit-ready tax reports while protecting your privacy and without exposing sensitive data.',
							expanded: false
						},
						{
							name: 'Accountant-Friendly Interface',
							description:
								'Provide exactly what your accountant needs and explain your Bitcoin activity without the panic or hassle.',
							expanded: false
						}
					]
				},
				{
					title: 'Spending + Transaction Clarity',
					features: [
						{
							name: 'Informed Spending',
							description:
								'Understand when, why, and how to spend Bitcoin — including tax impact and capital gains.',
							expanded: false
						},
						{
							name: 'Transaction Management',
							description:
								'Identify, categorize, and detect unknown or forgotten Bitcoin movements with ease and accuracy.',
							expanded: false
						}
					]
				},
				{
					title: 'Advanced Income & Events',
					features: [
						{
							name: 'Comprehensive Income Tracking',
							description:
								'Track Bitcoin received from mining, gifts, income, or services with detailed annotations.',
							expanded: false
						},
						{
							name: 'Special Situation Handling',
							description:
								'Account for lost funds, scams, inactive wallets, and tax implications of staking, lending, and rewards with clear labeling.',
							expanded: false
						},
						{
							name: 'Family Bitcoin Management',
							description:
								'Track Bitcoin delegated to family or dependents with full visibility and control.',
							expanded: false
						}
					]
				},
				{
					title: 'Visibility + Control Over Holdings',
					features: [
						{
							name: 'Unified Portfolio Management',
							description:
								'See everything across all wallets in one dashboard—no more guessing about your complete Bitcoin position.',
							expanded: false
						},
						{
							name: 'Family & Shared Wallet Visibility',
							description:
								'Track family holdings securely with complete clarity across shared wallets.',
							expanded: false
						},
						{
							name: 'Goal Setting & Tracking',
							description:
								'Transform scattered Bitcoin data into clear savings goals and calculate how much Bitcoin you need to reach financial freedom.',
							expanded: false
						},
						{
							name: 'Performance Insights',
							description:
								'Convert confusing Bitcoin data into actionable insights that help you understand your actual financial position and portfolio performance.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'Business Owner',
			description:
				"Managing Bitcoin shouldn't feel like a liability. Clams gives your team real-time financial visibility, tax-ready reports, and the tools to make Bitcoin a business advantage.",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 21h19.5m-18-18v18m10.5-18v18m6-13.5V21M6.75 6.75h.75m-.75 3h.75m-.75 3h.75m3-6h.75m-.75 3h.75m-.75 3h.75M6.75 21v-3.375c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21M3 3h12m-.75 4.5H21m-3.75 3.75h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Accounting + Tax Compliance',
					features: [
						{
							name: 'Bitcoin-Optimized Accounting System',
							description:
								'Handle Bitcoin finances with confidence using a QuickBooks-like experience designed specifically for Bitcoin accounting and reporting.',
							expanded: false
						},
						{
							name: 'Compliant Financial Management',
							description:
								'Maintain accurate, compliant Bitcoin accounting for tax and reporting purposes with Bitcoin-specific tools for balance sheet management.',
							expanded: false
						},
						{
							name: 'Stakeholder & Reporting Tools',
							description:
								'Provide clean, investor-ready reports for board meetings, stakeholders, and regulatory compliance.',
							expanded: false
						}
					]
				},
				{
					title: 'Payments + Operations',
					features: [
						{
							name: 'Comprehensive Transaction Management',
							description:
								'Streamline billing and reconcile Bitcoin payments, refunds, conversions, and operational expenses seamlessly.',
							expanded: false
						},
						{
							name: 'Business Documentation Suite',
							description:
								'Create proper documentation for vendor and customer transactions with automated record-keeping.',
							expanded: false
						},
						{
							name: 'Operational Integration',
							description:
								'Support Bitcoin payroll, refunds, and cold storage strategies with full transparency and operational controls.',
							expanded: false
						}
					]
				},
				{
					title: 'Treasury Management',
					features: [
						{
							name: 'Unified Bitcoin Position',
							description:
								'Replace scattered wallet data with comprehensive treasury management—see your complete Bitcoin position with realized vs. unrealized gains at a glance.',
							expanded: false
						},
						{
							name: 'Strategic Fund Allocation',
							description:
								'Segment Bitcoin by business use categories such as treasury, operations, customer funds, and more for proper financial management.',
							expanded: false
						}
					]
				},
				{
					title: 'Performance + Analytics',
					features: [
						{
							name: 'Strategic Performance Dashboard',
							description:
								'Transform Bitcoin performance uncertainty into clear metrics—monitor holdings and performance across wallets and teams to know exactly how your strategy is working.',
							expanded: false
						},
						{
							name: 'Business Impact Analysis',
							description:
								'Get clarity on whether Bitcoin is helping or hurting your bottom line with concrete metrics and actionable insights.',
							expanded: false
						},
						{
							name: 'Revenue Intelligence',
							description:
								"See exactly what percentage of revenue comes from Bitcoin, how often it's used, and transform revenue questions into actionable business intelligence.",
							expanded: false
						},
						{
							name: 'Customer Behavior Insights',
							description:
								'Track average transaction value, payment frequency, and customer behavior with detailed payment analytics and trends.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'Accountant',
			description:
				'No more Bitcoin headaches at tax time. Finally, a tool that makes client activity easy to track, reconcile, and explain.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Tax + Regulatory Compliance',
					features: [
						{
							name: 'Comprehensive Tax Automation',
							description:
								'Automate complex Bitcoin tax scenarios and confidently calculate cost basis and capital gains—even with partial Bitcoin sales.',
							expanded: false
						},
						{
							name: 'Standards Compliance',
							description:
								'Align with the latest accounting standards like FASB effortlessly and stay current with regulatory expectations.',
							expanded: false
						},
						{
							name: 'Complex Event Handling',
							description:
								'Process forks, gifts, multi-signature arrangements, and other complex Bitcoin events correctly and efficiently.',
							expanded: false
						}
					]
				},
				{
					title: 'Client Service Excellence',
					features: [
						{
							name: 'Bitcoin Translation Layer',
							description:
								'Transform client Bitcoin transactions from chaos to clarity and turn technical Bitcoin concepts into familiar accounting terms—no UTXO expertise required.',
							expanded: false
						},
						{
							name: 'Methodical Approach',
							description:
								'Replace Bitcoin accounting uncertainty with proven methodologies—follow clear, established practices with confidence.',
							expanded: false
						},
						{
							name: 'Client Confidentiality',
							description:
								'Maintain strict client confidentiality with private data controls—no data ever leaves your device while still generating necessary reports.',
							expanded: false
						}
					]
				},
				{
					title: 'Advanced Client Management',
					features: [
						{
							name: 'Specialized Bitcoin Infrastructure',
							description:
								'Conquer Lightning node accounting complexities and handle clients with multiple wallets, Lightning channels, and various transaction types.',
							expanded: false
						},
						{
							name: 'Portfolio Organization',
							description:
								'Transform multi-wallet headaches into organized client portfolios—track everything in one place with intuitive tools and interfaces.',
							expanded: false
						},
						{
							name: 'Longitudinal Record-Keeping',
							description:
								'Track activity across fiscal years with full transaction history and efficient data verification against blockchain records.',
							expanded: false
						},
						{
							name: 'Multi-Client Efficiency',
							description:
								'Batch-process reporting and reconciliation across multiple clients efficiently while maintaining individual client attention.',
							expanded: false
						}
					]
				},
				{
					title: 'Reporting + Audit Readiness',
					features: [
						{
							name: 'Client-Tailored Reporting',
							description:
								'Generate the right reports for individuals, businesses, or international clients with customized reporting options.',
							expanded: false
						},
						{
							name: 'Audit Support System',
							description:
								'Support audits and forensic accounting—even for years-old activity—with comprehensive, well-documented transaction histories.',
							expanded: false
						},
						{
							name: 'Documentation Automation',
							description:
								'Document transactions seamlessly with automated tools that create proper audit trails and verification mechanisms.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'CFO',
			description:
				"Bitcoin doesn't belong in a black box. From treasury to compliance, Clams delivers the audit-ready data and strategic insights you need to manage risk and report with confidence.",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Compliance & Reporting',
					features: [
						{
							name: 'Regulatory Documentation Suite',
							description:
								'Generate SEC- and auditor-ready reports with a few clicks while staying compliant with GAAP, impairment rules, and current accounting standards.',
							expanded: false
						},
						{
							name: 'Financial Integrity Assurance',
							description:
								'Produce auditable data that meets all requirements and reflects Bitcoin holdings properly in every financial statement.',
							expanded: false
						},
						{
							name: 'Streamlined Reporting Cycle',
							description:
								'Transform quarterly reporting stress into confidence with automatically generated comprehensive reports that satisfy stakeholder and regulatory requirements.',
							expanded: false
						}
					]
				},
				{
					title: 'Security & Governance',
					features: [
						{
							name: 'Enterprise Data Protection',
							description:
								'Protect sensitive information with enterprise-grade security—nothing is stored in the cloud or shared externally without explicit authorization.',
							expanded: false
						},
						{
							name: 'Policy Framework',
							description:
								'Build robust internal policies and governance around Bitcoin with clarity and control for consistent organizational alignment.',
							expanded: false
						},
						{
							name: 'Executive Communication',
							description:
								'Present clean Bitcoin financials to boards, founders, and investors with stress-free, professional presentations and reports.',
							expanded: false
						}
					]
				},
				{
					title: 'Treasury Management',
					features: [
						{
							name: 'Strategic Asset Control',
							description:
								'Transform treasury management uncertainty into strategic clarity with complete control over Bitcoin assets and allocation decisions.',
							expanded: false
						},
						{
							name: 'Performance Intelligence',
							description:
								'Replace treasury performance doubts with precise, actionable metrics that measure Bitcoin treasury performance correctly in real time.',
							expanded: false
						},
						{
							name: 'Balance Sheet Integration',
							description:
								"Understand Bitcoin's impact on financial ratios, risk exposure, and overall balance sheet health with sophisticated financial modeling.",
							expanded: false
						}
					]
				},
				{
					title: 'Strategic Planning',
					features: [
						{
							name: 'Scenario Modeling',
							description:
								'Transform Bitcoin allocation questions into data-driven strategies by modeling different approaches with sophisticated analytical tools.',
							expanded: false
						},
						{
							name: 'Cash Flow Forecasting',
							description:
								'Forecast liquidity needs, optimal allocations, and capital preservation strategies across market cycles with predictive analytics.',
							expanded: false
						},
						{
							name: 'Strategic Decision Support',
							description:
								'Make confident Bitcoin treasury decisions backed by comprehensive data, clear metrics, and projected financial outcomes.',
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
					class="flex flex-col items-center rounded-lg border p-6 transition-all duration-200 {userType.expanded
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
					class="mt-8 rounded-lg border bg-white p-6 shadow-sm"
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
