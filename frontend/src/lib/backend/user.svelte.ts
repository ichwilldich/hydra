import { get, ResponseType } from 'positron-components/backend';

export interface UserInfo {
  id: string;
  name: string;
  email: string;
}

export const user_info = async () => {
  let res = await get<UserInfo>('/api/user/info', ResponseType.Json);

  if (typeof res === 'object') {
    return res;
  }
};
