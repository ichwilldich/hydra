import { delete_, get, post, ResponseType } from 'positron-components/backend';
import type {
  CertSource,
  PostgresVersion
} from '../../routes/deployments/postgres/create/schema.svelte';

export interface CreateDeployment {
  name: string;
  namespace?: string;
  version: PostgresVersion;
  replicas: number;
  resources: CreateDeploymentResources;
  backup: CreateDeploymentBackup;
  connection: CreateDeploymentConnection;
  monitoring: CreateDeploymentMonitoring;
  advanced: CreateDeploymentAdvanced;
}

export interface CreateDeploymentResources {
  storage_mb: number;
  memory_request_mb: number;
  memory_limit_mb: number;
  cpu_request_millicores: number;
  cpu_limit_millicores: number;
}

export type CreateDeploymentBackup =
  | {
      enabled: false;
    }
  | {
      enabled: true;
      schedule: string;
      retention_days: number;
      storage_location: string;
    };

export type CreateDeploymentConnection = {
  external_access: boolean;
} & (
  | {
      ssl_enabled: false;
    }
  | ({
      ssl_enabled: true;
      ssl_cert: SslFile;
      ssl_key: SslFile;
    } & (
      | {
          ca_enabled: false;
        }
      | {
          ca_enabled: true;
          ssl_ca: SslFile;
        }
    ))
);

export type SslFile =
  | {
      type: CertSource.Auto;
    }
  | {
      type: CertSource.Text;
      content: string;
    }
  | {
      type: CertSource.Reference;
      ref_name: string;
      ref_key: string;
    }
  | {
      type: CertSource.HostFilePath;
      host_file_path: string;
    };

export type CreateDeploymentMonitoring =
  | {
      enabled: false;
    }
  | {
      enabled: true;
      external_access: boolean;
      deploy_monitoring: boolean;
    };

export interface CreateDeploymentAdvanced {
  allow_alter_system: boolean;
  extra_params: Record<string, string>;
}

export const create_deployment = async (payload: CreateDeployment) => {
  let res = await post<undefined>(
    '/api/deployment/postgres',
    ResponseType.None,
    payload
  );

  return res;
};

export interface DeploymentInfo {
  uuid: string;
  name: string;
}

export const list_deployments = async () => {
  let res = await get<DeploymentInfo[]>(
    '/api/deployment/postgres',
    ResponseType.Json
  );

  if (Array.isArray(res)) {
    return res;
  }
};

export const delete_deployment = async (uuid: string) => {
  let res = await delete_<undefined>(
    `/api/deployment/postgres`,
    ResponseType.None,
    { uuid }
  );

  return res;
};

export enum ConnectorType {
  Docker = 'Docker',
  Kubernetes = 'Kubernetes'
}

export interface SystemInfo {
  connector: ConnectorType;
  namespaces: string[];
}

export const system_info = async () => {
  let res = await get<SystemInfo>(
    '/api/deployment/postgres/info',
    ResponseType.Json
  );

  if (typeof res === 'object') {
    return res;
  }
};
