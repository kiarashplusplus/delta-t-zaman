import { Alarm, AlarmState } from '../types/alarm';
import { getAlarms } from '../state/alarms';
import { subscribe } from '../state/event-bus';
import { AlarmCard } from './alarm-card';
import { AlarmForm } from './alarm-form';
import { getNotificationPermission, requestNotificationPermission } from '../services/ipc';

export class AlarmList {
  public element: HTMLElement;
  private listContainer: HTMLElement;
  private emptyState: HTMLElement;
  private unsubscribeAlarms: () => void;
  private cards: AlarmCard[] = [];

  constructor(container: HTMLElement) {
    this.element = document.createElement('div');
    this.element.className = 'alarm-list-view';

    const header = document.createElement('div');
    header.className = 'alarm-list-header';
    const title = document.createElement('h2');
    title.textContent = 'Alarms';
    const addBtn = document.createElement('button');
    addBtn.textContent = '+ Add Alarm';
    addBtn.setAttribute('aria-label', 'Add Alarm');
    addBtn.addEventListener('click', () => this.openAddAlarmForm());

    header.appendChild(title);
    header.appendChild(addBtn);

    this.listContainer = document.createElement('div');
    this.listContainer.className = 'alarm-list-container';
    this.listContainer.setAttribute('role', 'list');

    this.emptyState = document.createElement('div');
    this.emptyState.className = 'alarm-list-empty';
    this.emptyState.innerHTML = '<p>No alarms yet</p>';
    const emptyAddBtn = document.createElement('button');
    emptyAddBtn.textContent = 'Create an Alarm';
    emptyAddBtn.addEventListener('click', () => this.openAddAlarmForm());
    this.emptyState.appendChild(emptyAddBtn);

    this.element.appendChild(header);
    this.element.appendChild(this.listContainer);
    this.element.appendChild(this.emptyState);

    container.appendChild(this.element);

    this.render();

    this.unsubscribeAlarms = subscribe<Alarm[]>('alarms-changed', () => {
      this.render();
    });
  }

  private async openAddAlarmForm() {
    // Check permission on first alarm creation
    const alarms = getAlarms();
    if (alarms.length === 0) {
      try {
        const perm = await getNotificationPermission();
        if (perm !== 'granted') {
          await requestNotificationPermission();
        }
      } catch (err) {
        console.error('Failed to request notification permission', err);
      }
    }

    const form = new AlarmForm(document.body);
    form.show();
  }

  private render() {
    // Clear existing
    this.cards.forEach(card => card.destroy());
    this.cards = [];
    this.listContainer.innerHTML = '';

    const alarms = getAlarms();
    
    if (alarms.length === 0) {
      this.listContainer.style.display = 'none';
      this.emptyState.style.display = 'flex';
      return;
    }

    this.listContainer.style.display = 'block';
    this.emptyState.style.display = 'none';

    // Grouping: active first, then snoozed, then completed/dismissed
    const activeAlarms = alarms.filter(a => a.state === AlarmState.active);
    const snoozedAlarms = alarms.filter(a => a.state === AlarmState.snoozed);
    const completedAlarms = alarms.filter(a => a.state === AlarmState.completed || a.state === AlarmState.dismissed);

    const orderedAlarms = [...activeAlarms, ...snoozedAlarms, ...completedAlarms];

    orderedAlarms.forEach(alarm => {
      const card = new AlarmCard(this.listContainer, alarm);
      this.cards.push(card);
    });
  }

  destroy() {
    this.unsubscribeAlarms();
    this.cards.forEach(card => card.destroy());
    this.element.remove();
  }
}
