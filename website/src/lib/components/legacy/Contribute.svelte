<script lang="ts">
	import ClamsGradientIcon from '$lib/icons/clams-gradient';
	import ArrowUpIcon from '$lib/icons/arrow-up';
	import { scrollTop } from 'svelte-scrolling';
</script>

<section
	class="flex flex-col items-center gap-20 bg-[#F4F6F8] px-6 py-[72px] md:py-[96px] dark:bg-[#1e1e1e]"
>
	<div class="flex w-full max-w-3xl items-center gap-6">
		<div class="">
			{@html ClamsGradientIcon}
		</div>
		<div class="flex flex-col gap-2">
			<h2 class="text-2xl text-[20px]">
				Want to contribute or have ideas on how to improve our apps?
			</h2>
			<p>
				Join us in our <a class="text-light-purple dark:text-bitcoin font-bold underline" href="/"
					>Discord</a
				> server
			</p>
		</div>
	</div>

	<button class="text-bold flex cursor-pointer items-center gap-2" on:click={() => scrollTop()}>
		Scroll to top {@html ArrowUpIcon}</button
	>
</section>
