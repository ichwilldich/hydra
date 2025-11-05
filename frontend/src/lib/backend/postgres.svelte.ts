import { delete_, get, post, ResponseType } from 'positron-components/backend';

export interface CreateDeployment {
  name: string;
  storage_mb: number;
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
