import { backendApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { PageInfo } from '../types';


export class PortalPreferencesAdminApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List portal preferences for tenant administration */
  async list(requestOptions?: ApiRequestOptions): Promise<{ items: { userId: string; theme: string; pinnedCount: number; }[]; pageInfo: PageInfo; }> {
    return this.client.request<{ items: { userId: string; theme: string; pinnedCount: number; }[]; pageInfo: PageInfo; }>(backendApiPath(`/portal/preferences`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class PortalPreferencesApi {
  public readonly admin: PortalPreferencesAdminApi;

  constructor(client: HttpClient) {
    this.admin = new PortalPreferencesAdminApi(client);
  }

}

export class PortalApi {
  public readonly preferences: PortalPreferencesApi;

  constructor(client: HttpClient) {
    this.preferences = new PortalPreferencesApi(client);
  }

}

export function createPortalApi(client: HttpClient): PortalApi {
  return new PortalApi(client);
}
