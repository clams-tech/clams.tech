<script lang="ts">
	import WindowsIcon from '$lib/icons/windows';
	import AppleIcon from '$lib/icons/apple';
	import LinuxIcon from '$lib/icons/linux';
	import { CRAB_NEBULA_URL } from '$lib/constants';

	let downloads = [
		{
			os: 'Silicon (dmg-aarch64)',
			href: `${CRAB_NEBULA_URL}/dmg-aarch64`,
			icon: AppleIcon
		},
		{
			os: 'Intel (dmg-x86_64)',
			href: `${CRAB_NEBULA_URL}/dmg-x86_64`,
			icon: AppleIcon
		},
		{
			os: 'Windows (wix-x86_64)',
			href: `${CRAB_NEBULA_URL}/wix-x86_64`,
			icon: WindowsIcon
		},
		{
			os: 'Windows (nsis-x86_64)',
			href: `${CRAB_NEBULA_URL}/nsis-x86_64`,
			icon: WindowsIcon
		},
		{
			os: 'Linux (appimage-x86_64)',
			href: `${CRAB_NEBULA_URL}/appimage-x86_64`,
			icon: LinuxIcon
		},
		{
			os: 'Linux (debian-x86_64)',
			href: `${CRAB_NEBULA_URL}/debian-x86_64`,
			icon: LinuxIcon
		}
	];
</script>

<!-- Downloads -->
<div class="grid gap-8 sm:grid-cols-1 md:grid-cols-2">
	{#each downloads as { os, href, icon }}
		<a
			{href}
			class="inline-flex items-center justify-center rounded-xl bg-black px-4 py-3 text-white transition-colors duration-200 hover:bg-black/90 active:bg-black/80 disabled:cursor-not-allowed disabled:opacity-50"
			type="button"
		>
			<div class="xs:w-12 mr-2 w-6">
				{@html icon}
			</div>
			{os}
		</a>
	{/each}
</div>
