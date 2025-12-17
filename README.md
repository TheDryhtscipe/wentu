# Wentu - Meeting Scheduler with STV Voting

A lightweight, ephemeral web app for scheduling meetings using Single Transferable Vote (STV). No accounts, no persistence—everything expires.

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
2. Set quota: `(total_votes / 2) + 1` (majority for 1 seat)
3. If winner reaches quota, done
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

MIT

---

**Note**: Wentu is designed for simplicity and ephemeral data. It's not suitable for sensitive information or long-term record-keeping. Everything expires.
