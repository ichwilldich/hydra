import type { CreateDeployment } from '$lib/backend/postgres.svelte';
import type { FormRecord } from 'positron-components/components/form';
import z from 'zod';

export const units = {
  B: 1,
  KB: 1000,
  KiB: 1024,
  MB: 1000 * 1000,
  MiB: 1024 * 1024,
  GB: 1000 * 1000 * 1000,
  GiB: 1024 * 1024 * 1024,
  TB: 1000 * 1000 * 1000 * 1000,
  TiB: 1024 * 1024 * 1024 * 1024
};

export enum PostgresVersion {
  V13 = '13',
  V14 = '14',
  V15 = '15',
  V16 = '16',
  V17 = '17',
  V18 = '18'
}

export const reformatData = (data: FormRecord): CreateDeployment => {
  let storage_size = data.storage_size as number;
  let storage_size_unit = (data.storage_size_unit as string[])[0];
  let storage_mb = Math.ceil(
    (storage_size * (units as Record<string, number>)[storage_size_unit]) /
      (1000 * 1000)
  );

  return {
    name: data.name as string,
    storage_mb
  };
};

export const generalInformation = z.object({
  name: z.string().min(1, 'Name is required').default(''),
  namespace: z.string().default(''),
  version: z.array(z.enum(PostgresVersion)).default([PostgresVersion.V18]),
  replicas: z.number().default(1)
});

const amount = z.number().gt(0, 'Must be greater than 0').default(1);
const unit = z
  .array(z.enum(Object.keys(units)))
  .min(1)
  .max(1)
  .default(['GB']);

export const resources = z.object({
  storage_size: amount,
  storage_size_unit: unit,
  memory_request_size: amount,
  memory_request_size_unit: unit,
  memory_limit_size: amount,
  memory_limit_size_unit: unit,
  cpu_request: amount,
  cpu_limit: amount
});

export const backup = z.object({
  backups_enabled: z.boolean().default(false),
  backup_schedule: z.string().default('0 0 * * *'),
  backup_retention: z.number().min(1, 'Must be at least 1').default(7),
  backup_storage_location: z.string().default('')
});

export enum RefType {
  ConfigMap = 'ConfigMap',
  Secret = 'Secret'
}

export const connection = z
  .object({
    external_access: z.boolean().default(false),
    ssl_enabled: z.boolean().default(false),
    ssl_cert_file: z.file().optional(),
    ssl_cert_ref_type: z.enum(RefType).optional(),
    ssl_cert_ref_name: z.string().optional(),
    ssl_cert_ref_key: z.string().optional(),
    ssl_cert_host_file_path: z.string().optional(),
    ssl_key_file: z.file().optional(),
    ssl_key_ref_type: z.enum(RefType).optional(),
    ssl_key_ref_name: z.string().optional(),
    ssl_key_ref_key: z.string().optional(),
    ssl_key_host_file_path: z.string().optional(),
    ssl_ca_file: z.file().optional(),
    ssl_ca_ref_type: z.enum(RefType).optional(),
    ssl_ca_ref_name: z.string().optional(),
    ssl_ca_ref_key: z.string().optional(),
    ssl_ca_host_file_path: z.string().optional()
  })
  .superRefine((data, ctx) => {
    if (data.ssl_enabled) {
      let cert_set =
        data.ssl_cert_file ||
        (data.ssl_cert_ref_type &&
          data.ssl_cert_ref_name &&
          data.ssl_cert_ref_key) ||
        data.ssl_cert_host_file_path;
      let key_set =
        data.ssl_key_file ||
        (data.ssl_key_ref_type &&
          data.ssl_key_ref_name &&
          data.ssl_key_ref_key) ||
        data.ssl_key_host_file_path;

      if (!cert_set) {
        ctx.addIssue({
          code: 'custom',
          message: 'You must provide an SSL certificate'
        });
      }

      if (!key_set) {
        ctx.addIssue({
          code: 'custom',
          message: 'You must provide an SSL key'
        });
      }
    }
  });

export const monitoring = z.object({
  monitoring_enabled: z.boolean().default(false),
  external_access: z.boolean().default(false),
  deploy_monitoring_resources: z.boolean().default(true)
});

export const advanced = z.object({
  allow_alter_system: z.boolean().default(false),
  extra_database_parameters: z.string().default('')
});

export const summary = z.object({
  _phantom_summary: z.string().default('')
});

export const cancelDeployment = z.object({
  _phantom_cancel: z.string().default('')
});
