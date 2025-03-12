<script>
	import { API_DOCS_URL } from '$lib/constants';
</script>

<section class="mx-auto mb-24 mt-36 max-w-5xl lg:px-6">
	<h1 class="text-center text-4xl font-bold">Get Started</h1>
	<div class="mt-12 grid grid-cols-1 gap-12 md:grid-cols-2">
		<div>
			<p class="text-lg">
				<strong>Founders:</strong> Become an early partner for our closed beta.
			</p>
			<div id="mc_embed_shell" class="mt-4">
				<div
					id="mc_embed_signup"
					class="w-full max-w-md rounded-lg border border-gray-400 p-4 md:p-6"
				>
					<form
						action="https://tech.us14.list-manage.com/subscribe/post?u=f8a848454e4e2bfaf4577f68f&amp;id=efb2bbd0c2&amp;f_id=002184e0f0"
						method="post"
						id="mc-embedded-subscribe-form"
						name="mc-embedded-subscribe-form"
						class="validate space-y-4"
						target="_self"
						novalidate
					>
						<div id="mc_embed_signup_scroll">
							<div class="mc-field-group flex flex-col space-y-2">
								<label for="mce-EMAIL" class="text-left text-sm font-medium"
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
							<div hidden><input type="hidden" name="tags" value="7433788" /></div>
							<div id="mce-responses" class="hidden">
								<div
									class="response text-red-500"
									id="mce-error-response"
									style="display: none;"
								></div>
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
							<div class="mt-3">
								<button
									type="submit"
									name="subscribe"
									id="mc-embedded-subscribe"
									class="text-md block cursor-pointer rounded-md bg-indigo-600 px-3 py-2 text-center font-semibold leading-6 text-white hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
								>
									Apply for Beta
								</button>
							</div>
						</div>
					</form>
				</div>
			</div>
		</div>
		<div>
			<!-- API Docs CTA -->
			<p class="text-lg">
				<strong>Developers:</strong> Explore our API docs for full integration details.
			</p>
			<a
				href={API_DOCS_URL}
				rel="noopener noreferrer"
				target="_blank"
				class="mt-4 inline-block rounded-lg border bg-black px-6 py-3 text-lg font-semibold text-white transition
	       hover:bg-gray-600
	       dark:border-white dark:bg-white dark:text-black
	       dark:hover:bg-gray-300"
			>
				View API Docs
			</a>
		</div>
	</div>
</section>
