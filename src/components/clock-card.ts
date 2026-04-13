import { subscribe } from '../state/event-bus';
import { getPreferences } from '../state/preferences';
import { removeClock, updateLabel } from '../state/clocks';
import { DayNightIndicator } from './daynight-indicator';
import type { TimeZoneEntry, TimezoneMetadata } from '../types/timezone';

export class ClockCard {
  public element: HTMLElement;
  private timeElement: HTMLElement;
  private dateElement: HTMLElement;
  private offsetElement: HTMLElement;
  private relativeElement: HTMLElement;
  private badgeElement: HTMLElement;
  private dayNightIndicator: DayNightIndicator;
  
  private unsubscribeTicker: () => void;
  private unsubscribePrefs?: () => void;
  private zoneEntry: TimeZoneEntry;
  private metadata: TimezoneMetadata;

  constructor(container: HTMLElement, zoneEntry: TimeZoneEntry, metadata: TimezoneMetadata) {
    this.zoneEntry = zoneEntry;
    this.metadata = metadata;

    this.element = document.createElement('div');
    this.element.className = 'clock-card';

    const header = document.createElement('div');
    header.className = 'clock-card__header';
    
    const title = document.createElement('div');
    title.className = 'clock-card__title';
    title.textContent = zoneEntry.display_label;
    title.contentEditable = 'true';
    title.spellcheck = false;
    title.addEventListener('blur', () => {
      const newLabel = title.textContent?.trim() || zoneEntry.iana_id;
      if (newLabel !== zoneEntry.display_label) {
        updateLabel(zoneEntry.id, newLabel);
      }
    });
    title.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault();
        title.blur();
      }
    });

    this.badgeElement = document.createElement('div');
    this.badgeElement.className = 'clock-card__badge';
    this.badgeElement.style.display = 'none';

    const deleteBtn = document.createElement('button');
    deleteBtn.className = 'clock-card__delete';
    deleteBtn.innerHTML = '&times;';
    deleteBtn.setAttribute('aria-label', 'Delete clock');
    deleteBtn.addEventListener('click', () => removeClock(zoneEntry.id));

    header.appendChild(title);
    header.appendChild(this.badgeElement);
    header.appendChild(deleteBtn);

    this.timeElement = document.createElement('div');
    this.timeElement.className = 'clock-card__time';

    this.dateElement = document.createElement('div');
    this.dateElement.className = 'clock-card__date';

    const footer = document.createElement('div');
    footer.className = 'clock-card__footer';

    this.offsetElement = document.createElement('div');
    this.offsetElement.className = 'clock-card__offset';

    this.relativeElement = document.createElement('div');
    this.relativeElement.className = 'clock-card__relative';

    footer.appendChild(this.offsetElement);
    footer.appendChild(this.relativeElement);

    this.element.appendChild(header);
    this.element.appendChild(this.timeElement);
    this.element.appendChild(this.dateElement);
    this.element.appendChild(footer);

    this.dayNightIndicator = new DayNightIndicator(this.element);

    container.appendChild(this.element);

    this.unsubscribeTicker = subscribe<number>('TIME_TICK', (timestamp) => {
      this.update(timestamp);
    });
  }

  update(timestamp: number) {
    const prefs = getPreferences();
    const is12h = prefs.time_format === '12h';

    const timeFmt = new Intl.DateTimeFormat(prefs.locale, {
      timeZone: this.zoneEntry.iana_id,
      hour: 'numeric',
      minute: '2-digit',
      second: '2-digit',
      hour12: is12h
    });

    const dateFmt = new Intl.DateTimeFormat(prefs.locale, {
      timeZone: this.zoneEntry.iana_id,
      weekday: 'short',
      month: 'short',
      day: 'numeric'
    });

    this.timeElement.textContent = timeFmt.format(new Date(timestamp));
    this.dateElement.textContent = dateFmt.format(new Date(timestamp));

    const localDateStr = new Intl.DateTimeFormat('en-US', { timeZone: Intl.DateTimeFormat().resolvedOptions().timeZone, year: 'numeric', month: 'numeric', day: 'numeric' }).format(new Date(timestamp));
    const targetDateStr = new Intl.DateTimeFormat('en-US', { timeZone: this.zoneEntry.iana_id, year: 'numeric', month: 'numeric', day: 'numeric' }).format(new Date(timestamp));
    
    const localDate = new Date(localDateStr);
    const targetDate = new Date(targetDateStr);
    
    const diffDays = Math.round((targetDate.getTime() - localDate.getTime()) / (1000 * 60 * 60 * 24));
    
    if (diffDays === 1) {
      this.badgeElement.textContent = 'Tomorrow';
      this.badgeElement.style.display = '';
    } else if (diffDays === -1) {
      this.badgeElement.textContent = 'Yesterday';
      this.badgeElement.style.display = '';
    } else if (diffDays > 1) {
      this.badgeElement.textContent = `+${diffDays} days`;
      this.badgeElement.style.display = '';
    } else if (diffDays < -1) {
       this.badgeElement.textContent = `${diffDays} days`;
       this.badgeElement.style.display = '';
    } else {
      this.badgeElement.style.display = 'none';
    }

    const timeZoneNameFmt = new Intl.DateTimeFormat('en-US', {
      timeZone: this.zoneEntry.iana_id,
      timeZoneName: 'shortOffset'
    });
    const parts = timeZoneNameFmt.formatToParts(new Date(timestamp));
    const tzName = parts.find(p => p.type === 'timeZoneName')?.value || '';
    
    const abbrFmt = new Intl.DateTimeFormat('en-US', {
      timeZone: this.zoneEntry.iana_id,
      timeZoneName: 'short'
    });
    const abbrParts = abbrFmt.formatToParts(new Date(timestamp));
    const abbr = abbrParts.find(p => p.type === 'timeZoneName')?.value || '';
    
    this.offsetElement.textContent = `${tzName} ${abbr !== tzName ? abbr : ''}`.trim();

    const localOffsetMinutes = -new Date().getTimezoneOffset();
    const targetOffsetMinutes = this.getOffsetMinutes(this.zoneEntry.iana_id, new Date(timestamp));
    const diffMinutes = targetOffsetMinutes - localOffsetMinutes;

    if (diffMinutes === 0) {
      this.relativeElement.textContent = 'same time';
    } else {
      const hours = diffMinutes / 60;
      this.relativeElement.textContent = hours > 0 ? `+${hours}h` : `${hours}h`;
    }

    this.dayNightIndicator.update(this.metadata.latitude, this.metadata.longitude, timestamp);
  }

  private getOffsetMinutes(timeZone: string, date: Date): number {
    const tzStr = new Intl.DateTimeFormat('en-US', {
      timeZone,
      timeZoneName: 'longOffset'
    }).format(date);
    const match = tzStr.match(/GMT([+-])(\d{2}):(\d{2})/);
    if (match) {
      const sign = match[1] === '+' ? 1 : -1;
      const hours = parseInt(match[2], 10);
      const minutes = parseInt(match[3], 10);
      return sign * (hours * 60 + minutes);
    }
    return 0;
  }

  destroy() {
    this.unsubscribeTicker();
    if (this.unsubscribePrefs) this.unsubscribePrefs();
    this.element.remove();
  }
}
