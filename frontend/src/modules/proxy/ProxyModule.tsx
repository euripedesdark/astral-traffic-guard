import React, {useCallback, useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {Card} from 'primereact/card';
import {Button} from 'primereact/button';
import {Dropdown} from 'primereact/dropdown';
import {InputText} from 'primereact/inputtext';
import {InputNumber} from 'primereact/inputnumber';
import {DataTable} from 'primereact/datatable';
import {Column} from 'primereact/column';
import {Tag} from 'primereact/tag';
import {Message} from 'primereact/message';
import {api} from '../../shared/services/api';

type Rule = {
  id: string;
  name: string;
  identity_source: string;
  identity_group: string | null;
  host_pattern: string;
  port: number | null;
  action: 'ALLOW' | 'DENY';
  priority: number;
  enabled: boolean;
  created_by: string;
};
type Audit = {
  id: string;
  recorded_at: string;
  actor: string;
  operation: string;
  target: string;
  outcome: string;
  identity_source: string;
};

const endpoint = '/api/v1/secure-proxy';

export default function ProxyModule() {
  const {t} = useTranslation();
  const [rules, setRules] = useState<Rule[]>([]);
  const [logs, setLogs] = useState<Audit[]>([]);
  const [error, setError] = useState('');
  const [busy, setBusy] = useState(false);
  const [form, setForm] = useState({
    name: '',
    identity_source: 'ad',
    identity_group: '',
    host_pattern: '',
    port: null as number | null,
    action: 'DENY',
    priority: 100
  });

  const load = useCallback(async () => {
    try {
      const [r, l] = await Promise.all([
        api<Rule[]>(endpoint + '/rules'),
        api<Audit[]>(endpoint + '/audit')
      ]);
      setRules(r);
      setLogs(l);
      setError('');
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  async function create() {
    setBusy(true);
    try {
      await api(endpoint + '/rules', {
        method: 'POST',
        body: JSON.stringify({...form, identity_group: form.identity_group || null})
      });
      setForm({...form, name: '', host_pattern: ''});
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  async function remove(id: string) {
    setBusy(true);
    try {
      await api(endpoint + '/rules/' + encodeURIComponent(id), {method: 'DELETE'});
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="grid">
      <Card title={t('proxyModule.title')} className="col-12">
        {error && <Message severity="error" text={error} />}
        <div className="flex flex-wrap gap-2 align-items-center">
          <InputText
            aria-label={t('proxyModule.ruleName')}
            placeholder={t('proxyModule.ruleName')}
            value={form.name}
            onChange={(e) => setForm({...form, name: e.target.value})}
          />
          <InputText
            aria-label={t('proxyModule.domainPattern')}
            placeholder={t('proxyModule.domainPattern')}
            value={form.host_pattern}
            onChange={(e) => setForm({...form, host_pattern: e.target.value})}
          />
          <Dropdown
            aria-label={t('proxyModule.identitySource')}
            value={form.identity_source}
            options={[
              {label: t('proxyModule.sources.ad'), value: 'ad'},
              {label: t('proxyModule.sources.local'), value: 'local'},
              {label: t('proxyModule.sources.any'), value: 'any'}
            ]}
            onChange={(e) => setForm({...form, identity_source: e.value})}
          />
          <InputText
            aria-label={t('proxyModule.group')}
            placeholder={t('proxyModule.group')}
            value={form.identity_group}
            onChange={(e) => setForm({...form, identity_group: e.target.value})}
          />
          <InputNumber
            aria-label={t('proxyModule.port')}
            placeholder={t('proxyModule.port')}
            value={form.port}
            min={1}
            max={65535}
            onValueChange={(e) => setForm({...form, port: e.value ?? null})}
          />
          <Dropdown
            aria-label={t('proxyPolicies.action')}
            value={form.action}
            options={[
              {label: 'DENY', value: 'DENY'},
              {label: 'ALLOW', value: 'ALLOW'}
            ]}
            onChange={(e) => setForm({...form, action: e.value})}
          />
          <Button
            label={t('proxyModule.createRule')}
            disabled={busy || !form.name || !form.host_pattern}
            onClick={() => void create()}
          />
          <Button label={t('common.reload')} outlined disabled={busy} onClick={() => void load()} />
        </div>
        <DataTable value={rules} dataKey="id" paginator rows={15} emptyMessage={t('proxyModule.noRules')}>
          <Column field="name" header={t('proxyModule.colRule')} />
          <Column field="identity_source" header={t('proxyModule.colIdentity')} />
          <Column field="identity_group" header={t('proxyModule.colGroup')} />
          <Column field="host_pattern" header={t('proxyModule.colHost')} />
          <Column field="port" header={t('proxyModule.colPort')} />
          <Column field="priority" header={t('proxyModule.colPriority')} />
          <Column
            header={t('proxyModule.colAction')}
            body={(r: Rule) => (
              <Tag value={r.action} severity={r.action === 'DENY' ? 'danger' : 'success'} />
            )}
          />
          <Column
            header={t('proxyModule.colManage')}
            body={(r: Rule) => (
              <Button
                label={t('proxyModule.delete')}
                size="small"
                severity="danger"
                outlined
                disabled={busy}
                onClick={() => void remove(r.id)}
              />
            )}
          />
        </DataTable>
      </Card>
      <Card title={t('proxyModule.auditTitle')} className="col-12">
        <DataTable value={logs} dataKey="id" paginator rows={15} emptyMessage={t('proxyModule.noAudit')}>
          <Column field="recorded_at" header={t('proxyModule.colTimestamp')} />
          <Column field="actor" header={t('proxyModule.colIdentity')} />
          <Column field="identity_source" header={t('proxyModule.identitySource')} />
          <Column field="operation" header={t('proxyModule.colOperation')} />
          <Column field="target" header={t('proxyModule.colTarget')} />
          <Column field="outcome" header={t('proxyModule.colResult')} />
        </DataTable>
      </Card>
    </div>
  );
}
