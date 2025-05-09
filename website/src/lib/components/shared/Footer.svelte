<script lang="ts">
	import { BLOG_URL, DISCORD_URL, DOCS_URL, GITHUB_URL, X_URL } from '$lib/constants';
	import nostrIcon from '$lib/icons/nostr.svg';
	import discordIcon from '$lib/icons/discord.svg';
	import githubIcon from '$lib/icons/github.svg';
	import xIcon from '$lib/icons/x.svg';
	import clamsSeal from '$lib/icons/clams-seal.svg';
	import clamsLogotype from '$lib/icons/clams-logotype.svg';

	let links = [
		{
			title: 'Resources',
			items: [
				{
					title: 'Blog',
					href: BLOG_URL,
					target: '_blank',
					rel: 'noopener noreferrer'
				},
				{
					title: 'Docs',
					href: DOCS_URL,
					target: '_blank',
					rel: 'noopener noreferrer'
				},
				{
					title: 'FAQ',
					href: '/faq'
				}
			]
		},
		{
			title: 'Company',
			items: [
				{ title: 'About', href: '/about', target: '', rel: '' },
				{ title: 'Brand', href: '/brand', target: '', rel: '' }
			]
		},
		{
			title: 'Legal',
			items: [
				{ title: 'Privacy Policy', href: '/privacy', target: '', rel: '' },
				{ title: 'Terms of Service', href: '/terms', target: '', rel: '' },
				{ title: 'Disclaimer', href: '/disclaimer', target: '', rel: '' }
			]
		}
	];

	let socials = [
		{
			icon: nostrIcon,
			title: 'Nostr',
			href: 'https://nostr.at/npub136hk9wu6xnrz64kfaapsvgc5rfnylz4djlx4w30w66h6cy48vhws3gth6q'
		},
		{ icon: discordIcon, title: 'Discord', href: DISCORD_URL },
		{ icon: githubIcon, title: 'GitHub', href: GITHUB_URL },
		{ icon: xIcon, title: 'X', href: X_URL }
	];

	const currentDate = new Date();
	const currentYear = currentDate.getFullYear();
</script>

<footer aria-labelledby="footer-heading" class="px-4 pt-4 pb-4">
	<div class="mt-4 mb-4 border-b"></div>
	<div class="m-auto flex max-w-7xl flex-col">
		<div class="grid gap-8 md:grid-cols-2">
			<div class="w-full">
				<img class="max-w-[200px]" src={clamsSeal} alt="Clams Seal" />
			</div>
			<div class="flex flex-col justify-between">
				<div class="flex justify-between">
					{#each links as { title, items }}
						<div>
							<p class="text-lg font-bold">{title}</p>
							<ul>
								{#each items as { title, href }}
									<li class="mt-1"><a {href} class="hover:underline">{title}</a></li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
				<div class="mt-8 flex justify-between md:mt-0">
					<div class="flex items-center gap-4">
						{#each socials as { icon, title, href }}
							<a class="flex hover:underline" {href} target="_blank" rel="noopener noreferrer">
								<img src={icon} alt={title} class="mr-2 h-6 w-6" />
							</a>
						{/each}
					</div>

					<div class="w-24">
						<img src={clamsLogotype} alt="Clams" />
					</div>
				</div>
			</div>
		</div>
	</div>
	<div class="mt-4 mb-4 border-b"></div>
	<div class="mt-4 grid md:grid-cols-2 md:gap-8">
		<div></div>
		<div>
			<p>{`${currentYear} Clams, LLC. All rights reserved.`}</p>
		</div>
	</div>
</footer>
