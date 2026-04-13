import { getStoreValue, setStoreValue } from '../services/persistence';
import { publish } from './event-bus';
import type { UserPreferences } from '../types/preferences';
import { getSystemInfo } from '../services/ipc';

let preferences: UserPreferences | null = null;

const DEFAULT_PREFS: UserPreferences = {
  time_format: '12h',
  locale: 'en-US',
  theme: 'system',
  high_contrast: 'system',
  default_timezone: 'UTC',
  always_on_top: false,
  autostart: false,
  auto_update: true,
  compact_mode: false
};

export async function loadPreferences(): Promise<void> {
  const stored = await getStoreValue<Partial<UserPreferences>>('preferences.dat', 'preferences', {});
  
  if (Object.keys(stored).length === 0) {
    try {
      const sysInfo = await getSystemInfo();
      DEFAULT_PREFS.locale = sysInfo.locale;
      DEFAULT_PREFS.default_timezone = sysInfo.timezone;
    } catch (e) {
      console.warn("Could not fetch system info for default preferences", e);
    }
  }
  
  preferences = { ...DEFAULT_PREFS, ...stored };
  publish('PREFERENCES_CHANGED', preferences);
}

export function getPreferences(): UserPreferences {
  if (!preferences) throw new Error('Preferences not loaded');
  return preferences;
}

export async function updatePreferences(updates: Partial<UserPreferences>): Promise<void> {
  if (!preferences) return;
  preferences = { ...preferences, ...updates };
  await setStoreValue('preferences.dat', 'preferences', preferences);
  publish('PREFERENCES_CHANGED', preferences);
}
