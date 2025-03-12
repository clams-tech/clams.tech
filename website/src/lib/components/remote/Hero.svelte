<script lang="ts">
	import { REMOTE_DOCS_URL, REMOTE_APP_URL } from '$lib/constants';
	import Button from '$lib/elements/Button.svelte';
	import RemoteIcon from '$lib/icons/remote';
</script>

<section class="flex w-full items-center justify-center md:h-screen">
	<div class="mt-28 flex w-full max-w-[1000px] justify-center px-6 md:mt-0">
		<div class="flex flex-col gap-8 text-center">
			<div class="m-auto w-80">
				{@html RemoteIcon}
			</div>
			<div>
				<h1 class="text-6xl font-bold">Secure. Private. Fast.</h1>
				<div class="mt-4 flex flex-col text-center">
					<h2 class="text-2xl">
						The essential interface for your Core <span class="text-light-orange">Lightning</span> Node.
					</h2>
					<div class="m-auto mt-8 flex gap-6">
						<a href={REMOTE_DOCS_URL} target="_blank" rel="noopener noreferrer" class={'w-[134px]'}>
							<Button text="Learn More" />
						</a>
						<a class={'w-[134px]'} target="_blank" rel="noopener noreferrer" href={REMOTE_APP_URL}>
							<Button text="Get Started" primary />
						</a>
					</div>
				</div>
			</div>
		</div>
	</div>
</section>
