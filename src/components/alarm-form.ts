import { Alarm, AlarmState, Recurrence } from '../types/alarm';
import timezonesData from '../data/timezones.json';
import { addAlarm, updateAlarm } from '../state/alarms';

function uuidv4() {
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
    const r = Math.random() * 16 | 0, v = c === 'x' ? r : (r & 0x3 | 0x8);
    return v.toString(16);
  });
}

export class AlarmForm {
  public element: HTMLDialogElement;
  private form: HTMLFormElement;
  private existingAlarm?: Alarm;
  private onSaved?: () => void;
  
  constructor(container: HTMLElement, existingAlarm?: Alarm, onSaved?: () => void) {
    this.existingAlarm = existingAlarm;
    this.onSaved = onSaved;
    
    this.element = document.createElement('dialog');
    this.element.className = 'alarm-form-modal';
    
    this.form = document.createElement('form');
    this.form.className = 'alarm-form';
    this.form.addEventListener('submit', this.handleSubmit.bind(this));

    const header = document.createElement('h2');
    header.textContent = existingAlarm ? 'Edit Alarm' : 'New Alarm';
    this.form.appendChild(header);

    // Title
    const titleGroup = document.createElement('div');
    titleGroup.className = 'form-group';
    const titleLabel = document.createElement('label');
    titleLabel.textContent = 'Title';
    const titleInput = document.createElement('input');
    titleInput.type = 'text';
    titleInput.name = 'title';
    titleInput.maxLength = 200;
    titleInput.required = true;
    titleInput.value = existingAlarm?.title || 'Alarm';
    titleGroup.appendChild(titleLabel);
    titleGroup.appendChild(titleInput);
    this.form.appendChild(titleGroup);

    // Time
    const timeGroup = document.createElement('div');
    timeGroup.className = 'form-group';
    const timeLabel = document.createElement('label');
    timeLabel.textContent = 'Time';
    const timeInput = document.createElement('input');
    timeInput.type = 'time';
    timeInput.name = 'target_time';
    timeInput.required = true;
    timeInput.value = existingAlarm?.target_time || '08:00';
    timeGroup.appendChild(timeLabel);
    timeGroup.appendChild(timeInput);
    this.form.appendChild(timeGroup);

    // Timezone
    const tzGroup = document.createElement('div');
    tzGroup.className = 'form-group';
    const tzLabel = document.createElement('label');
    tzLabel.textContent = 'Timezone';
    const tzSelect = document.createElement('select');
    tzSelect.name = 'timezone_id';
    tzSelect.required = true;
    
    const localTz = Intl.DateTimeFormat().resolvedOptions().timeZone;
    timezonesData.forEach((tz: any) => {
      const option = document.createElement('option');
      option.value = tz.id;
      option.textContent = `${tz.city} (${tz.id})`;
      if (existingAlarm) {
        if (tz.id === existingAlarm.timezone_id) option.selected = true;
      } else {
        if (tz.id === localTz) option.selected = true;
      }
      tzSelect.appendChild(option);
    });
    tzGroup.appendChild(tzLabel);
    tzGroup.appendChild(tzSelect);
    this.form.appendChild(tzGroup);

    // Recurrence
    const recGroup = document.createElement('div');
    recGroup.className = 'form-group';
    const recLabel = document.createElement('label');
    recLabel.textContent = 'Recurrence';
    const recSelect = document.createElement('select');
    recSelect.name = 'recurrence';
    
    Object.values(Recurrence).forEach(r => {
      const option = document.createElement('option');
      option.value = r;
      option.textContent = r.charAt(0).toUpperCase() + r.slice(1);
      if (existingAlarm?.recurrence === r) option.selected = true;
      recSelect.appendChild(option);
    });
    
    recGroup.appendChild(recLabel);
    recGroup.appendChild(recSelect);
    this.form.appendChild(recGroup);

    // Custom Days
    const customDaysGroup = document.createElement('div');
    customDaysGroup.className = 'form-group custom-days-group';
    customDaysGroup.style.display = (existingAlarm?.recurrence === Recurrence.custom) ? 'block' : 'none';
    
    const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    days.forEach((day, index) => {
      const label = document.createElement('label');
      label.className = 'custom-day-label';
      const checkbox = document.createElement('input');
      checkbox.type = 'checkbox';
      checkbox.name = `custom_day_${index}`;
      checkbox.value = index.toString();
      if (existingAlarm?.custom_days?.includes(index)) {
        checkbox.checked = true;
      }
      label.appendChild(checkbox);
      label.appendChild(document.createTextNode(day));
      customDaysGroup.appendChild(label);
    });
    this.form.appendChild(customDaysGroup);

    recSelect.addEventListener('change', () => {
      customDaysGroup.style.display = recSelect.value === Recurrence.custom ? 'block' : 'none';
    });

    // Sound
    const soundGroup = document.createElement('div');
    soundGroup.className = 'form-group checkbox-group';
    const soundLabel = document.createElement('label');
    const soundCheckbox = document.createElement('input');
    soundCheckbox.type = 'checkbox';
    soundCheckbox.name = 'sound_enabled';
    soundCheckbox.checked = existingAlarm ? existingAlarm.sound_enabled : true;
    soundLabel.appendChild(soundCheckbox);
    soundLabel.appendChild(document.createTextNode('Play Sound'));
    soundGroup.appendChild(soundLabel);
    this.form.appendChild(soundGroup);

    // Actions
    const actions = document.createElement('div');
    actions.className = 'form-actions';
    
    const cancelBtn = document.createElement('button');
    cancelBtn.type = 'button';
    cancelBtn.textContent = 'Cancel';
    cancelBtn.addEventListener('click', () => this.close());
    
    const saveBtn = document.createElement('button');
    saveBtn.type = 'submit';
    saveBtn.textContent = 'Save';
    
    actions.appendChild(cancelBtn);
    actions.appendChild(saveBtn);
    this.form.appendChild(actions);

    this.element.appendChild(this.form);
    container.appendChild(this.element);

    // Focus trap setup
    this.element.addEventListener('keydown', this.handleKeydown.bind(this));
  }

  public show() {
    this.element.showModal();
    const firstInput = this.form.querySelector('input') as HTMLInputElement;
    if (firstInput) firstInput.focus();
  }

  public close() {
    this.element.close();
    this.element.remove();
  }

  private handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      this.close();
    }
  }

  private async handleSubmit(e: Event) {
    e.preventDefault();
    
    const formData = new FormData(this.form);
    const recurrence = formData.get('recurrence') as Recurrence;
    const custom_days: number[] = [];
    if (recurrence === Recurrence.custom) {
      for (let i = 0; i < 7; i++) {
        if (formData.get(`custom_day_${i}`)) {
          custom_days.push(i);
        }
      }
    }

    const alarm: Alarm = {
      id: this.existingAlarm ? this.existingAlarm.id : uuidv4(),
      title: formData.get('title') as string,
      target_time: formData.get('target_time') as string,
      timezone_id: formData.get('timezone_id') as string,
      recurrence,
      custom_days,
      enabled: true,
      state: AlarmState.active,
      snooze_count: 0,
      sound_enabled: formData.get('sound_enabled') === 'on',
      created_at: this.existingAlarm ? this.existingAlarm.created_at : Date.now()
    };

    try {
      if (this.existingAlarm) {
        await updateAlarm(alarm);
      } else {
        await addAlarm(alarm);
      }
      if (this.onSaved) this.onSaved();
      this.close();
    } catch (err) {
      alert(err instanceof Error ? err.message : 'Error saving alarm');
    }
  }
}
