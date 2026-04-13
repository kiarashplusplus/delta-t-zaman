import { getClocks } from '../state/clocks';
import { updateTrayDisplay } from '../services/ipc';

export function setupTrayUpdates() {
  const updateTray = async () => {
    const clocks = getClocks();
    const zones = clocks.map(clock => {
      const date = new Date();
      const timeString = new Intl.DateTimeFormat('en-US', {
        timeZone: clock.iana_id,
        hour: 'numeric',
        minute: 'numeric',
        hour12: true
      }).format(date);
      
      return {
        label: clock.display_label,
        time: timeString,
        is_pinned: clock.pinned_to_tray || false
      };
    });
    
    await updateTrayDisplay(zones);
  };

  updateTray();
  setInterval(updateTray, 60000);
}
