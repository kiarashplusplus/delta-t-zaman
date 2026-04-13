import { type } from '@tauri-apps/plugin-os';

const osType = type();

export const is_mobile = osType === 'ios' || osType === 'android';
export const is_desktop = osType === 'macos' || osType === 'windows' || osType === 'linux';
export const supports_tray = is_desktop && osType !== 'linux'; // Appoximation

export function getPlatformType(): string {
  return osType;
}
