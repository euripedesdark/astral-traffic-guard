# Astral Traffic Guard – Documento de Referência e Atualizações

**Documento Vivo – Versão 2.0**
**Última atualização:** 21 DE OUTUBRO DE 2026

---

## 📅 LINHA DO TEMPO DE ATUALIZAÇÕES

### **21 DE OUTUBRO DE 2026 – Versão 2.0 (Desacoplamento como Solução Autônoma de Proxy)**
**Migração e Isolamento do Módulo Astral Traffic Guard**:
- Projeto desacoplado da antiga base monolitica e configurado como serviço autônomo de Proxy e Filtro Web.
- **Portas Padronizadas**:
  - `:8085`: API REST de administração e interface web do Proxy/ACL.
  - `:8091`: Socket local `ProxyAuthorizationServer` para autenticação do ATS (`authproxy.so`).
- **Camada de Cache**: Otimização do `AclCacheService` via Redis com fallback transparente para PostgreSQL.
- **Frontend Exclusivo**: Interface especializada para controle de categorias de navegação, domínios permitidos/bloqueados e auditoria de requisições.
