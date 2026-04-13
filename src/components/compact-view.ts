import { getClocks } from '../state/clocks';

export class CompactView {
  private container: HTMLElement;
  private intervalId?: number;

  constructor(parent: HTMLElement) {
    this.container = document.createElement('div');
    this.container.className = 'compact-view';
    parent.appendChild(this.container);
    
    this.render();
    this.intervalId = window.setInterval(() => this.render(), 60000);
  }

  public destroy() {
    if (this.intervalId) clearInterval(this.intervalId);
    this.container.remove();
  }

  private render() {
    const clocks = getClocks();
    this.container.innerHTML = '';
    
    clocks.forEach(clock => {
      const date = new Date();
      const timeString = new Intl.DateTimeFormat('en-US', {
        timeZone: clock.iana_id,
        hour: 'numeric',
        minute: 'numeric',
        hour12: true
      }).format(date);

      const strip = document.createElement('div');
      strip.className = 'compact-strip';
      strip.style.display = 'flex';
      strip.style.justifyContent = 'space-between';
      strip.style.padding = '8px';
      strip.style.borderBottom = '1px solid var(--surface-2)';
      
      strip.innerHTML = `
        <span class="compact-label" style="font-weight: 500;">${clock.display_label}</span>
        <span class="compact-time">${timeString}</span>
      `;
      this.container.appendChild(strip);
    });
  }
}
