import { Store, load } from '@tauri-apps/plugin-store';

const stores: Record<string, Store> = {};
const debouncers: Record<string, number> = {};

export async function initStore(name: string): Promise<Store> {
  if (!stores[name]) {
    stores[name] = await load(name);
  }
  return stores[name];
}

export async function getStoreValue<T>(storeName: string, key: string, defaultValue: T): Promise<T> {
  const store = await initStore(storeName);
  const value = await store.get<T>(key);
  return value !== null && value !== undefined ? value : defaultValue;
}

export async function setStoreValue<T>(storeName: string, key: string, value: T): Promise<void> {
  const store = await initStore(storeName);
  await store.set(key, value);
  debounceSave(storeName);
}

function debounceSave(storeName: string) {
  if (debouncers[storeName]) {
    window.clearTimeout(debouncers[storeName]);
  }
  debouncers[storeName] = window.setTimeout(async () => {
    const store = stores[storeName];
    if (store) {
      await store.save();
    }
  }, 300);
}

export async function runMigrations(
  storeName: string,
  currentSchemaVersion: number,
  migrations: Record<number, (store: Store) => Promise<void>>
) {
  const store = await initStore(storeName);
  const storedVersion = await store.get<number>('schema_version') || 0;

  if (storedVersion < currentSchemaVersion) {
    for (let version = storedVersion + 1; version <= currentSchemaVersion; version++) {
      if (migrations[version]) {
        try {
          await migrations[version](store);
        } catch (error) {
          console.error(`Migration to version ${version} failed for store ${storeName}:`, error);
          throw error;
        }
      }
    }
    await store.set('schema_version', currentSchemaVersion);
    await store.save();
  }
}

export const getZonesStore = () => initStore('zones.dat');
export const getAlarmsStore = () => initStore('alarms.dat');
export const getPreferencesStore = () => initStore('preferences.dat');
