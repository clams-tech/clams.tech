<script lang="ts">
	import { API_DOCS_URL } from '$lib/constants';

	let transactionCodeCopied: boolean = false;
	let transactionCode = `import fetch from 'node-fetch';

const API_KEY = 'your-api-key';
const TRANSACTION = {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  transactions: [
    {
      transactionId: 'tx123',
      timestamp: new Date().toISOString(),
      direction: 'send',
      amount: '1.5',
      currency: 'BTC',
      fee: '0.001',
      destinationWalletId: '550e8400-e29b-41d4-a716-446655440000',
      note: 'Payment for services',
      walletId: '550e8400-e29b-41d4-a716-446655440000'
    }
  ]
};

await fetch('https://staging.api.clams.tech/v0/transactions', {
  method: 'POST',
  headers: {
    'x-api-key': API_KEY,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify(TRANSACTION)
});`;

	let tradeCodeCopied: boolean = false;
	let tradeCode = `import fetch from 'node-fetch';

const API_KEY = 'your-api-key';
const TRADE = {
  userId: '550e8400-e29b-41d4-a716-446655440000',
  trades: [
    {
      tradeId: 'trade456',
      timestamp: new Date().toISOString(),
      fromCurrency: 'BTC',
      toCurrency: 'USD',
      fromAmount: '0.5',
      toAmount: '10000',
      feeAmount: '10',
      feeCurrency: 'USD',
      note: 'Sold BTC for USD',
      walletId: '550e8400-e29b-41d4-a716-446655440000'
    }
  ]
};

await fetch('https://staging.api.clams.tech/v0/trades', {
  method: 'POST',
  headers: {
    'x-api-key': API_KEY,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify(TRADE)
});`;

	function copyToClipboard(code: string, type: 'transaction' | 'trade') {
		navigator.clipboard
			.writeText(code)
			.then(() => {
				if (type === 'transaction') {
					transactionCodeCopied = true;
				}
				if (type === 'trade') {
					tradeCodeCopied = true;
				}
				// Hide tooltips after 2 seconds
				setTimeout(() => {
					transactionCodeCopied = false;
					tradeCodeCopied = false;
				}, 2000);
			})
			.catch((err) => {
				console.error('Failed to copy: ', err);
			});
	}
</script>

<section class="mx-auto mt-36 max-w-5xl lg:px-6">
	<h2 class="text-center text-4xl font-bold">Code Examples</h2>

	<p class="mt-4 text-center text-lg leading-relaxed">
		Capture user activity in real-time to surface insights and generate financial reports.
		<a class="underline" href={API_DOCS_URL} rel="noopener noreferrer" target="_blank"
			>Learn more.</a
		>
	</p>

	<div class="mt-8 grid grid-cols-1 gap-6 md:grid-cols-2 md:gap-8">
		<!-- Transaction example -->
		<div class="flex flex-grow flex-col">
			<h3 class="mb-2 text-xl font-bold text-black dark:text-white">Post a Transaction</h3>
			<div class="h-full rounded-lg border border-gray-300 bg-gray-900 p-4 text-white">
				<div class="relative mb-2 flex items-center justify-end">
					{#if transactionCodeCopied}
						<div
							id="tooltip-default"
							role="tooltip"
							class="shadow-xs tooltip absolute bottom-full mb-2 inline-block rounded-lg bg-gray-900 px-3 py-2 text-sm font-medium text-white opacity-100 transition-opacity duration-300 dark:bg-gray-700"
						>
							Copied!
							<div class="tooltip-arrow" data-popper-arrow></div>
						</div>
					{/if}
					<button
						class="rounded-md bg-indigo-600 px-3 py-2 font-semibold text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
						on:click={() => copyToClipboard(transactionCode, 'transaction')}
						aria-label="Copy code"
					>
						Copy
					</button>
				</div>
				<div class="overflow-x-auto">
					<pre class="text-gray-300"><code>{transactionCode}</code></pre>
				</div>
			</div>
		</div>
		<!-- Trade example -->
		<div class="flex flex-grow flex-col">
			<h3 class="mb-2 text-xl font-bold text-black dark:text-white">Post a Trade</h3>
			<div class="h-full rounded-lg border border-gray-300 bg-gray-900 p-4 text-white">
				<div class="relative mb-2 flex items-center justify-end">
					{#if tradeCodeCopied}
						<div
							id="tooltip-default"
							role="tooltip"
							class="shadow-xs tooltip absolute bottom-full mb-2 inline-block rounded-lg bg-gray-900 px-3 py-2 text-sm font-medium text-white opacity-100 transition-opacity duration-300 dark:bg-gray-700"
						>
							Copied!
							<div class="tooltip-arrow" data-popper-arrow></div>
						</div>
					{/if}
					<button
						class="rounded-md bg-indigo-600 px-3 py-2 font-semibold text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
						on:click={() => copyToClipboard(tradeCode, 'trade')}
						aria-label="Copy code"
					>
						Copy
					</button>
				</div>
				<div class="overflow-x-auto">
					<pre class="text-gray-300"><code>{tradeCode}</code></pre>
				</div>
			</div>
		</div>
	</div>
</section>
