import { getStoreValue, setStoreValue } from '../services/persistence';
import { publish } from './event-bus';
import type { TimeZoneEntry } from '../types/timezone';

const MAX_ZONES = 30;
let zones: TimeZoneEntry[] = [];

export async function loadClocks(): Promise<void> {
  zones = await getStoreValue<TimeZoneEntry[]>('zones.dat', 'zones', []);
  publish('CLOCKS_CHANGED', zones);
}

export function getClocks(): TimeZoneEntry[] {
  return zones;
}

export async function addClock(zone: TimeZoneEntry): Promise<void> {
  if (zones.length >= MAX_ZONES) return;
  zones.push(zone);
  await saveClocks();
}

export async function removeClock(id: string): Promise<void> {
  zones = zones.filter(z => z.id !== id);
  await saveClocks();
}

export async function reorderClock(id: string, newIndex: number): Promise<void> {
  const currentIndex = zones.findIndex(z => z.id === id);
  if (currentIndex === -1) return;
  const [zone] = zones.splice(currentIndex, 1);
  zones.splice(newIndex, 0, zone);
  
  zones.forEach((z, idx) => z.sort_order = idx);
  await saveClocks();
}

export async function updateLabel(id: string, newLabel: string): Promise<void> {
  const zone = zones.find(z => z.id === id);
  if (zone) {
    zone.display_label = newLabel;
    await saveClocks();
  }
}

async function saveClocks() {
  await setStoreValue('zones.dat', 'zones', zones);
  publish('CLOCKS_CHANGED', zones);
}
