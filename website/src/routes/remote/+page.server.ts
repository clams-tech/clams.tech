import { redirect } from '@sveltejs/kit';
import { GITHUB_URL } from '$lib/constants';

export function load() {
	redirect(307, `${GITHUB_URL}/Remote`);
}
