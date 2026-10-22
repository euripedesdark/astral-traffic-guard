import ProxyPolicyModule from '../modules/proxy/ProxyPolicyModule';
import ProxyPage from '../modules/proxy/pages/Index';
import AclPage from '../modules/proxy/pages/Acl';

export const ORDER = ['acl', 'trafficServer', 'proxyPolicies'];
export const ICONS: Record<string, string> = {
  acl: 'pi pi-filter',
  trafficServer: 'pi pi-globe',
  proxyPolicies: 'pi pi-shield'
};
export const PAGES: Record<string, any> = {
  acl: AclPage,
  trafficServer: ProxyPage,
  proxyPolicies: ProxyPolicyModule
};
