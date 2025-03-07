<script lang="ts">
	import { scrollTo } from 'svelte-scrolling';
	import ClamsAltIcon from '$lib/icons/clams-alt';
	import { BLOG_URL, REMOTE_APP_URL } from '$lib/constants';

	export let isHomeRoute = true;
	export let isRemoteRoute = false;

	$: downloadHref = isRemoteRoute ? REMOTE_APP_URL : '/downloads';

	let showMobileMenu = false;
</script>

<header class="fixed left-0 top-0 z-50 w-full backdrop-blur">
	<!-- Desktop menu -->
	<nav class="flex items-center justify-between px-6 py-4 lg:px-8" aria-label="Global">
		<div class="flex lg:flex-1">
			<a href="/" class="-m-1.5 p-1.5">
				<span class="sr-only">Clams</span>
				<div class="w-10">
					{@html ClamsAltIcon}
				</div>
			</a>
		</div>
		<div class="fixed right-6 top-6 flex lg:hidden">
			<button
				on:click={() => (showMobileMenu = true)}
				type="button"
				class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700 dark:text-white"
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
		<!-- Home route -->
		{#if isHomeRoute}
			<div class="hidden lg:flex lg:gap-x-12">
				<span
					use:scrollTo={'#features'}
					class="cursor-pointer text-sm font-semibold leading-6 text-gray-900 dark:text-white"
					>Features</span
				>
				<span
					use:scrollTo={'#pricing'}
					class="cursor-pointer text-sm font-semibold leading-6 text-gray-900 dark:text-white"
					>Pricing</span
				>
				<span
					use:scrollTo={'#faq'}
					class="cursor-pointer text-sm font-semibold leading-6 text-gray-900 dark:text-white"
					>FAQ</span
				>
			</div>
		{/if}
		<!-- Not Home route && Not Remote route  -->
		{#if !isHomeRoute || isRemoteRoute}
			<div class="hidden lg:flex lg:gap-x-12">
				<a
					href="/#features"
					class="cursor-pointer text-sm font-semibold leading-6 text-gray-900 dark:text-white"
					>Features</a
				>
				<a
					href="/#pricing"
					class="cursor-pointer text-sm font-semibold leading-6 text-gray-900 dark:text-white"
					>Pricing</a
				>
				<a
					href="/#faq"
					class="cursor-pointer text-sm font-semibold leading-6 text-gray-900 dark:text-white"
					>FAQ</a
				>
			</div>
		{/if}
		<div class="hidden lg:flex">
			<a class="ml-12" href={BLOG_URL} target="_blank" rel="noopener noreferrer">
				<span class="cursor-pointer text-sm font-semibold leading-6 text-gray-900 dark:text-white"
					>Blog</span
				>
			</a>
		</div>
		<div class="hidden lg:flex lg:flex-1 lg:justify-end">
			<a
				href={downloadHref}
				target={isRemoteRoute ? '_blank' : ''}
				rel={isRemoteRoute ? 'noopener noreferrer' : ''}
				class="text-sm font-semibold leading-6 text-gray-900 dark:text-white"
				>Get Started<span aria-hidden="true">&rarr;</span></a
			>
		</div>
	</nav>
	<!-- Mobile menu, show/hide based on menu open state. -->
	<div class:hidden={!showMobileMenu} role="dialog" aria-modal="true">
		<!-- Background backdrop, show/hide based on slide-over state. -->
		<div class="fixed inset-0 z-50"></div>
		<div
			class="fixed inset-y-0 right-0 z-50 h-screen w-full overflow-y-auto bg-white px-6 py-6 dark:bg-black sm:max-w-sm sm:ring-1 sm:ring-gray-900/10"
		>
			<div class="flex items-center justify-between">
				<a on:click={() => (showMobileMenu = false)} href="/" class="-m-1.5 p-1.5">
					<span class="sr-only">Clams</span>
					<div class="w-10">
						{@html ClamsAltIcon}
					</div>
				</a>
				<button
					on:click={() => (showMobileMenu = false)}
					type="button"
					class="-m-2.5 rounded-md p-2.5 text-gray-700 dark:text-white"
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
					<!-- Home route -->
					{#if isHomeRoute}
						<div class="space-y-2 py-6">
							<a
								on:click={() => (showMobileMenu = false)}
								href="#features"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>Features</a
							>
							<a
								on:click={() => (showMobileMenu = false)}
								href="#pricing"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>Pricing</a
							>
							<a
								on:click={() => (showMobileMenu = false)}
								href="#faq"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>FAQ</a
							>
							<a
								on:click={() => (showMobileMenu = false)}
								href={BLOG_URL}
								target="_blank"
								rel="noopener noreferrer"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>Blog</a
							>
						</div>
					{/if}
					<!-- Not Home route && Not Remote route  -->
					{#if !isHomeRoute || isRemoteRoute}
						<div class="space-y-2 py-6">
							<a
								on:click={() => (showMobileMenu = false)}
								href="/#features"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>Features</a
							>
							<a
								on:click={() => (showMobileMenu = false)}
								href="/#pricing"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>Pricing</a
							>
							<a
								on:click={() => (showMobileMenu = false)}
								href="/#faq"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>FAQ</a
							>
							<a
								on:click={() => (showMobileMenu = false)}
								href={BLOG_URL}
								target="_blank"
								rel="noopener noreferrer"
								class="-mx-3 block cursor-pointer rounded-lg px-3 py-2 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
								>Blog</a
							>
						</div>
					{/if}
					<div class="py-6">
						<a
							on:click={() => (showMobileMenu = false)}
							href={downloadHref}
							target={isRemoteRoute ? '_blank' : ''}
							rel={isRemoteRoute ? 'noopener noreferrer' : ''}
							class="-mx-3 block rounded-lg px-3 py-2.5 text-base font-semibold leading-7 text-gray-900 hover:bg-gray-50 dark:text-white dark:hover:bg-gray-800"
							>Get Started</a
						>
					</div>
				</div>
			</div>
		</div>
	</div>
</header>
