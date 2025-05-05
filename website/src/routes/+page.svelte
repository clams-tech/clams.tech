<svelte:head>
	<title>Clams</title>
</svelte:head>

<div>
	<!-- Hero -->
	<section class="py-22 flex h-screen flex-col items-center justify-center px-4">
		<div class="grid grid-cols-2 gap-8">
			<div class="flex flex-col items-center justify-center">
				<img src="src/lib/icons/clams-lockup.svg" alt="Clams hero icon" class="w-full" />
				<p class="mt-4 text-2xl">Purpose Built Bitcoin Accounting</p>
			</div>
			<div class="w-full overflow-hidden rounded-lg">
				<img
					src="src/lib/images/clams-seal.png"
					alt="Clams Seal"
					class="h-full w-full object-cover"
				/>
			</div>
		</div>
	</section>

	<div class="mx-4 border-b"></div>

	<!-- Why Us -->
	<section class="px-4 py-16">
		<div class="m-auto grid max-w-5xl grid-cols-2 gap-8">
			<div class="flex flex-col gap-4">
				<h1 class="text-6xl">The Standard for Bitcoin Accounting</h1>
				<p>
					Clams unifies all your Bitcoin transactions into one trusted, private, best-in-class
					desktop app.
				</p>
				<!-- <p class="text-2xl">Guiding</p> -->
				<ul class="flex list-disc flex-col gap-2 pl-6">
					<li>
						<strong>Privacy First</strong>: Your data stays local, never shared, ensuring
						sovereignty and trust.
					</li>
					<li>
						<strong>Bitcoin Only</strong>: First class support for onchain, multisig, and Lightning
						transactions. And early support for any new form that Bitcoin takes.
					</li>

					<li>
						<strong>High Performance </strong>: Native desktop app built to handle millions of
						transactions. Reliable, secure and robust. Accurate, data integrity.
					</li>
				</ul>
			</div>
			<div class="w-full overflow-hidden rounded-lg">
				<img
					src="src/lib/images/clams-connections.png"
					alt="Clams Seal"
					class="h-full w-full object-cover"
				/>
			</div>
		</div>
	</section>

	<!-- FAQ -->
	<!-- <section class="px-4 py-16">
		<div class="m-auto flex max-w-5xl flex-col gap-4">
			<h1 class="text-6xl">Frequently Asked Questions</h1>
			<div class="flex flex-col gap-4">
				<div>
					<p class="font-bold">Which wallet types does Clams support?</p>
					<p>
						Clams supports all major Bitcoin wallet types, including onchain, multisig, Lightning,
						and exchanges. Visit our
						<a class="underline" href={DOCS_URL} target="_blank" rel="noopener noreferrer">docs</a>
						to learn more or
						<a class="underline" href="discord" target="_blank" rel="noopener noreferrer"
							>join our Discord</a
						> to share your needs and stay updated.
					</p>
				</div>
				<div>
					<p class="font-bold">Where is my data stored?</p>
					<p>
						All data is stored locally on your device, ensuring complete privacy and sovereignty.
					</p>
				</div>

				<div>
					<p class="font-bold">When will Clams include Reports / Exports features?</p>
					<p>
						We’re developing an enhanced paid version (cheers to our beta testers for feedback) with
						advanced export capabilities. Stay tuned for updates via
						<a class="underline" target="_blank" rel="noopener noreferrer" href={DISCORD_URL}
							>our Discord</a
						>.
					</p>
				</div>
				<div>
					<Button href="/faq">View all FAQs</Button>
				</div>
			</div>
		</div>
	</section> -->

	<div class="mx-4 border-b"></div>

	<!-- CTA -->
	<!-- <section class="px-4 py-16">
		<div class="m-auto grid max-w-6xl grid-cols-2 gap-8">
			<div class="w-full overflow-hidden rounded-lg">
				<img
					src="src/lib/images/clams-seal.png"
					alt="Clams Seal"
					class="h-full w-full object-cover"
				/>
			</div>
			<div class="flex flex-col gap-4">
				<h1 class="text-6xl">Get Started</h1>
				<p>
					Download Clams to eliminate your Bitcoin accounting headaches. Core features are free,
					with advanced features coming soon as part of a paid version.
				</p>
				<p>Connect with our community to shape the future of Bitcoin accounting.</p>
				<div class="flex gap-4">
					<Button href="/downloads">Download Now</Button>
					<Button variant="green" target="_blank" rel="noopener noreferrer" href={DISCORD_URL}
						>Join our Community</Button
					>
				</div>
			</div>
		</div>
	</section> -->
</div>
