<script lang="ts">
	import WalletIcon from '$lib/icons/wallet.js';
	import KeysIcon from '$lib/icons/keys.js';
	import LightningIcon from '$lib/icons/lightning-outline.js';
	import { darkMode } from '$lib/stores';

	let isDarkMode = true;

	darkMode.subscribe((value) => {
		isDarkMode = value;
	});

	let iconColor: string;
	$: isDarkMode ? (iconColor = 'white') : (iconColor = 'black');
</script>

<section class="mx-auto max-w-5xl lg:px-6">
	<h2 class="text-center text-4xl font-bold">Who is it for?</h2>

	<div class="mt-12 grid grid-cols-1 gap-8 md:grid-cols-3">
		<!-- Wallets -->
		<div class="rounded-lg border p-6 text-center shadow-md">
			<div class="m-auto w-16">
				{@html `<div style="stroke: ${iconColor}">${WalletIcon}</div>`}
			</div>
			<h3 class="mt-4 text-2xl font-semibold">Wallets</h3>
			<p class="mt-2 text-gray-300">Offer automated accounting without leaving your app.</p>
		</div>

		<!-- Exchanges -->
		<div class="rounded-lg border p-6 text-center shadow-md">
			<div class="m-auto w-16">
				{@html `<div style="stroke: ${iconColor}">${KeysIcon}</div>`}
			</div>
			<h3 class="mt-4 text-2xl font-semibold">Exchanges</h3>
			<p class="mt-2 text-gray-300">Simplify tax season with capital gains reports.</p>
		</div>

		<!-- Payment Platforms -->
		<div class="rounded-lg border p-6 text-center shadow-md">
			<div class="m-auto w-16">
				{@html `<div style="stroke: ${iconColor}">${LightningIcon}</div>`}
			</div>
			<h3 class="mt-4 text-2xl font-semibold">Payment Platforms</h3>
			<p class="mt-2 text-gray-300">Empower merchants with profit and loss statements.</p>
		</div>
	</div>
</section>
