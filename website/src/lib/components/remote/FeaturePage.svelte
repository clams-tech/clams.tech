<script lang="ts">
	import { REMOTE_DOCS_URL, FEATURE_ICONS, type FEATURE } from '$lib/constants';
	export let header: {
		title: FEATURE;
		subtitle: string;
	};
	export let overview: string[] = [];
	export let features: { title: string; description: string }[] = [];
	export let comingSoon: { title: string }[] | null;
	export let deepDive = '';

	import Button from '$lib/elements/Button.svelte';
	import CheckIcon from '$lib/icons/check.svelte';
	import { darkMode } from '$lib/stores';

	let isDarkMode = true;

	darkMode.subscribe((value) => {
		isDarkMode = value;
	});

	let iconColor: string;
	$: isDarkMode ? (iconColor = '#6305F0') : (iconColor = 'white');
</script>

<section class="flex flex-col items-center px-6 pb-20 pt-28">
	<div class="w-full max-w-4xl">
		<!-- Header -->
		<div class="m-auto max-w-[630px] text-center">
			<h1 class="text-[64px] font-bold">{header.title}</h1>
			<h2 class="mt-2 text-[24px]">
				{header.subtitle}
			</h2>
		</div>
		<!-- Overview -->
		<div class="mt-12 grid gap-3">
			<h2 class="text-[20px] font-bold text-light-purple dark:text-dark-purple">OVERVIEW</h2>
			{#each overview as paragraph}
				<p class="text-[18px]">{paragraph}</p>
			{/each}
		</div>
		<!-- Features -->
		<div class="mt-12 grid gap-3">
			<h2 class="text-[20px] font-bold text-light-purple dark:text-dark-purple">FEATURES</h2>
			<div class="mt-2 grid grid-cols-1 gap-6 sm:grid-cols-2">
				{#each features as { title, description }}
					<div class="flex">
						<div class="mr-1 w-8">
							<CheckIcon />
						</div>
						<div class="flex flex-col gap-1">
							<div class="flex items-center">
								<h3 class="text-[18px] font-bold">{title}</h3>
							</div>
							<p>{description}</p>
						</div>
					</div>
				{/each}
			</div>
		</div>
		<!-- Coming Soon -->
		{#if comingSoon}
			<div class="mt-12 grid gap-3">
				<h2 class="text-2xl font-bold text-light-purple dark:text-dark-purple">COMING SOON!</h2>
				<div class="mt-2 grid grid-cols-1 gap-6 sm:grid-cols-2">
					{#each comingSoon as { title }}
						<div class="flex">
							<div class="mr-1 w-8">
								<CheckIcon />
							</div>
							<div class="flex flex-col gap-1 md:max-w-[280px]">
								<div class="flex items-center">
									<h3 class="text-[18px] font-bold">{title}</h3>
								</div>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
		{#if deepDive}
			<div class="mt-[48px] flex justify-center">
				<a href={`${REMOTE_DOCS_URL}${deepDive}`} target="_blank" rel="noopener noreferrer">
					<Button primary text={`Dive deeper into ${header.title}`}>
						<div slot="iconLeft" class="xs:w-12 w-10">
							{@html `<div style="stroke: ${iconColor}">${FEATURE_ICONS[header.title]}</div>`}
						</div>
					</Button>
				</a>
			</div>
		{/if}
	</div>
</section>
