<script lang="ts">
	import { BLOG_URL, DISCORD_URL, DOCS_URL, GITHUB_URL, X_URL } from '$lib/constants';

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
				{ title: 'Contact', href: '/contact', target: '', rel: '' },
				{ title: 'Brand Assets', href: '/brand-assets', target: '', rel: '' }
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
			icon: 'src/lib/icons/nostr.svg',
			title: 'Nostr',
			href: 'https://nostr.at/npub136hk9wu6xnrz64kfaapsvgc5rfnylz4djlx4w30w66h6cy48vhws3gth6q'
		},
		{ icon: 'src/lib/icons/discord.svg', title: 'Discord', href: DISCORD_URL },
		{ icon: 'src/lib/icons/github.svg', title: 'GitHub', href: GITHUB_URL },
		{ icon: 'src/lib/icons/x.svg', title: 'X', href: X_URL }
	];

	const currentDate = new Date();
	const currentYear = currentDate.getFullYear();
</script>

<footer aria-labelledby="footer-heading" class="px-4 pb-4 pt-16">
	<div class="mx-auto max-w-5xl">
		<div class="grid grid-cols-2 gap-8">
			<div class="w-full max-w-[200px]">
				<img src="src/lib/icons/clams-seal.svg" alt="Clams Seal" />
			</div>
			<div class=" flex flex-col justify-between">
				<div class="flex justify-between">
					{#each links as { title, items }}
						<div>
							<p class="text-lg font-bold">{title}</p>
							<ul>
								{#each items as { title, href }}
									<li><a {href} class="hover:underline">{title}</a></li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
				<div class="flex justify-between">
					<div class="flex items-center gap-4">
						{#each socials as { icon, title, href }}
							<a class="flex hover:underline" {href} target="_blank" rel="noopener noreferrer">
								<img src={icon} alt={title} class="mr-2 h-6 w-6" />
							</a>
						{/each}
					</div>

					<div class="w-24">
						<img src="/src/lib/icons/clams-logotype.svg" alt="Clams" />
					</div>
				</div>
			</div>
		</div>
	</div>
	<p class="mt-16 text-center">{`${currentYear} Clams, LLC. All rights reserved.`}</p>
</footer>
