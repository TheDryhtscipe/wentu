import { writable } from 'svelte/store';

const toasts = writable([]);

function add(message, variant, duration) {
  const id = crypto.randomUUID();
  toasts.update(list => [...list, { id, message, variant, duration }]);
  return () => remove(id);
}

function remove(id) {
  toasts.update(list => list.filter(t => t.id !== id));
}

export const toast = {
  success: (message, duration = 4000) => add(message, 'success', duration),
  error: (message, duration = 6000) => add(message, 'error', duration),
  info: (message, duration = 4000) => add(message, 'info', duration),
  dismiss: remove,
  subscribe: toasts.subscribe,
};
