import { Alarm, AlarmState, Recurrence } from '../types/alarm';
import timezonesData from '../data/timezones.json';
import { setAlarmEnabled, removeAlarm } from '../state/alarms';
import { AlarmForm } from './alarm-form';
import { subscribe } from '../state/event-bus';
import { getPreferences } from '../state/preferences';

export class AlarmCard {
  public element: HTMLElement;
  private timeElement!: HTMLElement;
  private unsubscribePrefs?: () => void;
  private alarm: Alarm;

  constructor(container: HTMLElement, alarm: Alarm) {
    this.alarm = alarm;
    this.element = document.createElement('div');
    this.element.className = `alarm-card alarm-state-${alarm.state}`;

    const header = document.createElement('div');
    header.className = 'alarm-card__header';

    const title = document.createElement('div');
    title.className = 'alarm-card__title';
    title.textContent = alarm.title;

    const toggle = document.createElement('input');
    toggle.type = 'checkbox';
    toggle.className = 'alarm-card__toggle';
    toggle.checked = alarm.enabled;
    toggle.addEventListener('change', async (e) => {
      const target = e.target as HTMLInputElement;
      await setAlarmEnabled(this.alarm.id, target.checked);
    });

    header.appendChild(title);
    header.appendChild(toggle);

    const body = document.createElement('div');
    body.className = 'alarm-card__body';

    const timeGroup = document.createElement('div');
    timeGroup.className = 'alarm-card__time-group';

    this.timeElement = document.createElement('div');
    this.timeElement.className = 'alarm-card__time';
    this.timeElement.textContent = alarm.target_time; // TODO: format based on user preference

    const tzData = timezonesData.find((tz: any) => tz.id === alarm.timezone_id);
    const timezone = document.createElement('div');
    timezone.className = 'alarm-card__timezone';
    timezone.textContent = tzData ? `in ${tzData.city}` : `in ${alarm.timezone_id}`;

    timeGroup.appendChild(this.timeElement);
    timeGroup.appendChild(timezone);

    const recurrence = document.createElement('div');
    recurrence.className = 'alarm-card__recurrence';
    recurrence.textContent = this.formatRecurrence(alarm);

    body.appendChild(timeGroup);
    body.appendChild(recurrence);

    const footer = document.createElement('div');
    footer.className = 'alarm-card__footer';

    const stateBadge = document.createElement('div');
    stateBadge.className = 'alarm-card__badge';
    stateBadge.textContent = this.formatState(alarm.state);

    const actions = document.createElement('div');
    actions.className = 'alarm-card__actions';

    const editBtn = document.createElement('button');
    editBtn.textContent = 'Edit';
    editBtn.addEventListener('click', () => {
      const form = new AlarmForm(document.body, this.alarm);
      form.show();
    });

    const deleteBtn = document.createElement('button');
    deleteBtn.textContent = 'Delete';
    deleteBtn.addEventListener('click', async () => {
      if (confirm('Are you sure you want to delete this alarm?')) {
        await removeAlarm(this.alarm.id);
      }
    });

    actions.appendChild(editBtn);
    actions.appendChild(deleteBtn);

    footer.appendChild(stateBadge);
    footer.appendChild(actions);

    this.element.appendChild(header);
    this.element.appendChild(body);
    this.element.appendChild(footer);

    container.appendChild(this.element);

    this.unsubscribePrefs = subscribe('PREFERENCES_CHANGED', () => {
      this.updateTimeDisplay();
    });
  }

  private updateTimeDisplay() {
    const prefs = getPreferences();
    if (prefs.time_format === '12h') {
      const [hh, mm] = this.alarm.target_time.split(':').map(Number);
      const period = hh >= 12 ? 'PM' : 'AM';
      const hours12 = hh % 12 || 12;
      this.timeElement.textContent = `${hours12}:${mm.toString().padStart(2, '0')} ${period}`;
    } else {
      this.timeElement.textContent = this.alarm.target_time;
    }
  }

  private formatRecurrence(alarm: Alarm): string {
    if (alarm.recurrence === Recurrence.once) return 'Once';
    if (alarm.recurrence === Recurrence.daily) return 'Daily';
    if (alarm.recurrence === Recurrence.weekdays) return 'Weekdays';
    if (alarm.recurrence === Recurrence.weekends) return 'Weekends';
    
    if (alarm.recurrence === Recurrence.custom) {
      const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
      return alarm.custom_days.map(d => days[d]).join(', ');
    }
    return '';
  }

  private formatState(state: AlarmState): string {
    switch (state) {
      case AlarmState.active: return '🟢 Active';
      case AlarmState.snoozed: return '🟡 Snoozed';
      case AlarmState.completed: return '⚪ Completed';
      case AlarmState.dismissed: return '❌ Dismissed';
    }
    return '';
  }

  destroy() {
    if (this.unsubscribePrefs) this.unsubscribePrefs();
    this.element.remove();
  }
}
