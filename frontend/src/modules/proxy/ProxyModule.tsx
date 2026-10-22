import React, {useCallback, useEffect, useState} from 'react';
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

type Rule = {id:string; name:string; identity_source:string; identity_group:string|null; host_pattern:string; port:number|null; action:'ALLOW'|'DENY'; priority:number; enabled:boolean; created_by:string};
type Audit = {id:string; recorded_at:string; actor:string; operation:string; target:string; outcome:string; identity_source:string};
const endpoint='/api/v1/secure-proxy';
export default function ProxyModule(){
 const [rules,setRules]=useState<Rule[]>([]);
 const [logs,setLogs]=useState<Audit[]>([]);
 const [error,setError]=useState('');
 const [busy,setBusy]=useState(false);
 const [form,setForm]=useState({name:'',identity_source:'ad',identity_group:'',host_pattern:'',port:null as number|null,action:'DENY',priority:100});
 const load=useCallback(async()=>{try {
   const [r,l]=await Promise.all([api<Rule[]>(endpoint+'/rules'),api<Audit[]>(endpoint+'/audit')]);
   setRules(r);setLogs(l);setError('');
 } catch(e){setError(String(e));}},[]);
 useEffect(()=>{void load();},[load]);
 async function create(){setBusy(true);try{
   await api(endpoint+'/rules',{method:'POST',body:JSON.stringify({...form,identity_group:form.identity_group || null})});
   setForm({...form,name:'',host_pattern:''});await load();
 }catch(e){setError(String(e));}finally{setBusy(false);}}
 async function remove(id:string){setBusy(true);try{
   await api(endpoint+'/rules/'+encodeURIComponent(id),{method:'DELETE'});await load();
 }catch(e){setError(String(e));}finally{setBusy(false);}}
 return <div className="grid">
   <Card title="Secure Proxy — Identity & Rules" className="col-12">
     {error && <Message severity="error" text={error}/>}
     <div className="flex flex-wrap gap-2 align-items-center">
       <InputText aria-label="Rule name" placeholder="Rule name" value={form.name} onChange={e=>setForm({...form,name:e.target.value})}/>
       <InputText aria-label="Domain pattern" placeholder="example.org or *.example.org" value={form.host_pattern} onChange={e=>setForm({...form,host_pattern:e.target.value})}/>
       <Dropdown aria-label="Identity source" value={form.identity_source} options={[{label:'AD / auth-service',value:'ad'},{label:'Local proxy role',value:'local'},{label:'Any authenticated',value:'any'}]} onChange={e=>setForm({...form,identity_source:e.value})}/>
       <InputText aria-label="Group" placeholder="AD group (optional)" value={form.identity_group} onChange={e=>setForm({...form,identity_group:e.target.value})}/>
       <InputNumber aria-label="Port" placeholder="Any port" value={form.port} min={1} max={65535} onValueChange={e=>setForm({...form,port:e.value ?? null})}/>
       <Dropdown aria-label="Action" value={form.action} options={[{label:'DENY',value:'DENY'},{label:'ALLOW',value:'ALLOW'}]} onChange={e=>setForm({...form,action:e.value})}/>
       <Button label="Create rule" disabled={busy || !form.name || !form.host_pattern} onClick={()=>void create()}/>
       <Button label="Refresh" outlined disabled={busy} onClick={()=>void load()}/>
     </div>
     <DataTable value={rules} dataKey="id" paginator rows={15} emptyMessage="No proxy rules">
       <Column field="name" header="Rule"/><Column field="identity_source" header="Identity"/>
       <Column field="identity_group" header="Group"/><Column field="host_pattern" header="Host"/>
       <Column field="port" header="Port"/><Column field="priority" header="Priority"/>
       <Column header="Action" body={(r:Rule)=><Tag value={r.action} severity={r.action==='DENY'?'danger':'success'}/>}/>
       <Column header="Manage" body={(r:Rule)=><Button label="Delete" size="small" severity="danger" outlined disabled={busy} onClick={()=>void remove(r.id)}/>}/>
     </DataTable>
   </Card>
   <Card title="ISO security audit — last 200 entries" className="col-12">
     <DataTable value={logs} dataKey="id" paginator rows={15} emptyMessage="No audit events">
       <Column field="recorded_at" header="Timestamp"/><Column field="actor" header="Identity"/>
       <Column field="identity_source" header="Source"/><Column field="operation" header="Operation"/>
       <Column field="target" header="Target"/><Column field="outcome" header="Result"/>
     </DataTable>
   </Card>
 </div>;
}
