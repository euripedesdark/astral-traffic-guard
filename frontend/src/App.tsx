import React, {useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {Button} from 'primereact/button';
import {Tag} from 'primereact/tag';
import {ProgressSpinner} from 'primereact/progressspinner';
import {Dropdown} from 'primereact/dropdown';
import i18n, {supportedLanguages} from './i18n';
import {api, me, logout} from './shared/services/api';
import Login from './shared/components/Login';
import {ORDER, PAGES} from './routes/routes';

function App() {
  const {t} = useTranslation();
  const [user, setUser] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [section, setSection] = useState('acl');

  useEffect(() => {
    me()
      .then(setUser)
      .catch(async () => {
        const params = new URLSearchParams(window.location.search);
        if (params.has('local') || params.has('ssoError')) return;
        try {
          const sso = await api<{enabled: boolean; authorizationUrl: string}>('/api/auth/sso');
          if (sso.enabled) window.location.assign(sso.authorizationUrl);
        } catch (_) {}
      })
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div className="center"><ProgressSpinner/></div>;
  if (!user) return <Login onAuth={setUser}/>;

  const isAdmin = (user.authorities || []).some((a: any) => a.authority === 'ROLE_ASTRAL_ADMIN');
  const Page = PAGES[section] || PAGES['acl'];

  return (
    <div className="page">
      <header className="topbar">
        <div className="brand">ASTRAL<small>Traffic Guard</small></div>
        <div className="topright">
          {ORDER.map((item) => (
            <Button
              key={item}
              text={section !== item}
              label={t(`nav.${item}`)}
              onClick={() => setSection(item)}
            />
          ))}
          <Dropdown
            value={i18n.resolvedLanguage || i18n.language}
            options={supportedLanguages}
            onChange={(e) => i18n.changeLanguage(e.value)}
            style={{width: 140}}
          />
          <Tag value={isAdmin ? t('app.admin') : t('app.user')} severity="info"/>
          <Button text label={user.username} icon="pi pi-user"/>
          <Button
            text
            label={t('app.logout')}
            icon="pi pi-sign-out"
            onClick={async () => {
              await logout();
              setUser(null);
            }}
          />
        </div>
      </header>
      <h1 className="modtitle">{t(`app.modtitle.${section}`)}</h1>
      <Page user={user} setSection={setSection}/>
    </div>
  );
}

export default App;
