<script lang="ts">
	import Downloads from '$lib/components/shared/Downloads.svelte';
</script>

<section class="px-4 pb-16 pt-28">
	<div class="m-auto flex max-w-5xl flex-col gap-4">
		<h1 class="text-6xl">Downloads</h1>
		<p>Clams is available as a Desktop app.</p>
		<div class="flex flex-col gap-4">
			<Downloads />
			<a
				class="underline"
				href="https://web.crabnebula.cloud/clams/clams/releases"
				target="_blank"
				rel="noopener noreferrer">More options</a
			>
		</div>
	</div>
</section>
