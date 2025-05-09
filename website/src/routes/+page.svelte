<script>
	import clamsLogo from '$lib/icons/clams-logo.svg';
	import clamsSeal from '$lib/images/clams-seal.png';
</script>

<svelte:head>
	<title>Clams</title>
</svelte:head>

<div>
	<!-- Hero -->
	<section class="px-4 py-16">
		<div class="mt-1 mb-16 border-b"></div>
		<div class="m-auto grid max-w-7xl gap-8 md:grid-cols-2">
			<div class="flex flex-col items-center justify-center gap-8">
				<img src={clamsLogo} alt="Clams hero icon" class="w-full" />
				<div class="max-w-md text-center">
					<p class="text-2xl font-bold">Purpose Built Bitcoin Accounting</p>
					<p class="mt-4">
						Clams unifies all your Bitcoin activity into one trusted, private, best-in-class desktop
						app.
					</p>
				</div>
			</div>
			<div class="w-full overflow-hidden rounded-lg">
				<img src={clamsSeal} alt="Clams Seal" class="h-full w-full object-cover" />
			</div>
		</div>
	</section>

	<div class="mx-4 border-b"></div>

	<!-- Why Us -->
	<section class="px-4 py-16">
		<div class="m-auto flex max-w-5xl flex-col gap-8">
			<div class="flex flex-col gap-4">
				<h1 class="text-center text-6xl">The Standard for Bitcoin Accounting</h1>
			</div>
			<div class="mt-8 grid gap-8 md:grid-cols-3">
				<div class="rounded-md border bg-white p-4">
					<p class="text-center text-2xl font-bold">Privacy First</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li>All data remains local — never uploaded, tracked, or shared.</li>
						<li>Zero third-party tracking or analytics.</li>
						<li>No account or personal information required to use the app.</li>
						<li>Audit-ready without compromising confidentiality.</li>
						<li>Connect personal Electrum server for maximum sovereignty.</li>
						<li>Encrypted backups, exports and support communications.</li>
					</ul>
				</div>
				<div class="rounded-md border bg-white p-4">
					<p class="text-center text-2xl font-bold">Bitcoin Only</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li>First class support for on-chain, Lightning, and multi-signature setups.</li>
						<li>Seamless integration with most popular Bitcoin wallets.</li>
						<li>First to support new Bitcoin payment protocols and standards.</li>
						<li>Pioneering the standard for Bitcoin-native accounting.</li>
						<li>Designed for those living on a Bitcoin standard.</li>
						<li>Created by long-time Bitcoiners who understand your needs.</li>
					</ul>
				</div>
				<div class="rounded-md border bg-white p-4">
					<p class="text-center text-2xl font-bold">Elite Performance</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li>Native desktop app for lightning-fast performance.</li>
						<li>Handles millions of transactions without breaking a sweat.</li>
						<li>Instant insights with advanced caching.</li>
						<li>Automatically transforms complex Bitcoin data into actionable insights.</li>
						<li>Designed for reliability across high-volume, multi-profile environments.</li>
						<li>Robust offline capabilities for data resilience.</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<div class="mx-4 border-b"></div>

	<!-- Features -->
	<section class="px-4 py-16">
		<div class="m-auto flex max-w-5xl flex-col gap-8">
			<div class="flex flex-col gap-4">
				<h1 class="text-center text-6xl">Powerful Solutions for Real Problems</h1>
			</div>
			<div class="mt-8 grid gap-8 md:grid-cols-2">
				<!-- Individuals -->
				<div class="rounded-md border bg-white p-4">
					<p class="text-center text-2xl font-bold">Individuals</p>
					<p class="mt-2 text-center italic">
						Tired of guessing your holdings, dreading tax season, or managing Bitcoin for your
						family? Clams turns stress into clarity and puts you in full control.
					</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li class="list-none font-bold">Tax + Cost Basis</li>
						<li>Track every purchase with precision, automatically.</li>
						<li>Calculate cost basis, realized gains, and losses for any transaction.</li>
						<li>Understand exactly what you'll owe come tax time before you spend or sell.</li>
						<li>
							Protect your privacy while staying tax compliant — generate reports without exposing
							your data.
						</li>
						<li>Generate clean, audit-ready tax reports without revealing private data.</li>
						<li>Provide exactly what your accountant needs without the hassle.</li>
						<li>Explain your Bitcoin activity to your accountant — without panic.</li>

						<li class="mt-2 list-none font-bold">Spending + Transaction Clarity</li>
						<li>
							Understand when, why, and how to spend Bitcoin — including tax impact and capital
							gains.
						</li>
						<li>Identify and categorize every Bitcoin movement with ease.</li>
						<li>Detect unknown or forgotten transactions quickly and accurately.</li>

						<li class="mt-2 list-none font-bold">Advanced Income & Events</li>
						<li>Track Bitcoin received from mining, gifts, income, or services.</li>
						<li>Tag and annotate transactions for clarity, even years later.</li>
						<li>Account for lost funds, scams, and inactive wallets with clear labeling.</li>
						<li>Track Bitcoin delegated to family or dependents, with full visibility.</li>
						<li>Handle tax implications of staking, lending, and Bitcoin rewards.</li>

						<li class="mt-2 list-none font-bold">Visibility + Control Over Holdings</li>
						<li>See everything across all wallets in one dashboard.</li>
						<li>Stop guessing — see all your Bitcoin across every wallet, instantly.</li>
						<li>Track your families holdings securely.</li>
						<li>Track family holdings — manage shared wallets with complete clarity.</li>
						<li>
							Transform scattered Bitcoin data into clear savings goals — visualize your path to
							financial freedom.
						</li>
						<li>Track savings goals and calculate how much Bitcoin you need to reach them.</li>
						<li>
							Convert confusing Bitcoin data into actionable insights — understand your actual
							financial position.
						</li>
						<li>
							Convert confusing Bitcoin activity into clear insights — including portfolio
							performance.
						</li>
					</ul>
				</div>

				<!-- Businesses -->
				<div class="rounded-md border bg-white p-4">
					<p class="text-center text-2xl font-bold">Businesses</p>
					<p class="mt-2 text-center italic">
						Managing Bitcoin shouldn’t feel like a liability. Clams gives your team real-time
						financial visibility, tax-ready reports, and the tools to make Bitcoin a business
						advantage.
					</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li class="list-none font-bold">Accounting + Tax Compliance</li>
						<li>Handle Bitcoin finances with confidence and precision.</li>
						<li>Maintain accurate, compliant Bitcoin accounting for tax and reporting purposes.</li>
						<li>Get the QuickBooks-like experience designed for Bitcoin.</li>
						<li>Manage your balance sheet with Bitcoin-specific tools.</li>
						<li>Get QuickBooks-like reporting — built specifically for Bitcoin.</li>
						<li>Provide clean, investor-ready reports for board meetings and stakeholders.</li>

						<li class="mt-2 list-none font-bold">Payments + Vendors</li>
						<li>Streamline billing and vendor transactions seamlessly.</li>
						<li>
							Easily reconcile Bitcoin payments, refunds, conversions, and operational expenses.
						</li>
						<li>Create proper documentation for vendor and customer transactions.</li>
						<li>
							Support Bitcoin payroll, refunds, and cold storage strategies with full transparency.
						</li>

						<li class="mt-2 list-none font-bold">Treasury + Storage Visibility</li>
						<li>
							Replace scattered wallet data with comprehensive treasury management — see your
							complete Bitcoin position.
						</li>
						<li>Segment Bitcoin by business use: treasury, operations, customer funds, etc.</li>
						<li>View realized vs. unrealized gains at a glance.</li>

						<li class="mt-2 list-none font-bold">Strategy + Performance Clarity</li>
						<li>
							Transform Bitcoin performance uncertainty into clear metrics — know exactly how your
							strategy is working.
						</li>
						<li>Monitor Bitcoin holdings and performance across wallets and teams.</li>
						<li>
							Get clarity on whether Bitcoin is helping or hurting your bottom line — with concrete
							metrics.
						</li>
						<li>Track every payment with detailed analytics and trends.</li>
						<li>
							Transform revenue questions into actionable insights — see exactly what percentage
							comes from Bitcoin.
						</li>
						<li>See how much revenue comes from Bitcoin and how often it’s used.</li>
						<li>Track average transaction value, payment frequency, and customer behavior.</li>
					</ul>
				</div>

				<!-- Accountants -->
				<div class="rounded-md border bg-white p-4">
					<p class="text-center text-2xl font-bold">Accountants</p>
					<p class="mt-2 text-center italic">
						No more Bitcoin headaches at tax time. Finally, a tool that makes client activity easy
						to track, reconcile, and explain.
					</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li class="list-none font-bold">Tax + Capital Gains</li>
						<li>Automate complex Bitcoin tax scenarios.</li>
						<li>
							Confidently calculate cost basis and capital gains — even with partial Bitcoin sales.
						</li>
						<li>Align with the latest accounting standards like FASB effortlessly.</li>

						<li class="mt-2 list-none font-bold">Clarity + Confidence in Client Work</li>
						<li>
							Transform client Bitcoin transactions from chaos to clarity — handle sales with
							confidence.
						</li>
						<li>
							Replace Bitcoin accounting uncertainty with proven methodologies — follow clear,
							established practices.
						</li>
						<li>
							Turn technical Bitcoin concepts into familiar accounting terms — no UTXO expertise
							required.
						</li>

						<li class="mt-2 list-none font-bold">Complex Clients + Lightning</li>
						<li>
							Conquer Lightning node accounting complexities — manage channels and liquidity with
							precision.
						</li>
						<li>
							Transform multi-wallet headaches into organized client portfolios — track everything
							in one place.
						</li>
						<li>
							Handle clients with multiple wallets, Lightning channels, and transaction types.
						</li>
						<li>Track activity across fiscal years with full transaction history.</li>

						<li class="mt-2 list-none font-bold">Privacy + Audit Support</li>
						<li>Generate comprehensive reports that satisfy requirements.</li>
						<li>Support audits and forensic accounting — even for years-old activity.</li>
						<li>
							Ensure client data remains completely private — control exactly what information is
							shared.
						</li>
						<li>Document transactions seamlessly.</li>
						<li>
							Generate the right reports for individuals, businesses, or international clients.
						</li>
						<li>Stay compliant with FASB guidance and regulatory expectations.</li>
						<li>Process forks, gifts, and multi-signature arrangements correctly.</li>
						<li>Batch-process reporting and reconciliation across multiple clients efficiently.</li>
						<li>Verify client-submitted data with blockchain-aligned records.</li>
						<li>Maintain strict client confidentiality — no data ever leaves your device.</li>
						<li>Get the clarity you need with intuitive tools.</li>
					</ul>
				</div>

				<!-- CFOs -->
				<div class="rounded-md border bg-white p-4">
					<p class="text-center text-2xl font-bold">CFOs</p>
					<p class="mt-2 text-center italic">
						Bitcoin doesn’t belong in a black box. From treasury to compliance, Clams delivers the
						audit-ready data and strategic insights you need to manage risk and report with
						confidence.
					</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li class="list-none font-bold">Compliance + Regulatory</li>
						<li>Generate regulator-ready documentation with a few clicks.</li>
						<li>Produce auditable data that meets all requirements.</li>
						<li>Stay compliant with current accounting standards.</li>
						<li>Generate SEC- and auditor-ready reports — without breaking a sweat.</li>
						<li>Comply with GAAP and impairment rules — effortlessly and accurately.</li>
						<li>Protect sensitive data — nothing is stored in the cloud or shared externally.</li>

						<li class="mt-2 list-none font-bold">Reporting + Privacy</li>
						<li>
							Transform quarterly reporting stress into confidence — generate comprehensive reports
							automatically.
						</li>
						<li>Reflect Bitcoin holdings properly every time.</li>
						<li>Protect sensitive information with enterprise-grade security.</li>

						<li class="mt-2 list-none font-bold">Treasury Strategy + Allocation</li>
						<li>
							Transform treasury management uncertainty into strategic clarity — gain complete
							control over Bitcoin assets.
						</li>
						<li>
							Replace treasury performance doubts with actionable metrics — know exactly how your
							Bitcoin is performing.
						</li>
						<li>
							Replace performance tracking guesswork with precise metrics — measure Bitcoin treasury
							performance correctly.
						</li>
						<li>
							Transform Bitcoin allocation questions into data-driven strategies — model different
							approaches with ease.
						</li>
						<li>Track and report on Bitcoin treasury performance in real time.</li>
						<li>
							Understand Bitcoin’s impact on financial ratios, risk exposure, and balance sheet
							health.
						</li>
						<li>Forecast liquidity needs, allocations, and capital preservation across cycles.</li>

						<li class="mt-2 list-none font-bold">Internal Governance</li>
						<li>
							Build internal policies and governance around Bitcoin — with clarity and control.
						</li>
						<li>
							Present clean Bitcoin financials to boards, founders, and investors — stress-free.
						</li>
					</ul>
				</div>
			</div>
		</div>
	</section>
</div>
