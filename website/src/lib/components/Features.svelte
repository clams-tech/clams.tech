<script lang="ts">
	import { slide } from 'svelte/transition';

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
			title: 'Individuals',
			description:
				'Tired of guessing your holdings, dreading tax season, or managing Bitcoin for your family? Clams turns stress into clarity and puts you in full control.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" /></svg>`,
			expanded: true,
			categories: [
				{
					title: 'Tax + Cost Basis',
					features: [
						{
							name: 'Automatic Purchase Tracking',
							description: 'Track every purchase with precision, automatically.',
							expanded: false
						},
						{
							name: 'Cost Basis Calculation',
							description: 'Calculate cost basis, realized gains, and losses for any transaction.',
							expanded: false
						},
						{
							name: 'Tax Prediction',
							description:
								"Understand exactly what you'll owe come tax time before you spend or sell.",
							expanded: false
						},
						{
							name: 'Privacy Protection',
							description:
								'Protect your privacy while staying tax compliant — generate reports without exposing your data.',
							expanded: false
						},
						{
							name: 'Audit-Ready Reports',
							description:
								'Generate clean, audit-ready tax reports without revealing private data.',
							expanded: false
						},
						{
							name: 'Accountant Support',
							description: 'Provide exactly what your accountant needs without the hassle.',
							expanded: false
						},
						{
							name: 'Bitcoin Activity Explanation',
							description: 'Explain your Bitcoin activity to your accountant — without panic.',
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
							name: 'Transaction Categorization',
							description: 'Identify and categorize every Bitcoin movement with ease.',
							expanded: false
						},
						{
							name: 'Unknown Transaction Detection',
							description: 'Detect unknown or forgotten transactions quickly and accurately.',
							expanded: false
						}
					]
				},
				{
					title: 'Advanced Income & Events',
					features: [
						{
							name: 'Income Tracking',
							description: 'Track Bitcoin received from mining, gifts, income, or services.',
							expanded: false
						},
						{
							name: 'Transaction Annotation',
							description: 'Tag and annotate transactions for clarity, even years later.',
							expanded: false
						},
						{
							name: 'Loss Accounting',
							description:
								'Account for lost funds, scams, and inactive wallets with clear labeling.',
							expanded: false
						},
						{
							name: 'Family Delegation Tracking',
							description: 'Track Bitcoin delegated to family or dependents, with full visibility.',
							expanded: false
						},
						{
							name: 'Special Event Handling',
							description: 'Handle tax implications of staking, lending, and Bitcoin rewards.',
							expanded: false
						}
					]
				},
				{
					title: 'Visibility + Control Over Holdings',
					features: [
						{
							name: 'Unified Dashboard',
							description: 'See everything across all wallets in one dashboard.',
							expanded: false
						},
						{
							name: 'Complete Portfolio View',
							description: 'Stop guessing — see all your Bitcoin across every wallet, instantly.',
							expanded: false
						},
						{
							name: 'Secure Family Tracking',
							description: 'Track your families holdings securely.',
							expanded: false
						},
						{
							name: 'Shared Wallet Management',
							description: 'Track family holdings — manage shared wallets with complete clarity.',
							expanded: false
						},
						{
							name: 'Goal Visualization',
							description:
								'Transform scattered Bitcoin data into clear savings goals — visualize your path to financial freedom.',
							expanded: false
						},
						{
							name: 'Savings Goal Tracking',
							description:
								'Track savings goals and calculate how much Bitcoin you need to reach them.',
							expanded: false
						},
						{
							name: 'Actionable Insights',
							description:
								'Convert confusing Bitcoin data into actionable insights — understand your actual financial position.',
							expanded: false
						},
						{
							name: 'Portfolio Performance',
							description:
								'Convert confusing Bitcoin activity into clear insights — including portfolio performance.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'Businesses',
			description:
				"Managing Bitcoin shouldn't feel like a liability. Clams gives your team real-time financial visibility, tax-ready reports, and the tools to make Bitcoin a business advantage.",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 21h19.5m-18-18v18m10.5-18v18m6-13.5V21M6.75 6.75h.75m-.75 3h.75m-.75 3h.75m3-6h.75m-.75 3h.75m-.75 3h.75M6.75 21v-3.375c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21M3 3h12m-.75 4.5H21m-3.75 3.75h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008zm0 3h.008v.008h-.008v-.008z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Accounting + Tax Compliance',
					features: [
						{
							name: 'Precise Financial Management',
							description: 'Handle Bitcoin finances with confidence and precision.',
							expanded: false
						},
						{
							name: 'Compliance Accounting',
							description:
								'Maintain accurate, compliant Bitcoin accounting for tax and reporting purposes.',
							expanded: false
						},
						{
							name: 'QuickBooks Experience',
							description: 'Get the QuickBooks-like experience designed for Bitcoin.',
							expanded: false
						},
						{
							name: 'Balance Sheet Management',
							description: 'Manage your balance sheet with Bitcoin-specific tools.',
							expanded: false
						},
						{
							name: 'Bitcoin-Specific Reporting',
							description: 'Get QuickBooks-like reporting — built specifically for Bitcoin.',
							expanded: false
						},
						{
							name: 'Investor-Ready Reports',
							description:
								'Provide clean, investor-ready reports for board meetings and stakeholders.',
							expanded: false
						}
					]
				},
				{
					title: 'Payments + Vendors',
					features: [
						{
							name: 'Streamlined Billing',
							description: 'Streamline billing and vendor transactions seamlessly.',
							expanded: false
						},
						{
							name: 'Transaction Reconciliation',
							description:
								'Easily reconcile Bitcoin payments, refunds, conversions, and operational expenses.',
							expanded: false
						},
						{
							name: 'Vendor Documentation',
							description: 'Create proper documentation for vendor and customer transactions.',
							expanded: false
						},
						{
							name: 'Payroll & Operations Support',
							description:
								'Support Bitcoin payroll, refunds, and cold storage strategies with full transparency.',
							expanded: false
						}
					]
				},
				{
					title: 'Treasury + Storage Visibility',
					features: [
						{
							name: 'Comprehensive Treasury Management',
							description:
								'Replace scattered wallet data with comprehensive treasury management — see your complete Bitcoin position.',
							expanded: false
						},
						{
							name: 'Business Use Segmentation',
							description:
								'Segment Bitcoin by business use: treasury, operations, customer funds, etc.',
							expanded: false
						},
						{
							name: 'Gains Visibility',
							description: 'View realized vs. unrealized gains at a glance.',
							expanded: false
						}
					]
				},
				{
					title: 'Strategy + Performance Clarity',
					features: [
						{
							name: 'Performance Metrics',
							description:
								'Transform Bitcoin performance uncertainty into clear metrics — know exactly how your strategy is working.',
							expanded: false
						},
						{
							name: 'Cross-Wallet Monitoring',
							description: 'Monitor Bitcoin holdings and performance across wallets and teams.',
							expanded: false
						},
						{
							name: 'Bottom Line Analysis',
							description:
								'Get clarity on whether Bitcoin is helping or hurting your bottom line — with concrete metrics.',
							expanded: false
						},
						{
							name: 'Payment Analytics',
							description: 'Track every payment with detailed analytics and trends.',
							expanded: false
						},
						{
							name: 'Revenue Insights',
							description:
								'Transform revenue questions into actionable insights — see exactly what percentage comes from Bitcoin.',
							expanded: false
						},
						{
							name: 'Bitcoin Revenue Tracking',
							description: "See how much revenue comes from Bitcoin and how often it's used.",
							expanded: false
						},
						{
							name: 'Transaction Behavior Analytics',
							description:
								'Track average transaction value, payment frequency, and customer behavior.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'Accountants',
			description:
				'No more Bitcoin headaches at tax time. Finally, a tool that makes client activity easy to track, reconcile, and explain.',
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Tax + Capital Gains',
					features: [
						{
							name: 'Complex Scenario Automation',
							description: 'Automate complex Bitcoin tax scenarios.',
							expanded: false
						},
						{
							name: 'Confident Calculations',
							description:
								'Confidently calculate cost basis and capital gains — even with partial Bitcoin sales.',
							expanded: false
						},
						{
							name: 'Accounting Standards Alignment',
							description: 'Align with the latest accounting standards like FASB effortlessly.',
							expanded: false
						}
					]
				},
				{
					title: 'Clarity + Confidence in Client Work',
					features: [
						{
							name: 'Transaction Clarity',
							description:
								'Transform client Bitcoin transactions from chaos to clarity — handle sales with confidence.',
							expanded: false
						},
						{
							name: 'Proven Methodologies',
							description:
								'Replace Bitcoin accounting uncertainty with proven methodologies — follow clear, established practices.',
							expanded: false
						},
						{
							name: 'Simplified Technical Concepts',
							description:
								'Turn technical Bitcoin concepts into familiar accounting terms — no UTXO expertise required.',
							expanded: false
						}
					]
				},
				{
					title: 'Complex Clients + Lightning',
					features: [
						{
							name: 'Lightning Node Accounting',
							description:
								'Conquer Lightning node accounting complexities — manage channels and liquidity with precision.',
							expanded: false
						},
						{
							name: 'Multi-Wallet Organization',
							description:
								'Transform multi-wallet headaches into organized client portfolios — track everything in one place.',
							expanded: false
						},
						{
							name: 'Complex Client Management',
							description:
								'Handle clients with multiple wallets, Lightning channels, and transaction types.',
							expanded: false
						},
						{
							name: 'Cross-Year Tracking',
							description: 'Track activity across fiscal years with full transaction history.',
							expanded: false
						}
					]
				},
				{
					title: 'Privacy + Audit Support',
					features: [
						{
							name: 'Comprehensive Reports',
							description: 'Generate comprehensive reports that satisfy requirements.',
							expanded: false
						},
						{
							name: 'Audit & Forensic Support',
							description: 'Support audits and forensic accounting — even for years-old activity.',
							expanded: false
						},
						{
							name: 'Private Data Control',
							description:
								'Ensure client data remains completely private — control exactly what information is shared.',
							expanded: false
						},
						{
							name: 'Seamless Documentation',
							description: 'Document transactions seamlessly.',
							expanded: false
						},
						{
							name: 'Customized Reporting',
							description:
								'Generate the right reports for individuals, businesses, or international clients.',
							expanded: false
						},
						{
							name: 'Regulatory Compliance',
							description: 'Stay compliant with FASB guidance and regulatory expectations.',
							expanded: false
						},
						{
							name: 'Complex Event Processing',
							description: 'Process forks, gifts, and multi-signature arrangements correctly.',
							expanded: false
						},
						{
							name: 'Multi-Client Efficiency',
							description:
								'Batch-process reporting and reconciliation across multiple clients efficiently.',
							expanded: false
						},
						{
							name: 'Data Verification',
							description: 'Verify client-submitted data with blockchain-aligned records.',
							expanded: false
						},
						{
							name: 'Client Confidentiality',
							description:
								'Maintain strict client confidentiality — no data ever leaves your device.',
							expanded: false
						},
						{
							name: 'Intuitive Tools',
							description: 'Get the clarity you need with intuitive tools.',
							expanded: false
						}
					]
				}
			]
		},
		{
			title: 'CFOs',
			description:
				"Bitcoin doesn't belong in a black box. From treasury to compliance, Clams delivers the audit-ready data and strategic insights you need to manage risk and report with confidence.",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-8 h-8"><path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" /></svg>`,
			expanded: false,
			categories: [
				{
					title: 'Compliance + Regulatory',
					features: [
						{
							name: 'One-Click Documentation',
							description: 'Generate regulator-ready documentation with a few clicks.',
							expanded: false
						},
						{
							name: 'Auditable Data',
							description: 'Produce auditable data that meets all requirements.',
							expanded: false
						},
						{
							name: 'Standards Compliance',
							description: 'Stay compliant with current accounting standards.',
							expanded: false
						},
						{
							name: 'SEC-Ready Reporting',
							description: 'Generate SEC- and auditor-ready reports — without breaking a sweat.',
							expanded: false
						},
						{
							name: 'GAAP Compliance',
							description: 'Comply with GAAP and impairment rules — effortlessly and accurately.',
							expanded: false
						},
						{
							name: 'Data Protection',
							description:
								'Protect sensitive data — nothing is stored in the cloud or shared externally.',
							expanded: false
						}
					]
				},
				{
					title: 'Reporting + Privacy',
					features: [
						{
							name: 'Quarterly Reporting Confidence',
							description:
								'Transform quarterly reporting stress into confidence — generate comprehensive reports automatically.',
							expanded: false
						},
						{
							name: 'Proper Holdings Reflection',
							description: 'Reflect Bitcoin holdings properly every time.',
							expanded: false
						},
						{
							name: 'Enterprise-Grade Security',
							description: 'Protect sensitive information with enterprise-grade security.',
							expanded: false
						}
					]
				},
				{
					title: 'Treasury Strategy + Allocation',
					features: [
						{
							name: 'Strategic Clarity',
							description:
								'Transform treasury management uncertainty into strategic clarity — gain complete control over Bitcoin assets.',
							expanded: false
						},
						{
							name: 'Actionable Performance Metrics',
							description:
								'Replace treasury performance doubts with actionable metrics — know exactly how your Bitcoin is performing.',
							expanded: false
						},
						{
							name: 'Precise Performance Tracking',
							description:
								'Replace performance tracking guesswork with precise metrics — measure Bitcoin treasury performance correctly.',
							expanded: false
						},
						{
							name: 'Data-Driven Strategies',
							description:
								'Transform Bitcoin allocation questions into data-driven strategies — model different approaches with ease.',
							expanded: false
						},
						{
							name: 'Real-Time Performance Reporting',
							description: 'Track and report on Bitcoin treasury performance in real time.',
							expanded: false
						},
						{
							name: 'Financial Impact Understanding',
							description:
								"Understand Bitcoin's impact on financial ratios, risk exposure, and balance sheet health.",
							expanded: false
						},
						{
							name: 'Liquidity Forecasting',
							description:
								'Forecast liquidity needs, allocations, and capital preservation across cycles.',
							expanded: false
						}
					]
				},
				{
					title: 'Internal Governance',
					features: [
						{
							name: 'Policy Development',
							description:
								'Build internal policies and governance around Bitcoin — with clarity and control.',
							expanded: false
						},
						{
							name: 'Stakeholder Presentations',
							description:
								'Present clean Bitcoin financials to boards, founders, and investors — stress-free.',
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
			<p class="mt-4 text-center text-xl">Choose your role to see how Clams can help you</p>
		</div>

		<!-- User Type Cards -->
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
			{#each userTypes as userType, userIndex}
				<button
					class="flex flex-col items-center rounded-lg border p-6 transition-all duration-200 {userType.expanded
						? 'bg-[#212420] text-white'
						: 'bg-white hover:shadow-md'}"
					on:click={() => toggleUserType(userIndex)}
				>
					<div
						class="flex h-16 w-16 items-center justify-center rounded-full {userType.expanded
							? 'bg-blue-100'
							: 'bg-gray-100'} mb-4"
					>
						{@html userType.icon}
					</div>
					<h2 class="text-xl font-bold">{userType.title}</h2>
				</button>
			{/each}
		</div>

		<!-- Expanded Content -->
		{#each userTypes as userType, userIndex}
			{#if userType.expanded}
				<div
					transition:slide={{ duration: 300 }}
					class="mt-8 rounded-lg border bg-white p-6 shadow-sm"
				>
					<div class="mb-6">
						<h2 class="text-center text-3xl font-bold">{userType.title}</h2>
						<p class="mt-2 text-center italic">{userType.description}</p>
					</div>

					<div class="mt-8 grid gap-8 md:grid-cols-2">
						{#each userType.categories as category, categoryIndex}
							<div class="rounded-lg bg-gray-50 p-6">
								<h3 class="mb-4 text-xl font-bold">{category.title}</h3>

								<div class="space-y-4">
									{#each category.features as feature, featureIndex}
										<div class="border-b pb-4 last:border-0 last:pb-0">
											<button
												class="flex w-full items-center justify-between text-left font-medium transition-colors hover:text-blue-600"
												on:click={() => toggleFeature(userIndex, categoryIndex, featureIndex)}
											>
												<span>{feature.name}</span>
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
												<div transition:slide={{ duration: 200 }} class="mt-2 text-gray-600">
													{feature.description}
												</div>
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
