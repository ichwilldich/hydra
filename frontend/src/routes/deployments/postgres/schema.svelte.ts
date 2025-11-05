import z from 'zod';

export const deleteDeployment = z.object({
  _phantom_delete: z.string().default('')
});
