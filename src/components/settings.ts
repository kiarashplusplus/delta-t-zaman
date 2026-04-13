import { getPreferences, updatePreferences } from '../state/preferences';
import type { UserPreferences } from '../types/preferences';

export class SettingsPanel {
  public element: HTMLElement;

  constructor(container: HTMLElement) {
    this.element = document.createElement('div');
    this.element.className = 'settings-panel';
    
    this.render();
    container.appendChild(this.element);
  }

  private render() {
    const prefs = getPreferences();

    this.element.innerHTML = `
      <div class="settings-section">
        <h3>Display & General</h3>
        
        <label>
          <span>Theme</span>
          <select id="setting-theme">
            <option value="system" ${prefs.theme === 'system' ? 'selected' : ''}>System</option>
            <option value="light" ${prefs.theme === 'light' ? 'selected' : ''}>Light</option>
            <option value="dark" ${prefs.theme === 'dark' ? 'selected' : ''}>Dark</option>
          </select>
        </label>
        
        <label>
          <span>High Contrast</span>
          <select id="setting-high-contrast">
            <option value="system" ${prefs.high_contrast === 'system' ? 'selected' : ''}>System</option>
            <option value="on" ${prefs.high_contrast === 'on' ? 'selected' : ''}>On</option>
            <option value="off" ${prefs.high_contrast === 'off' ? 'selected' : ''}>Off</option>
          </select>
        </label>

        <label>
          <span>Time Format</span>
          <select id="setting-time-format">
            <option value="12h" ${prefs.time_format === '12h' ? 'selected' : ''}>12-hour</option>
            <option value="24h" ${prefs.time_format === '24h' ? 'selected' : ''}>24-hour</option>
          </select>
        </label>

        <label>
          <input type="checkbox" id="setting-always-on-top" ${prefs.always_on_top ? 'checked' : ''} />
          <span>Always on Top</span>
        </label>

        <label>
          <input type="checkbox" id="setting-autostart" ${prefs.autostart ? 'checked' : ''} />
          <span>Launch on Startup</span>
        </label>
      </div>

      <div class="settings-section">
        <h3>Notifications</h3>
        <p><em>Notification preferences coming soon.</em></p>
      </div>

      <div class="settings-section">
        <h3>Privacy</h3>
        <label>
          <input type="checkbox" id="setting-telemetry" ${prefs.telemetry_opt_in ? 'checked' : ''} />
          <span>Allow anonymous crash reports and analytics</span>
        </label>
      </div>

      <div class="settings-section">
        <h3>Updates</h3>
        <label>
          <input type="checkbox" id="setting-auto-update" ${prefs.auto_update ? 'checked' : ''} />
          <span>Check for updates automatically</span>
        </label>
      </div>

      <div class="settings-section">
        <h3>Data</h3>
        <div style="display: flex; gap: 1rem;">
          <button id="btn-export-data">Export Data</button>
          <button id="btn-import-data">Import Data</button>
        </div>
      </div>
    `;

    this.attachListeners();
  }

  private attachListeners() {
    const bindSelect = (id: string, key: keyof UserPreferences) => {
      const el = this.element.querySelector(`#${id}`) as HTMLSelectElement;
      el?.addEventListener('change', () => {
        updatePreferences({ [key]: el.value } as any);
      });
    };

    const bindCheck = (id: string, key: keyof UserPreferences) => {
      const el = this.element.querySelector(`#${id}`) as HTMLInputElement;
      el?.addEventListener('change', () => {
        updatePreferences({ [key]: el.checked } as any);
      });
    };

    bindSelect('setting-theme', 'theme');
    bindSelect('setting-high-contrast', 'high_contrast');
    bindSelect('setting-time-format', 'time_format');
    bindCheck('setting-always-on-top', 'always_on_top');
    bindCheck('setting-autostart', 'autostart');
    bindCheck('setting-telemetry', 'telemetry_opt_in');
    bindCheck('setting-auto-update', 'auto_update');

    this.element.querySelector('#btn-export-data')?.addEventListener('click', () => {
      console.log('Export data clicked');
      // Placeholder for Phase 9
    });

    this.element.querySelector('#btn-import-data')?.addEventListener('click', () => {
      console.log('Import data clicked');
      // Placeholder for Phase 9
    });
  }

  destroy() {
    this.element.remove();
  }
}
