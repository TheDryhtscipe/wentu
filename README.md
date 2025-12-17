# Wentu - Meeting Scheduler with STV Voting

A lightweight, ephemeral web app for scheduling meetings using Single Transferable Vote (STV). No accounts, no data persistence, no third party storage.

## Features

- **Create wentus**: Set title, description, and date options
- **Share & join**: No registration, just share the wentu code
- **Drag-drop preferences**: Intuitively order dates by preference
- **STV voting**: Fair date selection algorithm eliminates lowest-voted options
- **Live expiry**: All data deleted after 7 days (default)
- **Dark red theme**: High-contrast design for accessibility
- **Lean code**: Minimal dependencies, focus on performance

## Tech Stack

### Backend
- **Axum** (async Rust web framework)
- **PostgreSQL** (with automatic expiry cleanup)
- **SQLx** (async SQL toolkit)
- **Tokio** (async runtime)

### Frontend
- **Svelte** (lean, reactive framework)
- **Vite** (fast bundler)
- **Tailwind CSS** (utility-first styling)
- **svelte-dnd-action** (drag-and-drop)

## Quick Start

### Prerequisites
- Rust & Cargo (install via https://rustup.rs/)
- Node.js 18+
- PostgreSQL 14+

### Setup

#### 1. Database
```bash
sudo -u postgres psql -c "CREATE DATABASE wentu;"
```

#### 2. Backend
```bash
cd backend
cp .env.example .env  # Edit with your DB URL
cargo run
# Server runs on http://127.0.0.1:3000
```

#### 3. Frontend
```bash
cd frontend
npm install
npm run dev
# App runs on http://127.0.0.1:5173
```

### Docker Compose Staging Stack

To spin up PostgreSQL + backend + frontend with a single command:

```bash
docker compose up -d --build
```

- Backend: http://localhost:3000  
- Frontend (nginx serving the Vite build): http://localhost:4173  
- Database: exposed on localhost:5434 (volume persisted as `wentu_postgres_data`)

Helpful commands:

```bash
docker compose ps                # check container status
docker compose logs backend -f   # follow backend logs
docker compose down              # stop everything, keep volume
docker compose down -v           # teardown and wipe postgres volume
```

The frontend image is built with `VITE_API_URL=http://backend:3000`; override via
`frontend.build.args` in `docker-compose.yml` if you need a different API endpoint.

## API Endpoints

### Wentu Management
- `POST /api/wentu` - Create new wentu
- `GET /api/wentu/:slug` - Get wentu details
- `POST /api/wentu/:slug/close` - Close poll early (creator only)

### Participation
- `POST /api/wentu/:slug/join` - Join as participant
- `POST /api/wentu/:slug/preferences` - Submit vote preferences

### Results
- `GET /api/wentu/:slug/stv-results` - Get current STV voting results

## Data Model

### Wentu
```
id: UUID
slug: String (unique, shareable code)
title: String
creator_name: String (no accounts)
created_at: DateTime
expires_at: DateTime
status: open | closed | expired
date_options: DateRange[]
```

### Participant
```
id: UUID
wentu_id: UUID (FK)
name: String
participant_key: String (secret token)
joined_at: DateTime
```

### Ranking (STV preferences)
```
participant_id: UUID (FK)
date_option_id: UUID (FK)
preference_order: Int (1 = first choice)
```

## STV Algorithm

1. Count first preferences for each date
2. Set quota: `(total_votes / 2) + 1`
3. If option reaches quota, that option is winner
4. Eliminate lowest-voted date, redistribute votes
5. Repeat until winner found

Example:
- 4 voters, 3 date options
- Quota = 3
- Round 1: Date A=2, Date B=1, Date C=1
- A doesn't reach quota, eliminate C
- B's voter had Date B as 2nd choice
- Round 2: Date A=2, Date B=2
- Still tied, eliminate lowest (arbitrary), A wins

## Color Palette

| Name | Color | Usage |
|------|-------|-------|
| Dark BG | #2B1313 | Page background |
| Content BG | #3C1A1A | Cards, inputs |
| Text Primary | #FCEDEA | Main text (13.6:1 vs content) |
| Text Secondary | #E6BEB4 | Secondary text (9.1:1 vs content) |
| Accent | #FF9C63 | Buttons, highlights (7.5:1 vs content) |
| Success | #63E6BE | Confirmation states |
| Error | #FF909F | Errors, warnings (7.2:1 vs content) |

Text/foreground colors keep at least a 7:1 contrast ratio against the content background.

## Development

### Running Tests
```bash
cd backend
cargo test
```

To exercise the full HTTP flow, run the root-level smoke test:

```bash
bash test_api.sh
```

The script now enforces rate-limit friendly pauses (`WRITE_DELAY` env var, default 0.8s) and gracefully handles endpoints that return empty bodies (e.g., preference updates, closing a wentu).

### Building for Production
```bash
# Backend
cd backend
cargo build --release

# Frontend
cd frontend
npm run build
```

### Environment Variables
Backend: `.env` file with `DATABASE_URL=postgres://...`

## Roadmap

None of this is definite, but these were stretch goals in early design docs & may be added later. Consider complexity to be directly correlated with likelihood of development: the more complex the feature, the less likely it'll arrive soon.

- [ ] WebSocket support for live vote updates
- [ ] Export results as calendar invite
- [ ] Multi-language support
- [ ] Mobile app (React Native)
- [ ] Advanced scheduling (timezone handling)
- [ ] Anonymous voting mode

## Contributing

1. Fork the repo
2. Create a feature branch
3. Submit a PR

## License

Unlicense
