import ProxyPage from '../modules/proxy/pages/Index';
import AclPage from '../modules/proxy/pages/Acl';

export const ORDER = ['acl', 'trafficServer'];
export const ICONS: Record<string, string> = {
  acl: 'pi pi-filter',
  trafficServer: 'pi pi-globe'
};
export const PAGES: Record<string, any> = {
  acl: AclPage,
  trafficServer: ProxyPage
};
