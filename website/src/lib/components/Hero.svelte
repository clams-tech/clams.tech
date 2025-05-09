<script lang="ts">
	import clamsLogo from '$lib/icons/clams-logo.svg';
	import clamsSeal from '$lib/images/clams-seal.png';
</script>

<section class="px-4 py-16">
	<div class="mt-1 mb-16 border-b"></div>
	<div class="m-auto grid max-w-7xl gap-8 md:grid-cols-2">
		<div class="flex flex-col items-center justify-center gap-8">
			<img src={clamsLogo} alt="Clams hero icon" class="w-full" />
			<div class="max-w-md text-center">
				<p class="text-2xl font-bold">Purpose Built Bitcoin Accounting</p>
				<p class="mt-4">
					Clams unifies all your Bitcoin activity into one trusted, private, best-in-class desktop
					app.
				</p>
			</div>
		</div>
		<div class="w-full overflow-hidden rounded-lg">
			<img src={clamsSeal} alt="Clams Seal" class="h-full w-full object-cover" />
		</div>
	</div>
</section>
