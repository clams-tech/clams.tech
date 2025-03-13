<script>
	import { API_DOCS_URL } from '$lib/constants';
	import BetaForm from '$lib/components/api/BetaForm.svelte';
</script>

<section class="mx-auto mb-24 mt-36 max-w-5xl lg:px-6">
	<h1 class="text-center text-4xl font-bold">Get Started</h1>
	<div class="mt-12 grid grid-cols-1 gap-12 md:grid-cols-2">
		<div class="w-full">
			<p class="text-lg">
				<strong>Founders:</strong> Become an early partner for our closed beta.
			</p>
			<BetaForm />
		</div>
		<div class="flex w-full flex-grow flex-col justify-between">
			<!-- API Docs CTA -->
			<p class="block text-lg">
				<strong>Developers:</strong> Explore our API docs for full integration details.
			</p>
			<div class="mt-2 flex w-full justify-end">
				<a
					href={API_DOCS_URL}
					rel="noopener noreferrer"
					target="_blank"
					class="text-md block cursor-pointer rounded-md border border-indigo-600 bg-indigo-600 px-3 py-2 text-center font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
				>
					View API Docs 📖
				</a>
			</div>
		</div>
	</div>
</section>
