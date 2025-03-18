<script lang="ts">
	import AppScreenshotImage from '$lib/images/app-screenshot.jpeg';
	import SyncIcon from '$lib/icons/features/sync';
	import EnhanceIcon from '$lib/icons/features/enhance';
	import VisualizeIcon from '$lib/icons/features/visualize';
	import ReportsIcon from '$lib/icons/features/reports';
	import ContentContainer from '../shared/ContentContainer.svelte';

	const features = [
		{
			icon: SyncIcon,
			title: 'SYNC',
			highlights: [
				'Unlimited Wallets and Transactions.',
				'Automated Transaction Syncing.',
				'Private Local Data Store.',
				'Full Bitcoin and Lightning Support.'
			]
		},
		{
			icon: EnhanceIcon,
			title: 'ENHANCE',
			highlights: [
				'Streamlined Double Entry Accounting.',
				'Smart Transfer Matching.',
				'Wallet and Transaction Tagging.',
				'Error Detection and Reconciliation.'
			]
		},
		{
			icon: VisualizeIcon,
			title: 'VISUALIZE',
			highlights: [
				'Dynamic Sats Flow Dashboard.',
				'Comprehensive Balance Tracking.',
				'Interactive Charts for Insights.',
				'Cost Basis and Capital Gains Analysis.'
			]
		},
		{
			icon: ReportsIcon,
			title: 'REPORT',
			highlights: [
				'Cost Basis Analysis.',
				'Capital Gains Reporting.',
				'Profit and Loss Statements.',
				'Exportable Reports for Tax & Audits.'
			]
		}
	];
</script>

<ContentContainer title="Features" scrollTo="features">
	<div
		class="mx-auto grid grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-5"
	>
		<div class="lg:col-span-4 lg:pr-8">
			<div class="">
				<h2 class="text-base font-semibold leading-7 text-light-purple dark:text-dark-purple">
					Bitcoin-Only Accounting
				</h2>
				<h2 class="mt-2 text-3xl font-bold tracking-tight text-gray-900 dark:text-white">
					Built for Finance Teams
				</h2>
				<p class="m-auto mt-4 text-xl tracking-tight text-gray-900 dark:text-white">
					A complete suite for accountants and CFOs managing Bitcoin treasuries and financial
					reporting.
				</p>
				<dl
					class="mt-10 grid gap-8 text-base leading-7 text-gray-600 dark:text-gray-100 sm:grid-cols-1 md:grid-cols-2 lg:max-w-none"
				>
					{#each features as { icon, title, highlights }, i}
						<div class="relative pl-9">
							<dt class="inline font-semibold text-gray-900 dark:text-white">
								<div class="absolute left-1 top-1 h-5 w-5 text-light-purple dark:text-dark-purple">
									{@html icon}
								</div>
								<div class="flex flex-wrap">
									{title}
								</div>
							</dt>
							<dd>
								<ul class="list-inside list-disc leading-6">
									{#each highlights as highlight}
										<li>{highlight}</li>
									{/each}
								</ul>
							</dd>
						</div>
					{/each}
				</dl>
			</div>
		</div>
		<img
			src={AppScreenshotImage}
			alt="Product screenshot"
			class="w-[48rem] max-w-none overflow-hidden rounded-xl shadow-xl ring-1 ring-white/10 dark:shadow-indigo-500/50 dark:ring-white/20 sm:w-[57rem]"
			width="2432"
			height="1442"
		/>
	</div>
</ContentContainer>
