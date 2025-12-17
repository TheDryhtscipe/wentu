# Wentu - Quick Start Guide

## One-Time Setup (5 minutes)

### 1. Install Dependencies
```bash
# If you don't have Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# If you don't have Node.js 18+
# Download from https://nodejs.org/

# If you don't have PostgreSQL
# macOS: brew install postgresql
# Ubuntu: sudo apt-get install postgresql
```

### 2. Create Database
```bash
sudo -u postgres psql -c "CREATE DATABASE wentu;"
```

### 3. Install Dependencies
```bash
# Backend
cd backend
cargo fetch

# Frontend
cd ../frontend
npm install
```

Done! ✅

---

## Running the App

### Terminal 1: Backend
```bash
cd backend
cargo run
# Listens on http://127.0.0.1:3000
```

### Terminal 2: Frontend
```bash
cd frontend
npm run dev
# Listens on http://127.0.0.1:5173
```

### Terminal 3 (optional): Open Browser
```bash
open http://127.0.0.1:5173
# Or just visit that URL
```

---

## Usage

### Create a Meeting
1. Click **"Create new"** on home page
2. Fill in title, your name, dates
3. Click **"Create Wentu"**
4. Share the code with others

### Join a Meeting
1. Enter the code from the creator
2. Click **"Join"**
3. Enter your name
4. Drag-drop dates to order preferences (best first)
5. Click **"Submit preferences"**
6. See live STV results!

---

## Testing

Run all API tests:
```bash
bash test_api.sh
```

Run Rust unit tests:
```bash
cd backend
cargo test
```

---

## Troubleshooting

### Port Already in Use
```bash
# Find and kill process on port 3000
lsof -i :3000 | grep LISTEN | awk '{print $2}' | xargs kill -9

# Same for port 5173
lsof -i :5173 | grep LISTEN | awk '{print $2}' | xargs kill -9
```

### Database Connection Failed
```bash
# Check PostgreSQL is running
sudo -u postgres psql -c "SELECT 1"

# Verify database exists
sudo -u postgres psql -l | grep wentu

# Check backend .env file has correct URL
cat backend/.env
```

### Frontend Won't Connect to Backend
1. Verify backend is running on `:3000`
2. Check browser console for CORS errors
3. Try `curl http://127.0.0.1:3000/health`

### Stale Cache
```bash
# Backend
cd backend && cargo clean && cargo build

# Frontend
cd frontend && rm -rf node_modules/.vite && npm run dev
```

---

## Command Cheatsheet

```bash
# Start fresh (cleanup all)
pkill -f "wentu_backend" || true
pkill -f "vite" || true
rm -rf backend/target frontend/node_modules frontend/dist

# Backend only
cd backend && cargo run

# Frontend only (requires running backend)
cd frontend && npm run dev

# Build for production
cd backend && cargo build --release  # → backend/target/release/wentu_backend
cd frontend && npm run build         # → frontend/dist/

# Run tests
cd backend && cargo test
bash test_api.sh

# Database reset
sudo -u postgres psql -d wentu -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
# Then restart backend to re-run migrations
```

---

## Performance Tips

- **Frontend**: Uses system fonts (no download delays)
- **Backend**: Connection pooling limits DB overhead
- **Database**: Indexes on slug, FK, expiry for fast lookups
- **Build**: Frontend bundle is ~80KB gzipped

---

## Architecture

```
┌──────────────────┐
│   Browser UI     │
│  (Svelte/Vite)  │
└────────┬─────────┘
         │ JSON/HTTP
         ↓
┌──────────────────┐
│   Rust Backend   │
│   (Axum/Tokio)   │
└────────┬─────────┘
         │ SQL
         ↓
┌──────────────────┐
│   PostgreSQL     │
│   (wentus DB)    │
└──────────────────┘
```

---

## Environment Variables

### Backend (.env)
```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/wentu
RUST_LOG=info
```

### Frontend (none needed)
Frontend talks to `http://127.0.0.1:3000` hardcoded.

---

## Key Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| POST | `/api/wentu` | Create |
| GET | `/api/wentu/:slug` | Get details |
| POST | `/api/wentu/:slug/join` | Join |
| POST | `/api/wentu/:slug/preferences` | Vote |
| GET | `/api/wentu/:slug/stv-results` | Results |

Full docs: See `PLAN.md` and `IMPLEMENTATION_SUMMARY.md`

---

## Next Steps

- **Single Transferable Vote**: Read about it [here](https://en.wikipedia.org/wiki/Single_transferable_vote)
- **Deploy**: See `IMPLEMENTATION_SUMMARY.md` Phase 5
- **Contribute**: Fork the repo, submit PRs
- **Feedback**: Create issues on GitHub

---

Enjoy meeting scheduling! 🎉
