<script lang="ts">
	import ContentContainer from './shared/ContentContainer.svelte';
</script>

<ContentContainer title="Never miss an update">
	<p class="mt-8 text-center text-lg leading-8 text-gray-600 dark:text-gray-100 md:mt-12">
		Join our mailing list to get the latest updates delivered to your inbox.
	</p>
	<div id="mc_embed_shell" class="mt-4 flex items-center justify-center md:mt-6">
		<div id="mc_embed_signup" class="w-full max-w-md rounded-lg border border p-4 md:p-6">
			<form
				action="https://tech.us14.list-manage.com/subscribe/post?u=f8a848454e4e2bfaf4577f68f&amp;id=efb2bbd0c2&amp;f_id=002284e0f0"
				method="post"
				id="mc-embedded-subscribe-form"
				name="mc-embedded-subscribe-form"
				class="validate space-y-4"
				target="_self"
				novalidate
			>
				<div id="mc_embed_signup_scroll">
					<div class="mc-field-group flex flex-col space-y-2">
						<label for="mce-EMAIL" class="text-sm font-medium"
							>Email Address <span class="text-red-500">*</span></label
						>
						<input
							type="email"
							name="EMAIL"
							id="mce-EMAIL"
							required
							class="w-full rounded-lg border border-gray-300 px-4 py-2 focus:outline-none focus:ring focus:ring-blue-300"
							placeholder="Enter your email"
						/>
					</div>
					<div id="mce-responses" class="hidden">
						<div class="response text-red-500" id="mce-error-response" style="display: none;"></div>
						<div
							class="response text-green-500"
							id="mce-success-response"
							style="display: none;"
						></div>
					</div>
					<div aria-hidden="true" class="hidden">
						<input
							type="text"
							name="b_f8a848454e4e2bfaf4577f68f_efb2bbd0c2"
							tabindex="-1"
							value=""
						/>
					</div>
					<div class="mt-2">
						<input
							type="submit"
							name="subscribe"
							id="mc-embedded-subscribe"
							class="text-md block cursor-pointer rounded-md bg-indigo-600 px-3 py-2 text-center font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
							value="Subscribe"
						/>
					</div>
				</div>
			</form>
		</div>
	</div>
</ContentContainer>
