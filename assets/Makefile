.PHONY: help dev backend migrate superuser collectstatic prod-update frontend build

help:
	@echo ""
	@echo "Forge project targets:"
	@echo ""
	@echo "  make dev				Run backend + frontend (devs)"
	@echo "  make backend			Run Django dev server"
	@echo "  make frontend			Run frontend dev server"
	@echo "  make migrate			Run Django migrations"
	@echo "  make superuser			Create Django superuser"
	@echo "  make collectstatic		Collect Django static files"
	@echo ""
	@echo "Production:"
	@echo "		make prod-update	Pull code, rebuild assets, migrate DB"
	@echo ""


# --- Development ---

backend:
	(cd backend && uv run python manage.py runserver)

frontend:
	(cd frontend && npm run dev)

dev:
	@echo "Starting backend + frontend"
	(make -j2 backend frontend)

migrate:
	(cd backend && uv run python manage.py makemigrations)
	(cd backend && uv run python manage.py migrate)

superuser:
	(cd backend && uv run python manage.py createsuperuser)

collectstatic:
	(cd backend && uv run python manage.py collectstatic --noinput)

build:
	(cd frontend && npm run build)

# --- Production ---

prod-update:
	@echo "🔄 Updating production environment..."
	git pull
	(cd frontend && npm install && npm run build)
	(cd backend && uv sync)
	(cd backend && uv run python manage.py makemigrations)
	(cd backend && uv run python manage.py migrate)
	(cd backend && uv run python manage.py collectstatic --noinput)
	@echo ""
	@echo "✅ Update complete."
	@echo "👉 Restart your supervisor services for changes to take effect."
