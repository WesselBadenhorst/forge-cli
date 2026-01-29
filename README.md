# Forge 🔥

Forge is a personal scaffolding tool for building small, profitable SaaS products quickly — without overengineering or framework churn.

It is opinionated, boring, and designed for solo developers who want **leverage**, not novelty.

---

## Why Forge Exists

After 20+ years of shipping software, the bottleneck is no longer:
- languages
- frameworks
- frontend trends

The bottleneck is:
- repetition
- setup tax
- forgetting what “good defaults” look like

Forge exists to make sure **work stays done**.

---

## What Forge Generates

Forge creates a full-stack project with:

### Backend
- **Python + Django**
- **Django REST Framework**
- **django-allauth** (email + Google auth ready)
- Environment-based settings:
  - `base.py`
  - `dev.py` (SQLite)
  - `prod.py` (PostgreSQL)
- `.env` + `.env.example`
- Proper middleware + auth wiring
- Clean settings layout from day one

### Frontend
- **Vite + React + TypeScript**
- No native-look obsession
- Consistent UI across platforms
- Designed to build once and deploy everywhere

### Tooling
- **uv** for Python dependency management
- **npm** for frontend
- **Makefile** at repo root:
  - backend
  - frontend
  - dev
  - prod update (future)
- No containers by default
- No clusters
- No “magic scale”

A $5 Linode is enough until it isn’t.

---

## Philosophy

Forge is intentionally:

- ❌ Not framework-agnostic
- ❌ Not enterprise-ready
- ❌ Not trying to be universal

It **is**:
- reproducible
- understandable
- debuggable
- fast to iterate on

Perfect for:
- micro-SaaS
- B2C tools
- internal products
- learning distribution + marketing

---

## Installation

### One-line install

```bash
curl -fsSL https://raw.githubusercontent.com/WesselBadenhorst/forge-cli/main/install.sh | sh
```

### Verify:

```bash
forge --help
```

---

## Usage

### Create a new project

```bash
forge my_app
```

### Or initialize in the current directory

```bash
forge .
```

#### Forge will:

- create backend + frontend
- wire Django settings correctly
- inject auth + middleware
- generate env files
- install dependencies (unless --no-install)

---

## Development

```bash
cd my_app
make dev
```

Backend and frontend run side-by-side.

---

## Non-goals (Imporant)

Forge deliberately avoids:

- Kubernetes
- Docker-first workflows
- Microservices
- Framework churn
- Cross-compiling fantasies

Those can be added later - __once the product earns it.__

---

## Status

Forge is evolving alongside real projects.

Breaking changes may happen.

That's a feature.

---

## License

### MIT

