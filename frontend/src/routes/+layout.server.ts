import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = ({ cookies, url }) => {
  // redirect to /login when there is no auth_token cookie
  const authToken = cookies.get('auth_token');
  let path = url.pathname;

  if (path === '/login') return;

  if (!authToken) {
    redirect(302, '/login');
  }

  let content_bs64 = authToken.split('.')[1];
  let content_json = Buffer.from(content_bs64, 'base64').toString('utf-8');
  let content = JSON.parse(content_json);
  let exp = content.exp * 1000; // convert to milliseconds
  let now = Date.now();

  // redirect to /login when the auth_token cookie is expired
  if (now >= exp) {
    cookies.delete('auth_token', { path: '/' });
    redirect(302, '/login');
  }
};
