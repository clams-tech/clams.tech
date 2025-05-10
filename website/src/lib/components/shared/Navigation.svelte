<script lang="ts">
	import { BLOG_URL, DOCS_URL } from '$lib/constants';
	import clamsLogomark from '$lib/icons/clams-logomark.svg';
	import Button from '$lib/elements/Button.svelte';
	import { fly, fade } from 'svelte/transition';

	let showMobileMenu = false;
</script>

<header class="fixed top-0 left-0 z-50 w-full">
	<!-- Desktop menu -->
	<nav class="flex items-center justify-between px-4 py-4" aria-label="Global">
		<div class="flex lg:flex-1">
			<a href="/" class="-m-1.5 p-1.5">
				<span class="sr-only">Clams</span>
				<div class="w-10">
					<img src={clamsLogomark} alt="Clams Logomark" />
				</div>
			</a>
		</div>
		<div class="fixed top-6 right-6 flex lg:hidden">
			<button
				on:click={() => (showMobileMenu = true)}
				type="button"
				class="-m-2.5 inline-flex items-center justify-center rounded-md"
			>
				<span class="sr-only">Open main menu</span>
				<svg
					class="h-6 w-6"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					aria-hidden="true"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
					/>
				</svg>
			</button>
		</div>
		<div class="hidden items-center lg:flex lg:gap-x-12">
			<a
				class="cursor-pointer text-sm leading-6 font-semibold"
				href={BLOG_URL}
				target="_blank"
				rel="noopener noreferrer"
			>
				Blog
			</a>
			<a href={DOCS_URL} class="cursor-pointer text-sm leading-6 font-semibold">Docs</a>
			<Button href="/downloads" variant="green">Download</Button>
		</div>
	</nav>
	<!-- Mobile menu, show/hide based on menu open state. -->
	{#if showMobileMenu}
		<!-- Background backdrop with fade transition -->
		<div
			transition:fade={{ duration: 200 }}
			class="fixed inset-0 z-50 bg-black opacity-50"
			on:click={() => (showMobileMenu = false)}
			on:keydown={(e) => {
				if (e.key === 'Escape') {
					showMobileMenu = false;
				}
			}}
			role="button"
			tabindex="0"
			aria-label="Close menu"
		></div>

		<!-- Mobile menu with fly transition -->
		<div
			transition:fly={{ x: 300, duration: 300, opacity: 1 }}
			class="fixed inset-y-0 right-0 z-50 h-screen w-full overflow-y-auto bg-white px-4 py-4 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10"
		>
			<div class="flex items-center justify-between">
				<a on:click={() => (showMobileMenu = false)} href="/" class="-m-1.5 p-1.5">
					<span class="sr-only">Clams</span>
					<div class="w-10">
						<img src={clamsLogomark} alt="Clams Logo" />
					</div>
				</a>
				<button
					on:click={() => (showMobileMenu = false)}
					type="button"
					class="-m-2.5 rounded-md p-2.5"
				>
					<span class="sr-only">Close menu</span>
					<svg
						class="h-6 w-6"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						aria-hidden="true"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>
			<div class="mt-6 flow-root">
				<div class="-my-6 divide-y divide-gray-500/10">
					<div class="space-y-2 py-6">
						<a
							on:click={() => (showMobileMenu = false)}
							href={BLOG_URL}
							target="_blank"
							rel="noopener noreferrer"
							class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 leading-7 font-semibold hover:bg-gray-50"
							>Blog</a
						>
						<a
							on:click={() => (showMobileMenu = false)}
							href={DOCS_URL}
							target="_blank"
							rel="noopener noreferrer"
							class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 leading-7 font-semibold hover:bg-gray-50"
							>Docs</a
						>
						<button
							type="button"
							class="w-full border-0 bg-transparent p-0 text-left"
							on:click={() => (showMobileMenu = false)}
						>
							<Button href="/downloads" variant="green">Download</Button>
						</button>
					</div>
				</div>
			</div>
		</div>
	{/if}
</header>
