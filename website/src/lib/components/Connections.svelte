<script lang="ts">
	import { DISCORD_URL } from '$lib/constants';
	import CoreLightningIcon from '$lib/icons/connections/core-lightning';
	import LndIcon from '$lib/icons/connections/lnd';
	import NwcIcon from '$lib/icons/connections/nwc';
	import OnchainIcon from '$lib/icons/connections/onchain';
	import PhoenixIcon from '$lib/icons/connections/phoenix';
	import RiverIcon from '$lib/icons/connections/river';
	import StrikeIcon from '$lib/icons/connections/strike';
	import UnchainedIcon from '$lib/icons/connections/unchained';
	import WalletOfSatoshiIcon from '$lib/icons/connections/walletofsatoshi';
	import ContentContainer from './shared/ContentContainer.svelte';
	import CashuIcon from '$lib/icons/connections/cashu';
	import CustomImportIcon from '$lib/icons/connections/custom-import';

	const connections = [
		{
			name: 'xPub / Multisig',
			icon: OnchainIcon,
			link: 'https://bitcoin.org/en/'
		},
		{
			name: 'Core Lightning',
			icon: CoreLightningIcon,
			link: 'https://www.youtube.com/watch?v=bduhordWO7I'
		},
		{
			name: 'LND',
			icon: LndIcon,
			link: 'https://www.youtube.com/watch?v=r8JC4gNlObY'
		},
		{
			name: 'Nostr Wallet Connect',
			icon: NwcIcon,
			link: 'https://nwc.dev/'
		},
		{
			name: 'Cashu',
			icon: CashuIcon,
			link: 'https://blog.clams.tech/release-v0-3-0/#cashu-nip60-connection'
		},
		{
			name: 'Custom Import',
			icon: CustomImportIcon,
			link: 'https://blog.clams.tech/release-v0-3-0/#custom-imports'
		}
	];

	const comingSoon = [
		{
			name: 'Phoenix',
			icon: PhoenixIcon,
			link: 'https://phoenix.acinq.co/'
		},
		{
			name: 'Wallet of Satoshi',
			icon: WalletOfSatoshiIcon,
			link: 'https://www.walletofsatoshi.com/'
		},
		{
			name: 'Unchained',
			icon: UnchainedIcon,
			link: 'https://unchained.com/'
		},
		{
			name: 'Strike',
			icon: StrikeIcon,
			link: 'https://strike.me/'
		},
		{
			name: 'River',
			icon: RiverIcon,
			link: 'https://river.com/'
		}
	];
</script>

<ContentContainer title="Fully integrated wallets">
	<div class="text-center">
		<p class="mt-8 text-lg leading-8 text-gray-600 dark:text-gray-100 md:mt-12">
			These wallets are fully integrated into our system to streamline the data import.
		</p>
		<div class="mt-8 flex flex-wrap justify-center gap-8">
			{#each connections as { name, icon, link }}
				<a class="flex-col justify-between" href={link} target="_blank" rel="noopener noreferrer">
					<div
						class="m-auto flex h-28 w-28 w-32 flex-shrink-0 items-center justify-center rounded-full p-3"
					>
						{@html icon}
					</div>
					<p class="text-center">{name}</p>
				</a>
			{/each}
		</div>
		<p class="mt-8 text-lg leading-8 text-gray-600 dark:text-gray-100 md:mt-12">
			These wallets are coming soon. Let us know and we'll prioritize your favorite.
		</p>
		<div class="mt-8 flex flex-wrap justify-center gap-8">
			{#each comingSoon as { name, icon, link }}
				<a
					class="w-full max-w-[100px] flex-col opacity-[50%]"
					href={link}
					target="_blank"
					rel="noopener noreferrer"
				>
					<div class="m-auto w-16 p-3">
						{@html icon}
					</div>
					<p class="text-center">{name}</p>
				</a>
			{/each}
		</div>
		<div class="wrap mt-12 flex items-center gap-x-6">
			<a
				href={DISCORD_URL}
				target="_blank"
				rel="noopener noreferrer"
				class="text-md mx-auto block rounded-md bg-indigo-600 px-3 py-2 text-center font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
				>Join our Discord</a
			>
		</div>
	</div>
</ContentContainer>
