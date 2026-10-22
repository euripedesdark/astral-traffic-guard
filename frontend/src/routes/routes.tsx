import ProxyPolicyModule from '../modules/proxy/ProxyPolicyModule';
import FeedManagerModule from '../modules/proxy/FeedManagerModule';
import ProxyPage from '../modules/proxy/pages/Index';
import AclPage from '../modules/proxy/pages/Acl';

export const ORDER = ['acl', 'trafficServer', 'proxyPolicies','feeds'];
export const ICONS: Record<string, string> = {
  acl: 'pi pi-filter',
  trafficServer: 'pi pi-globe',
  proxyPolicies: 'pi pi-shield',
  feeds: 'pi pi-cloud-download'
};
export const PAGES: Record<string, any> = {
  acl: AclPage,
  trafficServer: ProxyPage,
  proxyPolicies: ProxyPolicyModule,
  feeds: FeedManagerModule
};
