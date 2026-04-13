import { getDayNightStatus } from '../services/daynight';

export class DayNightIndicator {
  private element: HTMLElement;
  private bar: HTMLElement;
  private marker: HTMLElement;
  private textElement: HTMLElement;

  constructor(container: HTMLElement) {
    this.element = document.createElement('div');
    this.element.className = 'daynight-indicator';
    this.element.setAttribute('role', 'img');
    
    this.bar = document.createElement('div');
    this.bar.className = 'daynight-indicator__bar';
    
    this.marker = document.createElement('div');
    this.marker.className = 'daynight-indicator__marker';
    
    this.textElement = document.createElement('div');
    this.textElement.className = 'daynight-indicator__text';

    this.element.appendChild(this.bar);
    this.element.appendChild(this.marker);
    this.element.appendChild(this.textElement);
    container.appendChild(this.element);
  }

  update(lat: number, lng: number, timestamp: number) {
    const date = new Date(timestamp);
    const status = getDayNightStatus(lat, lng, date);

    if (!status) {
      this.element.style.display = 'none';
      return;
    }

    this.element.style.display = '';

    if (status.polar) {
      this.element.classList.add('daynight-indicator--polar');
      this.bar.style.display = 'none';
      this.marker.style.display = 'none';
      this.textElement.style.display = '';
      
      const label = status.isDaylight ? '24h daylight' : '24h darkness';
      this.textElement.textContent = label;
      this.element.setAttribute('aria-label', label);
      
      this.element.classList.toggle('daynight-indicator--day', status.isDaylight);
      this.element.classList.toggle('daynight-indicator--night', !status.isDaylight);
    } else {
      this.element.classList.remove('daynight-indicator--polar');
      this.bar.style.display = '';
      this.marker.style.display = '';
      this.textElement.style.display = 'none';

      this.element.classList.toggle('daynight-indicator--day', status.isDaylight);
      this.element.classList.toggle('daynight-indicator--night', !status.isDaylight);

      if (status.sunrise && status.sunset) {
        const startOfDay = new Date(date).setHours(0, 0, 0, 0);
        const msInDay = 24 * 60 * 60 * 1000;
        const percent = ((timestamp - startOfDay) / msInDay) * 100;
        
        this.marker.style.left = `${percent}%`;

        const sunrisePercent = ((status.sunrise.getTime() - startOfDay) / msInDay) * 100;
        const sunsetPercent = ((status.sunset.getTime() - startOfDay) / msInDay) * 100;

        this.bar.style.background = `linear-gradient(to right, 
          #1a1a2e 0%, 
          #1a1a2e ${sunrisePercent}%, 
          #fdb813 ${sunrisePercent}%, 
          #fdb813 ${sunsetPercent}%, 
          #1a1a2e ${sunsetPercent}%, 
          #1a1a2e 100%)`;

        const label = status.isDaylight ? 'Daytime' : 'Nighttime';
        this.element.setAttribute('aria-label', `${label}, sunrise at ${status.sunrise.toLocaleTimeString()}, sunset at ${status.sunset.toLocaleTimeString()}`);
      }
    }
  }
}
