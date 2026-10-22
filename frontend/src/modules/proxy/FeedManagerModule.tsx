import React,{useCallback,useEffect,useState} from 'react';
import {Card} from 'primereact/card';
import {Button} from 'primereact/button';
import {InputText} from 'primereact/inputtext';
import {Dropdown} from 'primereact/dropdown';
import {DataTable} from 'primereact/datatable';
import {Column} from 'primereact/column';
import {Message} from 'primereact/message';
import {api} from '../../shared/services/api';

type Feed={id:string;name:string;url:string;category:string;enabled:boolean};
type Category={slug:string};
const root='/api/v1/secure-proxy';
export default function FeedManagerModule(){
 const [feeds,setFeeds]=useState<Feed[]>([]);
 const [categories,setCategories]=useState<Category[]>([]);
 const [form,setForm]=useState({name:'',url:'',category:'malware'});
 const [error,setError]=useState('');
 const [notice,setNotice]=useState('');
 const [busy,setBusy]=useState(false);
 const load=useCallback(async()=>{
  try{
   const [f,c]=await Promise.all([api<Feed[]>(root+'/feeds'),api<Category[]>(root+'/categories')]);
   setFeeds(f);setCategories(c);setError('');
  }catch(e){setError(String(e))}
 },[]);
 useEffect(()=>{void load()},[load]);
 async function run(fn:()=>Promise<void>){
  setBusy(true);setError('');setNotice('');
  try{await fn();await load()}catch(e){setError(String(e))}finally{setBusy(false)}
 }
 return <div className="grid">
  <Card title="Feed Manager — Web Reputation" className="col-12">
   {error&&<Message severity="error" text={error}/>}
   {notice&&<Message severity="success" text={notice}/>}
   <div className="flex flex-wrap gap-2 align-items-center">
    <InputText aria-label="Feed name" placeholder="Feed name" value={form.name} onChange={e=>setForm({...form,name:e.target.value})}/>
    <InputText aria-label="Feed URL" placeholder="https://trusted.example/feed.txt" value={form.url} style={{minWidth:'24rem'}} onChange={e=>setForm({...form,url:e.target.value})}/>
    <Dropdown aria-label="Web category" value={form.category} options={categories.map(c=>({label:c.slug,value:c.slug}))} onChange={e=>setForm({...form,category:e.value})}/>
    <Button label="Add feed" disabled={busy||!form.name||!form.url} onClick={()=>void run(async()=>{
     await api(root+'/feeds',{method:'POST',body:JSON.stringify(form)});
     setForm({name:'',url:'',category:'malware'});setNotice('Feed added; refresh it to import indicators.');
    })}/>
    <Button label="Reload" outlined onClick={()=>void load()} disabled={busy}/>
   </div>
   <p>Only administrator-approved HTTPS hosts are accepted by the backend. An empty or unsuccessful refresh preserves the previous version.</p>
   <DataTable value={feeds} dataKey="id" paginator rows={15}>
    <Column field="name" header="Feed"/>
    <Column field="category" header="Category"/>
    <Column field="url" header="HTTPS URL"/>
    <Column field="enabled" header="Enabled" body={(f:Feed)=>f.enabled?'Yes':'No'}/>
    <Column header="Import / Refresh" body={(f:Feed)=><Button label="Refresh" icon="pi pi-sync" disabled={busy||!f.enabled} onClick={()=>void run(async()=>{
     const report=await api<{entries:number}>(root+'/feeds/'+encodeURIComponent(f.id)+'/refresh',{method:'POST'});
     setNotice('Imported '+report.entries+' indicators from '+f.name);
    })}/>}/>
   </DataTable>
  </Card>
 </div>;
}
