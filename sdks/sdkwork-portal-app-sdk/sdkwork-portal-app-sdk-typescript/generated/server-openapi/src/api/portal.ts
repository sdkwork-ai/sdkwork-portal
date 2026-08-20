import { appApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { PortalPreferencesUpdateRequest } from '../types';


export class PortalPreferencesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve current user portal preferences */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<{ pinnedAppKeys: string[]; theme: string; }> {
    return this.client.request<{ pinnedAppKeys: string[]; theme: string; }>(appApiPath(`/portal/preferences`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update current user portal preferences */
  async update(body: PortalPreferencesUpdateRequest, requestOptions?: ApiRequestOptions): Promise<Record<string, unknown>> {
    return this.client.request<Record<string, unknown>>(appApiPath(`/portal/preferences`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PUT' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
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
