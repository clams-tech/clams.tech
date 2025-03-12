<script lang="ts">
	import ClamsIcon from '$lib/icons/clams';

	const sections = [
		{
			href: '/app',
			title: 'Individuals & Businesses',
			subtitle: 'Desktop Application',
			description:
				'Helping you manage life on a Bitcoin standard. Connect your wallets, consolidate transactions, and generate reports—effortlessly.'
		},
		{
			href: '/api',
			title: 'Founders & Developers',
			subtitle: 'API',
			description:
				'Bitcoin accounting for your application. Seamlessly integrate Bitcoin accounting to automate transaction tracking, gain real-time financial insights, and simplify bookkeeping.'
		}
	];
</script>

<div class="flex h-screen w-screen items-center justify-center p-6">
	<div class="flex max-w-3xl flex-col gap-8">
		<div class="ml-auto mr-auto w-64">
			{@html ClamsIcon}
		</div>
		<div class="grid grid-cols-1 gap-8 md:grid-cols-2">
			{#each sections as { href, title, subtitle, description }}
				<a {href} class="rounded-lg border p-4">
					<div class="group text-center">
						<h1 class="text-3xl font-bold group-hover:underline md:text-4xl">{title}</h1>
						<p
							class="mt-2 text-2xl font-bold text-light-purple hover:text-indigo-500 group-hover:underline dark:text-dark-purple"
						>
							{subtitle}
						</p>
						<p class="mt-4">{description}</p>
					</div>
				</a>
			{/each}
		</div>
	</div>
</div>
