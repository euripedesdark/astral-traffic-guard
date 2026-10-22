import React from 'react';
import {useTranslation} from 'react-i18next';
import {Card} from 'primereact/card';
import {Conf} from '../../../shared/components/Conf';

export default function ProxyPage() {
  const {t} = useTranslation();
  return (
    <div className="grid">
      <Conf
        rows={[
          [t('proxy.colListen'), t('proxy.listen')],
          [t('proxy.colAuth'), t('proxy.auth')],
          [t('proxy.colMode'), t('proxy.mode')],
          [t('proxy.colRemap'), t('proxy.remap')]
        ]}
      />
      <Card title={t('proxy.flowTitle')}>
        <p className="muted">{t('proxy.flow')}</p>
      </Card>
    </div>
  );
}
