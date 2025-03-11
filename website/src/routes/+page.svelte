<script lang="ts">
	import ClamsIcon from '$lib/icons/clams';
</script>

<div class="flex h-screen w-screen items-center justify-center p-6">
	<div class="flex max-w-3xl flex-col gap-8">
		<div class="ml-auto mr-auto w-64">
			{@html ClamsIcon}
		</div>
		<div class="grid grid-cols-1 gap-8 md:grid-cols-2">
			<a href="/app" class="rounded-lg border p-4">
				<div class="group text-center">
					<h1 class="text-4xl font-bold group-hover:underline md:text-5xl">App</h1>
					<p class="mt-2 text-lg font-bold group-hover:underline">
						Personal & Business Bitcoin Finance
					</p>
					<p class="mt-4">
						Helping businesses and individuals manage life on a Bitcoin standard. Connect your
						wallets, consolidate transactions, and generate reports—effortlessly.
					</p>
				</div>
			</a>
			<a href="/api" class="rounded-lg border p-4">
				<div class="group text-center">
					<h1 class="text-4xl font-bold group-hover:underline md:text-5xl">API</h1>
					<p class="mt-2 text-lg font-bold group-hover:underline">
						Bitcoin Accounting for Your App
					</p>
					<p class="mt-4">
						Seamlessly integrate Bitcoin accounting into your app. Automate transaction tracking,
						gain real-time financial insights, and simplify Bitcoin bookkeeping.
					</p>
				</div>
			</a>
		</div>
	</div>
</div>
