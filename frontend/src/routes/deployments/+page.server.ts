import { redirect } from '@sveltejs/kit';

export const load = () => {
  // the deployments route itself has no content so back to dashboard
  redirect(307, '/');
};
