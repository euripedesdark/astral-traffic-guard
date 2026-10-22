import React, {useCallback, useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {Card} from 'primereact/card';
import {Button} from 'primereact/button';
import {InputText} from 'primereact/inputtext';
import {InputNumber} from 'primereact/inputnumber';
import {Dropdown} from 'primereact/dropdown';
import {DataTable} from 'primereact/datatable';
import {Column} from 'primereact/column';
import {Message} from 'primereact/message';
import {api} from '../../shared/services/api';

type Action = 'ALLOW' | 'DENY' | 'INSPECT';
type Kind =
  | 'destination_domain'
  | 'source_ip_range'
  | 'destination_ip_range'
  | 'url_regex'
  | 'http_method'
  | 'destination_port'
  | 'user_agent_regex'
  | 'ad_group'
  | 'web_category'
  | 'application'
  | 'interface'
  | 'zone'
  | 'header'
  | 'time_window';
type Condition = {
  type: Kind;
  value: string | number | {name: string; value_regex: string} | {start_utc: string; end_utc: string};
};
type Policy = {
  id: string;
  name: string;
  action: Action;
  conditions: Condition[];
  priority: number;
  enabled: boolean;
};

const types: Kind[] = [
  'destination_domain',
  'source_ip_range',
  'destination_ip_range',
  'url_regex',
  'http_method',
  'destination_port',
  'user_agent_regex',
  'ad_group',
  'web_category',
  'application',
  'interface',
  'zone',
  'header',
  'time_window'
];
const options = types.map((t) => ({label: t.replace(/_/g, ' '), value: t}));
const empty = {name: '', action: 'DENY' as Action, priority: 100, conditions: [] as Condition[]};
const apiRoot = '/api/v1/secure-proxy/policies';

export default function ProxyPolicyModule() {
  const {t} = useTranslation();
  const [policies, setPolicies] = useState<Policy[]>([]);
  const [form, setForm] = useState(empty);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');

  const reload = useCallback(async () => {
    try {
      setPolicies(await api<Policy[]>(apiRoot));
      setError('');
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    void reload();
  }, [reload]);

  const add = () =>
    setForm((f) => ({...f, conditions: [...f.conditions, {type: 'destination_domain', value: ''}]}));

  const change = (i: number, c: Condition) =>
    setForm((f) => ({...f, conditions: f.conditions.map((v, j) => (i === j ? c : v))}));

  const removeCondition = (i: number) =>
    setForm((f) => ({...f, conditions: f.conditions.filter((_, j) => j !== i)}));

  const submit = async () => {
    setBusy(true);
    try {
      if (!form.name.trim() || !form.conditions.length)
        throw new Error(t('proxyPolicies.errors.required'));
      await api(apiRoot, {method: 'POST', body: JSON.stringify(form)});
      setForm(empty);
      await reload();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const drop = async (id: string) => {
    setBusy(true);
    try {
      await api(apiRoot + '/' + encodeURIComponent(id), {method: 'DELETE'});
      await reload();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="grid">
      <Card title={t('proxyPolicies.title')} className="col-12">
        {error && <Message severity="error" text={error} />}
        <div className="flex flex-wrap gap-2 align-items-center">
          <InputText
            aria-label={t('proxyPolicies.namePlaceholder')}
            placeholder={t('proxyPolicies.namePlaceholder')}
            value={form.name}
            onChange={(e) => setForm({...form, name: e.target.value})}
          />
          <Dropdown
            aria-label={t('proxyPolicies.action')}
            value={form.action}
            options={['DENY', 'ALLOW', 'INSPECT'].map((v) => ({label: v, value: v}))}
            onChange={(e) => setForm({...form, action: e.value})}
          />
          <InputNumber
            aria-label={t('proxyPolicies.priority')}
            value={form.priority}
            onValueChange={(e) => setForm({...form, priority: e.value ?? 100})}
          />
          <Button label={t('proxyPolicies.addCondition')} outlined icon="pi pi-plus" onClick={add} />
        </div>
        <p>{t('proxyPolicies.policyNote')}</p>
        {form.conditions.map((c, i) => (
          <div className="flex flex-wrap gap-2 align-items-center" key={i} style={{marginBottom: 8}}>
            <Dropdown
              aria-label={t('proxyPolicies.condition') + ' ' + (i + 1)}
              value={c.type}
              options={options}
              onChange={(e) =>
                change(i, {
                  type: e.value,
                  value:
                    e.value === 'destination_port'
                      ? 443
                      : e.value === 'header'
                      ? {name: '', value_regex: ''}
                      : e.value === 'time_window'
                      ? {start_utc: '09:00', end_utc: '17:00'}
                      : ''
                })
              }
            />
            {c.type === 'header' ? (
              <>
                <InputText
                  placeholder={t('proxyPolicies.headerName')}
                  value={(c.value as {name: string; value_regex: string}).name}
                  onChange={(e) =>
                    change(i, {
                      ...c,
                      value: {...(c.value as {name: string; value_regex: string}), name: e.target.value}
                    })
                  }
                />
                <InputText
                  placeholder={t('proxyPolicies.headerRegex')}
                  value={(c.value as {name: string; value_regex: string}).value_regex}
                  onChange={(e) =>
                    change(i, {
                      ...c,
                      value: {
                        ...(c.value as {name: string; value_regex: string}),
                        value_regex: e.target.value
                      }
                    })
                  }
                />
              </>
            ) : c.type === 'time_window' ? (
              <>
                <InputText
                  placeholder={t('proxyPolicies.startUtc')}
                  value={(c.value as {start_utc: string; end_utc: string}).start_utc}
                  onChange={(e) =>
                    change(i, {
                      ...c,
                      value: {
                        ...(c.value as {start_utc: string; end_utc: string}),
                        start_utc: e.target.value
                      }
                    })
                  }
                />
                <InputText
                  placeholder={t('proxyPolicies.endUtc')}
                  value={(c.value as {start_utc: string; end_utc: string}).end_utc}
                  onChange={(e) =>
                    change(i, {
                      ...c,
                      value: {
                        ...(c.value as {start_utc: string; end_utc: string}),
                        end_utc: e.target.value
                      }
                    })
                  }
                />
              </>
            ) : c.type === 'destination_port' ? (
              <InputNumber
                value={Number(c.value)}
                min={1}
                max={65535}
                onValueChange={(e) => change(i, {...c, value: e.value ?? 443})}
              />
            ) : (
              <InputText
                placeholder={t('proxyPolicies.conditionValue')}
                value={String(c.value)}
                onChange={(e) => change(i, {...c, value: e.target.value})}
              />
            )}
            <Button
              icon="pi pi-trash"
              severity="danger"
              outlined
              aria-label={t('proxyPolicies.removeCondition')}
              onClick={() => removeCondition(i)}
            />
          </div>
        ))}
        <Button
          label={t('proxyPolicies.createPolicy')}
          disabled={busy || !form.conditions.length}
          onClick={() => void submit()}
        />
      </Card>
      <Card title={t('proxyPolicies.orderedPolicies')} className="col-12">
        <Button label={t('common.reload')} icon="pi pi-refresh" outlined onClick={() => void reload()} />
        <DataTable value={policies} dataKey="id" paginator rows={15} sortField="priority" sortOrder={-1}>
          <Column field="priority" header={t('proxyPolicies.colPriority')} sortable />
          <Column field="name" header={t('proxyPolicies.colPolicy')} />
          <Column field="action" header={t('proxyPolicies.colAction')} />
          <Column
            header={t('proxyPolicies.colConditions')}
            body={(p: Policy) => p.conditions.map((c) => c.type).join(', ')}
          />
          <Column
            header={t('proxyPolicies.colDelete')}
            body={(p: Policy) => (
              <Button
                label={t('proxyPolicies.delete')}
                outlined
                severity="danger"
                disabled={busy}
                onClick={() => void drop(p.id)}
              />
            )}
          />
        </DataTable>
      </Card>
    </div>
  );
}
