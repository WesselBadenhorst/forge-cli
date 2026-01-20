# The Tech Stack

Web-first frontend + thin wrappers for everything else

## Foundation
Stack decisions (based on everything you’ve said):

### Frontend

React + TypeScript

Vite (fast, minimal config, hot reload)

TailwindCSS (or vanilla CSS modules if you prefer strict control)

### Backend

Django (Python) — you already have session auth and experience

Django REST Framework for API endpoints

Frontend → Backend communication: REST (simplest, no GraphQL overhead)

### Targets

Web → primary

Desktop → Tauri

Mobile (iOS / Android) → Capacitor
