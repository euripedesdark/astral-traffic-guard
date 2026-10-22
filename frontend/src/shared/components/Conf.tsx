import React from 'react';
import {useTranslation} from 'react-i18next';
import {Card} from 'primereact/card';

export function Conf({rows, title}: {rows: [string, string][]; title?: string}) {
  const {t} = useTranslation();
  return (
    <Card title={title || t('common.configuration')}>
      <table className="kv">
        <tbody>
          {rows.map(([k, v]) => (
            <tr key={k}>
              <td>{k}</td>
              <td><b>{v}</b></td>
            </tr>
          ))}
        </tbody>
      </table>
    </Card>
  );
}
