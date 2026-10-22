import React, {useCallback, useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {Card} from 'primereact/card';
import {Button} from 'primereact/button';
import {InputText} from 'primereact/inputtext';
import {Dropdown} from 'primereact/dropdown';
import {DataTable} from 'primereact/datatable';
import {Column} from 'primereact/column';
import {Message} from 'primereact/message';
import {api} from '../../shared/services/api';

type Feed = {id: string; name: string; url: string; category: string; enabled: boolean};
type Category = {slug: string};
const root = '/api/v1/secure-proxy';

export default function FeedManagerModule() {
  const {t} = useTranslation();
  const [feeds, setFeeds] = useState<Feed[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [form, setForm] = useState({name: '', url: '', category: 'malware'});
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const [busy, setBusy] = useState(false);

  const load = useCallback(async () => {
    try {
      const [f, c] = await Promise.all([
        api<Feed[]>(root + '/feeds'),
        api<Category[]>(root + '/categories')
      ]);
      setFeeds(f);
      setCategories(c);
      setError('');
    } catch (e) {
      setError(String(e));
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  async function run(fn: () => Promise<void>) {
    setBusy(true);
    setError('');
    setNotice('');
    try {
      await fn();
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="grid">
      <Card title={t('feeds.title')} className="col-12">
        {error && <Message severity="error" text={error} />}
        {notice && <Message severity="success" text={notice} />}
        <div className="flex flex-wrap gap-2 align-items-center">
          <InputText
            aria-label={t('feeds.namePlaceholder')}
            placeholder={t('feeds.namePlaceholder')}
            value={form.name}
            onChange={(e) => setForm({...form, name: e.target.value})}
          />
          <InputText
            aria-label={t('feeds.urlPlaceholder')}
            placeholder={t('feeds.urlPlaceholder')}
            value={form.url}
            style={{minWidth: '24rem'}}
            onChange={(e) => setForm({...form, url: e.target.value})}
          />
          <Dropdown
            aria-label={t('feeds.colCategory')}
            value={form.category}
            options={categories.map((c) => ({label: c.slug, value: c.slug}))}
            onChange={(e) => setForm({...form, category: e.value})}
          />
          <Button
            label={t('feeds.addFeed')}
            disabled={busy || !form.name || !form.url}
            onClick={() =>
              void run(async () => {
                await api(root + '/feeds', {method: 'POST', body: JSON.stringify(form)});
                setForm({name: '', url: '', category: 'malware'});
                setNotice(t('feeds.feedAddedNotice'));
              })
            }
          />
          <Button label={t('feeds.reload')} outlined onClick={() => void load()} disabled={busy} />
        </div>
        <p>{t('feeds.adminNote')}</p>
        <DataTable value={feeds} dataKey="id" paginator rows={15}>
          <Column field="name" header={t('feeds.colFeed')} />
          <Column field="category" header={t('feeds.colCategory')} />
          <Column field="url" header={t('feeds.colUrl')} />
          <Column
            field="enabled"
            header={t('feeds.colEnabled')}
            body={(f: Feed) => (f.enabled ? t('common.yes') : t('common.no'))}
          />
          <Column
            header={t('feeds.colImport')}
            body={(f: Feed) => (
              <Button
                label={t('feeds.refresh')}
                icon="pi pi-sync"
                disabled={busy || !f.enabled}
                onClick={() =>
                  void run(async () => {
                    const report = await api<{entries: number}>(
                      root + '/feeds/' + encodeURIComponent(f.id) + '/refresh',
                      {method: 'POST'}
                    );
                    setNotice(t('feeds.importedNotice', {entries: report.entries, name: f.name}));
                  })
                }
              />
            )}
          />
        </DataTable>
      </Card>
    </div>
  );
}
