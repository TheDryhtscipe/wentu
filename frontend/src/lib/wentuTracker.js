/**
 * Utility module for tracking Wentus in localStorage
 * Stores list of Wentus user has created or participated in
 */

const STORAGE_KEY = 'wentu-tracked';

/**
 * Get all tracked Wentus, sorted by timestamp (newest first)
 * @returns {Array} Array of tracked Wentu objects
 */
export function getTrackedWentus() {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) {
      return [];
    }
    const wentus = JSON.parse(stored);
    // Sort by timestamp descending (newest first)
    return wentus.sort((a, b) => b.timestamp - a.timestamp);
  } catch (err) {
    console.error('Failed to get tracked Wentus:', err);
    return [];
  }
}

/**
 * Add or update a tracked Wentu
 * Prevents duplicates by slug and prioritizes "owner" role
 * @param {string} slug - Wentu slug identifier
 * @param {string} title - Wentu title
 * @param {string} role - Either "owner" or "participant"
 * @param {string} [name] - Participant or creator name to remember
 * @param {string} [participantId] - Participant ID to reuse for updates
 * @param {string} [participantKey] - Participant key to reuse for updates
 */
export function addTrackedWentu(
  slug,
  title,
  role,
  name = '',
  participantId = '',
  participantKey = ''
) {
  try {
    const tracked = getTrackedWentus();
    const existingIndex = tracked.findIndex(w => w.slug === slug);
    const trimmedName = typeof name === 'string' ? name.trim() : '';
    const trimmedParticipantId = typeof participantId === 'string' ? participantId.trim() : '';
    const trimmedParticipantKey = typeof participantKey === 'string' ? participantKey.trim() : '';

    if (existingIndex !== -1) {
      // Update existing entry
      // Keep "owner" role if already owner (don't downgrade to participant)
      tracked[existingIndex].title = title;
      tracked[existingIndex].timestamp = Date.now();
      if (tracked[existingIndex].role !== 'owner') {
        tracked[existingIndex].role = role;
      }
      if (trimmedName) {
        tracked[existingIndex].name = trimmedName;
      }
      if (trimmedParticipantId) {
        tracked[existingIndex].participantId = trimmedParticipantId;
      }
      if (trimmedParticipantKey) {
        tracked[existingIndex].participantKey = trimmedParticipantKey;
      }
    } else {
      // Add new entry at the beginning
      tracked.unshift({
        slug,
        title,
        role,
        timestamp: Date.now(),
        name: trimmedName || undefined,
        participantId: trimmedParticipantId || undefined,
        participantKey: trimmedParticipantKey || undefined
      });
    }

    localStorage.setItem(STORAGE_KEY, JSON.stringify(tracked));
  } catch (err) {
    console.error('Failed to add tracked Wentu:', err);
    // Silent failure - app continues without tracking
  }
}

/**
 * Remove a Wentu from tracking
 * @param {string} slug - Wentu slug identifier to remove
 */
export function removeTrackedWentu(slug) {
  try {
    const tracked = getTrackedWentus();
    const filtered = tracked.filter(w => w.slug !== slug);
    localStorage.setItem(STORAGE_KEY, JSON.stringify(filtered));
  } catch (err) {
    console.error('Failed to remove tracked Wentu:', err);
  }
}

/**
 * Check if a Wentu is already being tracked
 * @param {string} slug - Wentu slug identifier
 * @returns {boolean} True if Wentu is tracked
 */
export function isWentuTracked(slug) {
  const tracked = getTrackedWentus();
  return tracked.some(w => w.slug === slug);
}

/**
 * Get a specific tracked Wentu by slug
 * @param {string} slug - Wentu slug identifier
 * @returns {object|null} Tracked Wentu entry or null
 */
export function getTrackedWentu(slug) {
  const tracked = getTrackedWentus();
  return tracked.find(w => w.slug === slug) || null;
}
