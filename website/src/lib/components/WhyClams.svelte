<script lang="ts">
	import wood from '$lib/images/wood.png';
	import marble from '$lib/images/marble.png';
	import leather from '$lib/images/leather.png';
</script>

<section class="px-4 py-16">
	<div class="m-auto flex max-w-5xl flex-col gap-8">
		<div class="flex flex-col gap-4">
			<h1 class="text-center text-6xl">The Standard for Bitcoin Accounting</h1>
		</div>
		<div class="mt-8 grid gap-16">
			<div class="grid grid-cols-2 gap-8">
				<div>
					<p class="text-4xl font-bold">Privacy First</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li>All data remains local — never uploaded, tracked, or shared.</li>
						<li>Zero third-party tracking or analytics.</li>
						<li>No account or personal information required to use the app.</li>
						<li>Audit-ready without compromising confidentiality.</li>
						<li>Connect personal Electrum server for maximum sovereignty.</li>
						<li>Encrypted backups, exports and support communications.</li>
					</ul>
				</div>
				<img src={wood} alt="Wood" class="rounded-sm" />
			</div>
			<div class="grid grid-cols-2 gap-8">
				<img src={marble} alt="Marble" class="rounded-sm" />
				<div>
					<p class="text-4xl font-bold">Bitcoin Only</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li>First class support for on-chain, Lightning, and multi-signature setups.</li>
						<li>Seamless integration with most popular Bitcoin wallets.</li>
						<li>First to support new Bitcoin payment protocols and standards.</li>
						<li>Pioneering the standard for Bitcoin-native accounting.</li>
						<li>Designed for those living on a Bitcoin standard.</li>
						<li>Created by long-time Bitcoiners who understand your needs.</li>
					</ul>
				</div>
			</div>
			<div class="grid grid-cols-2 gap-8">
				<div>
					<p class="text-4xl font-bold">Elite Performance</p>
					<ul class="mt-4 flex list-disc flex-col gap-2 pl-6">
						<li>Native desktop app for lightning-fast performance.</li>
						<li>Handles millions of transactions without breaking a sweat.</li>
						<li>Instant insights with advanced caching.</li>
						<li>Automatically transforms complex Bitcoin data into actionable insights.</li>
						<li>Designed for reliability across high-volume, multi-profile environments.</li>
						<li>Robust offline capabilities for data resilience.</li>
					</ul>
				</div>
				<img src={leather} alt="Leather" class="rounded-sm" />
			</div>
		</div>
	</div>
</section>
