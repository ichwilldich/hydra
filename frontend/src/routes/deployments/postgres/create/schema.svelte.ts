import z from 'zod';

export const generalInformation = z.object({
  name: z.string().min(1, 'Name is required').default(''),
});

export const resources = z.object({
  cpu: z
    .number()
    .min(1, 'CPU must be at least 1')
    .default(1),
  memory: z
    .number()
    .min(1, 'Memory must be at least 1 GB')
    .default(1),
});

export const cancelDeployment = z.object({
  _phantom_cancel: z.string().default(''),
})