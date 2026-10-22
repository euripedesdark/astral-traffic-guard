import React, {useCallback,useEffect,useState} from 'react';
import {Card} from 'primereact/card';
import {Button} from 'primereact/button';
import {InputText} from 'primereact/inputtext';
import {InputNumber} from 'primereact/inputnumber';
import {Dropdown} from 'primereact/dropdown';
import {DataTable} from 'primereact/datatable';
import {Column} from 'primereact/column';
import {Message} from 'primereact/message';
import {api} from '../../shared/services/api';

type Action='ALLOW'|'DENY'|'INSPECT';
type Kind='destination_domain'|'source_ip_range'|'destination_ip_range'|'url_regex'|'http_method'|'destination_port'|'user_agent_regex'|'ad_group'|'web_category'|'application'|'interface'|'zone'|'header'|'time_window';
type Condition={type:Kind,value:string|number|{name:string,value_regex:string}|{start_utc:string,end_utc:string}};
type Policy={id:string;name:string;action:Action;conditions:Condition[];priority:number;enabled:boolean};
const types:Kind[]=['destination_domain','source_ip_range','destination_ip_range','url_regex','http_method','destination_port','user_agent_regex','ad_group','web_category','application','interface','zone','header','time_window'];
const options=types.map(t=>({label:t.replace(/_/g,' '),value:t}));
const empty={name:'',action:'DENY' as Action,priority:100,conditions:[] as Condition[]};
const apiRoot='/api/v1/secure-proxy/policies';

export default function ProxyPolicyModule(){
 const [policies,setPolicies]=useState<Policy[]>([]);
 const [form,setForm]=useState(empty);
 const [busy,setBusy]=useState(false);
 const [error,setError]=useState('');
 const reload=useCallback(async()=>{try{setPolicies(await api<Policy[]>(apiRoot));setError('');}
 catch(e){setError(String(e));}},[]);
 useEffect(()=>{void reload();},[reload]);
 const add=()=>setForm(f=>({...f,conditions:[...f.conditions,{type:'destination_domain',value:''}]}));
 const change=(i:number,c:Condition)=>setForm(f=>({...f,conditions:f.conditions.map((v,j)=>i===j?c:v)}));
 const removeCondition=(i:number)=>setForm(f=>({...f,conditions:f.conditions.filter((_,j)=>j!==i)}));
 const submit=async()=>{
   setBusy(true);try{
    if(!form.name.trim() || !form.conditions.length)throw new Error('Name and conditions required');
    await api(apiRoot,{method:'POST',body:JSON.stringify(form)});
    setForm(empty);await reload();
   }catch(e){setError(String(e));}finally{setBusy(false);}
 };
 const drop=async(id:string)=>{
   setBusy(true);try{await api(apiRoot+'/'+encodeURIComponent(id),{method:'DELETE'});await reload();}
   catch(e){setError(String(e));}finally{setBusy(false);}
 };
 return <div className="grid">
   <Card title="Hybrid Policy Builder — First Match Wins" className="col-12">
     {error && <Message severity="error" text={error}/>}
     <div className="flex flex-wrap gap-2 align-items-center">
       <InputText aria-label="Policy name" placeholder="Policy name" value={form.name} onChange={e=>setForm({...form,name:e.target.value})}/>
       <Dropdown aria-label="Action" value={form.action} options={['DENY','ALLOW','INSPECT'].map(v=>({label:v,value:v}))} onChange={e=>setForm({...form,action:e.value})}/>
       <InputNumber aria-label="Priority" value={form.priority} onValueChange={e=>setForm({...form,priority:e.value ?? 100})}/>
       <Button label="Add condition" outlined icon="pi pi-plus" onClick={add}/>
     </div>
     <p>All conditions must match (AND). Larger priority runs first; UUID breaks ties. No match means DENY. INSPECT is not currently a TLS intercept capability.</p>
     {form.conditions.map((c,i)=><div className="flex flex-wrap gap-2 align-items-center" key={i} style={{marginBottom:8}}>
       <Dropdown aria-label={'Condition '+(i+1)} value={c.type} options={options} onChange={e=>change(i,{type:e.value,value:e.value==='destination_port'?443:e.value==='header'?{name:'',value_regex:''}:e.value==='time_window'?{start_utc:'09:00',end_utc:'17:00'}:''})}/>
       {c.type==='header'?<>
         <InputText placeholder="Header name" value={(c.value as {name:string,value_regex:string}).name} onChange={e=>change(i,{...c,value:{...(c.value as {name:string,value_regex:string}),name:e.target.value}})}/>
         <InputText placeholder="Header regex" value={(c.value as {name:string,value_regex:string}).value_regex} onChange={e=>change(i,{...c,value:{...(c.value as {name:string,value_regex:string}),value_regex:e.target.value}})}/>
       </>:c.type==='time_window'?<>
         <InputText placeholder="Start UTC HH:MM" value={(c.value as {start_utc:string,end_utc:string}).start_utc} onChange={e=>change(i,{...c,value:{...(c.value as {start_utc:string,end_utc:string}),start_utc:e.target.value}})}/>
         <InputText placeholder="End UTC HH:MM" value={(c.value as {start_utc:string,end_utc:string}).end_utc} onChange={e=>change(i,{...c,value:{...(c.value as {start_utc:string,end_utc:string}),end_utc:e.target.value}})}/>
       </>:c.type==='destination_port'?
         <InputNumber value={Number(c.value)} min={1} max={65535} onValueChange={e=>change(i,{...c,value:e.value??443})}/>:
         <InputText placeholder="Condition value" value={String(c.value)} onChange={e=>change(i,{...c,value:e.target.value})}/>
       }
       <Button icon="pi pi-trash" severity="danger" outlined aria-label="Remove condition" onClick={()=>removeCondition(i)}/>
     </div>)}
     <Button label="Create policy" disabled={busy || !form.conditions.length} onClick={()=>void submit()}/>
   </Card>
   <Card title="Ordered policies" className="col-12">
     <Button label="Refresh" icon="pi pi-refresh" outlined onClick={()=>void reload()}/>
     <DataTable value={policies} dataKey="id" paginator rows={15} sortField="priority" sortOrder={-1}>
       <Column field="priority" header="Priority" sortable/>
       <Column field="name" header="Policy"/>
       <Column field="action" header="Action"/>
       <Column header="Conditions" body={(p:Policy)=>p.conditions.map(c=>c.type).join(', ')}/>
       <Column header="Delete" body={(p:Policy)=><Button label="Delete" outlined severity="danger" disabled={busy} onClick={()=>void drop(p.id)}/>}/>
     </DataTable>
   </Card>
 </div>;
}
