import { superValidate } from 'sveltekit-superforms';
import type { PageServerLoad } from './$types';
import {
  cancelDeployment,
  generalInformation,
  resources
} from './schema.svelte';
import { zod4 } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async () => {
  return {
    generalInformation: await superValidate(zod4(generalInformation)),
    cancelDeployment: await superValidate(zod4(cancelDeployment)),
    resources: await superValidate(zod4(resources))
  };
};
