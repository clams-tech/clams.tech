<script lang="ts">
	import ClamsIcon from '$lib/icons/clams';
	import BetaForm from '$lib/components/api/BetaForm.svelte';
</script>

<section class="flex items-center justify-center md:h-screen">
	<div class="relative isolate mx-auto max-w-2xl py-32 text-center">
		<!-- Logo -->
		<div class="ml-auto mr-auto w-52 md:w-64">
			{@html ClamsIcon}
		</div>
		<!-- Header -->
		<h1 class="mt-8 max-w-lg text-4xl font-bold">Effortless Accounting for Bitcoin</h1>
		<!-- Beta CTA -->
		<div class="m-auto mt-8 grid max-w-lg grid-cols-1 gap-4">
			<p class="text-center text-lg md:text-xl">
				<strong>Join the Beta:</strong> Be one of the first to integrate advanced accounting into your
				Bitcoin app.
			</p>
			<BetaForm />
		</div>
	</div>
</section>
