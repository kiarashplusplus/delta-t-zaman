import { publish } from '../state/event-bus';

export function registerGlobalShortcuts() {
  document.addEventListener('keydown', (e: KeyboardEvent) => {
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
    const cmdOrCtrl = isMac ? e.metaKey : e.ctrlKey;

    if (cmdOrCtrl) {
      switch (e.key.toLowerCase()) {
        case 'n':
          e.preventDefault();
          publish('shortcut:add-timezone');
          break;
        case 'k':
          e.preventDefault();
          publish('shortcut:search');
          break;
        case ',':
          e.preventDefault();
          publish('shortcut:settings');
          break;
        case 'm':
          e.preventDefault();
          publish('shortcut:compact-mode');
          break;
        case 'q':
          e.preventDefault();
          publish('shortcut:quit');
          break;
      }
    } else if (e.key === 'Escape') {
      publish('shortcut:escape');
    }
  });
}
