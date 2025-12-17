const API_URL = import.meta.env.VITE_API_URL || 'http://127.0.0.1:3000';

export const api = {
  async get(path) {
    const response = await fetch(`${API_URL}${path}`);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    // Only parse JSON if there's content
    const text = await response.text();
    return text ? JSON.parse(text) : null;
  },

  async post(path, body) {
    const response = await fetch(`${API_URL}${path}`, {
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
