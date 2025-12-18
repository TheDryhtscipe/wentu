const API_URL = import.meta.env.VITE_API_URL || '';
const BASE_URL = API_URL.endsWith('/') ? API_URL.slice(0, -1) : API_URL;

function buildUrl(path) {
  if (!BASE_URL) return path;
  if (BASE_URL.endsWith('/api') && path.startsWith('/api/')) {
    return `${BASE_URL}${path.slice(4)}`;
  }
  return `${BASE_URL}${path}`;
}

export const api = {
  async get(path) {
    const response = await fetch(buildUrl(path));
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    // Only parse JSON if there's content
    const text = await response.text();
    return text ? JSON.parse(text) : null;
  },

  async post(path, body) {
    const response = await fetch(buildUrl(path), {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    // Only parse JSON if there's content
    const text = await response.text();
    return text ? JSON.parse(text) : null;
  },
};
