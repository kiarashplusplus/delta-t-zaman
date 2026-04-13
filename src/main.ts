import { loadClocks, getClocks, addClock } from './state/clocks';
import { loadPreferences, getPreferences } from './state/preferences';
import { startTimeTicker } from './state/time';
import { ClockList } from './components/clock-list';
import { TimezoneSearchModal } from './components/timezone-search';
import { getSystemInfo, restoreWindowState, getPlatformCloseBehavior, hideToTray, showFromTray, toggleCompactMode } from './services/ipc';
import timezonesData from './data/timezones.json';
import type { TimeZoneEntry } from './types/timezone';
import { setupTrayUpdates } from './components/tray-preview';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { CompactView } from './components/compact-view';
import { initAlarms, handleAlarmFired, handleAlarmSnoozed, getAlarm } from './state/alarms';
import { AlarmList } from './components/alarm-list';
import { SettingsPanel } from './components/settings';
import { AboutPanel } from './components/about';

function applyTheme(prefs: any) {
  const isDark = prefs.theme === 'dark' || (prefs.theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
  document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
}

function applyHighContrast(prefs: any) {
  const isHighContrast = prefs.high_contrast === 'on' || (prefs.high_contrast === 'system' && window.matchMedia('(prefers-contrast: more)').matches);
  if (isHighContrast) {
    document.documentElement.setAttribute('data-theme', 'high-contrast');
  }
}

window.onerror = (message, source, lineno, colno, error) => {
  console.error("Global Error:", { message, source, lineno, colno, error });
};

window.onunhandledrejection = (event) => {
  console.error("Unhandled Promise Rejection:", event.reason);
};

function showInAppBanner(title: string, alarmId: string) {
  let container = document.getElementById('toast-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'toast-container';
    document.body.appendChild(container);
  }

  const toast = document.createElement('div');
  toast.className = 'toast banner';
  toast.style.background = 'var(--color-primary, #005fcc)';
  toast.style.color = 'white';
  toast.style.padding = '1rem';
  toast.style.borderRadius = '8px';
  toast.style.boxShadow = '0 4px 6px rgba(0,0,0,0.1)';
  toast.style.display = 'flex';
  toast.style.justifyContent = 'space-between';
  toast.style.alignItems = 'center';
  toast.style.gap = '1rem';
  toast.style.marginBottom = '0.5rem';

  const text = document.createElement('div');
  text.textContent = `⏰ ${title}`;

  const actions = document.createElement('div');
  actions.style.display = 'flex';
  actions.style.gap = '0.5rem';

  const snoozeBtn = document.createElement('button');
  snoozeBtn.textContent = 'Snooze';
  snoozeBtn.onclick = async () => {
    await handleAlarmSnoozed(alarmId);
    toast.remove();
  };

  const dismissBtn = document.createElement('button');
  dismissBtn.textContent = 'Dismiss';
  dismissBtn.onclick = () => {
    toast.remove();
  };

  actions.appendChild(snoozeBtn);
  actions.appendChild(dismissBtn);

  toast.appendChild(text);
  toast.appendChild(actions);

  container.appendChild(toast);
}

async function initializeApp() {
  try {
    const sysInfo = await getSystemInfo();
    
    await loadPreferences();
    await loadClocks();
    await initAlarms();

    const prefs = getPreferences();
    applyTheme(prefs);
    applyHighContrast(prefs);

    const zones = getClocks();
    if (zones.length === 0) {
      const localTz = sysInfo.timezone || Intl.DateTimeFormat().resolvedOptions().timeZone;
      const localMeta = (timezonesData as any[]).find(t => t.id === localTz) || { id: localTz, city: localTz };
      
      const localZone: TimeZoneEntry = {
        id: crypto.randomUUID(),
        iana_id: localTz,
        display_label: localMeta.city || localTz,
        sort_order: 0,
        pinned_to_tray: false,
        created_at: Date.now()
      };
      await addClock(localZone);

      const utcZone: TimeZoneEntry = {
        id: crypto.randomUUID(),
        iana_id: 'UTC',
        display_label: 'UTC',
        sort_order: 1,
        pinned_to_tray: false,
        created_at: Date.now()
      };
      await addClock(utcZone);

      const contrastingTz = sysInfo.locale.startsWith('en-US') ? 'Europe/London' : 'Asia/Tokyo';
      const contrastMeta = (timezonesData as any[]).find(t => t.id === contrastingTz) || { id: contrastingTz, city: contrastingTz };
      const contrastZone: TimeZoneEntry = {
        id: crypto.randomUUID(),
        iana_id: contrastingTz,
        display_label: contrastMeta.city || contrastingTz,
        sort_order: 2,
        pinned_to_tray: false,
        created_at: Date.now()
      };
      await addClock(contrastZone);
    }

    document.body.innerHTML = '';
    const appContainer = document.createElement('div');
    appContainer.id = 'app';
    document.body.appendChild(appContainer);

    // Setup tabs
    const tabsContainer = document.createElement('div');
    tabsContainer.className = 'tabs-container';
    
    const tabClocks = document.createElement('button');
    tabClocks.className = 'tab active';
    tabClocks.textContent = 'Clocks';
    
    const tabAlarms = document.createElement('button');
    tabAlarms.className = 'tab';
    tabAlarms.textContent = 'Alarms';

    const tabSettings = document.createElement('button');
    tabSettings.className = 'tab';
    tabSettings.textContent = 'Settings';

    tabsContainer.appendChild(tabClocks);
    tabsContainer.appendChild(tabAlarms);
    tabsContainer.appendChild(tabSettings);
    appContainer.appendChild(tabsContainer);

    const contentArea = document.createElement('div');
    contentArea.className = 'content-area';
    appContainer.appendChild(contentArea);

    let touchStartX = 0;
    let touchStartY = 0;
    let currentTarget: HTMLElement | null = null;
    let originalTransform = '';

    contentArea.addEventListener('touchstart', (e: TouchEvent) => {
      const target = (e.target as HTMLElement).closest('.clock-card, .alarm-card') as HTMLElement;
      if (target) {
        touchStartX = e.touches[0].clientX;
        touchStartY = e.touches[0].clientY;
        currentTarget = target;
        originalTransform = target.style.transform || '';
        target.style.transition = 'none';
      }
    }, { passive: true });

    contentArea.addEventListener('touchmove', (e: TouchEvent) => {
      if (!currentTarget) return;
      const touchEndX = e.touches[0].clientX;
      const touchEndY = e.touches[0].clientY;
      const deltaX = touchEndX - touchStartX;
      const deltaY = touchEndY - touchStartY;
      
      if (Math.abs(deltaY) > Math.abs(deltaX)) {
        currentTarget.style.transform = originalTransform;
        currentTarget = null;
        return;
      }
      
      if (deltaX < 0) {
        currentTarget.style.transform = `translateX(${deltaX}px)`;
        if (e.cancelable) e.preventDefault();
      }
    }, { passive: false });

    contentArea.addEventListener('touchend', (e: TouchEvent) => {
      if (!currentTarget) return;
      const touchEndX = e.changedTouches[0].clientX;
      const deltaX = touchEndX - touchStartX;
      
      currentTarget.style.transition = 'transform 0.3s ease';
      
      if (deltaX < -100) {
        currentTarget.style.transform = `translateX(-100%)`;
        const targetElement = currentTarget;
        setTimeout(() => {
          let deleteBtn = targetElement.querySelector('.clock-card__delete') as HTMLButtonElement;
          if (!deleteBtn) {
            const buttons = Array.from(targetElement.querySelectorAll('button'));
            deleteBtn = buttons.find(b => b.textContent === 'Delete') as HTMLButtonElement;
          }
          if (deleteBtn) {
            deleteBtn.click();
          } else {
            targetElement.style.transform = 'translateX(0)';
          }
        }, 300);
      } else {
        currentTarget.style.transform = 'translateX(0)';
      }
      
      currentTarget = null;
    });

    const clocksPanel = document.createElement('div');
    clocksPanel.className = 'panel';
    const alarmsPanel = document.createElement('div');
    alarmsPanel.className = 'panel hidden';
    const settingsPanelContainer = document.createElement('div');
    settingsPanelContainer.className = 'panel hidden';

    contentArea.appendChild(clocksPanel);
    contentArea.appendChild(alarmsPanel);
    contentArea.appendChild(settingsPanelContainer);

    let currentView: ClockList | CompactView = new ClockList(clocksPanel);
    new AlarmList(alarmsPanel);
    new SettingsPanel(settingsPanelContainer);
    new AboutPanel(settingsPanelContainer);

    new TimezoneSearchModal();

    tabClocks.addEventListener('click', () => {
      tabClocks.classList.add('active');
      tabAlarms.classList.remove('active');
      tabSettings.classList.remove('active');
      clocksPanel.classList.remove('hidden');
      alarmsPanel.classList.add('hidden');
      settingsPanelContainer.classList.add('hidden');
    });

    tabAlarms.addEventListener('click', () => {
      tabAlarms.classList.add('active');
      tabClocks.classList.remove('active');
      tabSettings.classList.remove('active');
      alarmsPanel.classList.remove('hidden');
      clocksPanel.classList.add('hidden');
      settingsPanelContainer.classList.add('hidden');
    });

    tabSettings.addEventListener('click', () => {
      tabSettings.classList.add('active');
      tabClocks.classList.remove('active');
      tabAlarms.classList.remove('active');
      settingsPanelContainer.classList.remove('hidden');
      clocksPanel.classList.add('hidden');
      alarmsPanel.classList.add('hidden');
    });

    startTimeTicker();

    try {
      await restoreWindowState();
    } catch (e) {
      console.warn("Failed to restore window state:", e);
    }

    setupTrayUpdates();

    const appWindow = getCurrentWindow();
    const closeBehavior = await getPlatformCloseBehavior();

    appWindow.onCloseRequested(async (event) => {
      if (closeBehavior.close_action === 'hide_to_tray' || closeBehavior.close_action === 'suspend') {
        event.preventDefault();
        await hideToTray();
      }
    });

    listen('tray-action', async (event) => {
      const action = event.payload as string;
      if (action === 'show') {
        await showFromTray();
      } else if (action === 'toggle_compact') {
        const isCompact = document.body.classList.contains('compact-mode');
        const nextState = !isCompact;
        
        if (nextState) {
          document.body.classList.add('compact-mode');
          currentView.destroy?.();
          clocksPanel.innerHTML = '';
          currentView = new CompactView(clocksPanel);
        } else {
          document.body.classList.remove('compact-mode');
          currentView.destroy?.();
          clocksPanel.innerHTML = '';
          currentView = new ClockList(clocksPanel);
        }
        
        await toggleCompactMode(nextState);
      }
    });

    listen('alarm-fired', async (event: any) => {
      const payload = event.payload as any; // { alarm_id: string }
      if (payload && payload.alarm_id) {
        const alarm = getAlarm(payload.alarm_id);
        const title = alarm ? alarm.title : 'Alarm';
        await handleAlarmFired(payload.alarm_id);
        
        // Show in-app banner if foreground
        const isFocused = await appWindow.isFocused();
        if (isFocused || document.hasFocus()) {
          showInAppBanner(title, payload.alarm_id);
        }
      }
    });

    listen('alarm-snoozed', async (event: any) => {
      const payload = event.payload as any; // { alarm_id: string }
      if (payload && payload.alarm_id) {
        await handleAlarmSnoozed(payload.alarm_id);
      }
    });

    // Also listen to internal 'alarm-fired' if we publish it directly from state check on startup
    // wait, `state/alarms.ts` does: `publish('alarm-fired', { id: alarm.id })`
    // Wait, the state `event-bus.ts` publish is different from `tauri` listen.
    // I need to import subscribe from event-bus and subscribe to it as well!
    const { subscribe } = await import('./state/event-bus');
    subscribe('alarm-fired', async (data: any) => {
      if (data && data.id) {
        const alarm = getAlarm(data.id);
        const title = alarm ? alarm.title : 'Alarm';
        await handleAlarmFired(data.id);
        showInAppBanner(title, data.id);
      }
    });

    listen('webview-heartbeat', (event: any) => {
      if (event.payload && typeof event.payload.sequence === 'number') {
        invoke('heartbeat_ack', { sequence: event.payload.sequence }).catch(console.error);
      }
    });

    listen('deep-link-received', async (event: any) => {
      if (event.payload && event.payload.url) {
        try {
          const result = await invoke('handle_deep_link', { url: event.payload.url });
          console.log("Deep link result:", result);
        } catch (e) {
          console.error("Failed to handle deep link:", e);
        }
      }
    });

    invoke('check_for_update').catch(e => console.warn("Update check failed:", e));
    invoke('check_timezone_data_update').catch(e => console.warn("Timezone data update check failed:", e));

    console.log("App initialized successfully");
  } catch (error) {
    console.error("Plugin initialization failed:", error);
    document.body.innerHTML = `<div style="color: red; padding: 20px;">Failed to initialize app: ${error}</div>`;
  }
}

document.addEventListener("DOMContentLoaded", () => {
  initializeApp();
});
