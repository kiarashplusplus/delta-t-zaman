import { Alarm, AlarmState, Recurrence } from '../types/alarm';
import { getStoreValue, setStoreValue } from '../services/persistence';
import { publish } from './event-bus';
import { scheduleAlarm, snoozeAlarm as schedulerSnooze, cancelAlarmSchedule, computeNextFireTime } from '../services/alarm-scheduler';

const ALARMS_STORE = 'alarms.dat';
const ALARMS_KEY = 'alarms';

let stateAlarms: Alarm[] = [];

export async function initAlarms(): Promise<void> {
  const data = await getStoreValue<Alarm[]>(ALARMS_STORE, ALARMS_KEY, []);
  
  const now = Date.now();
  const thirtyDaysMs = 30 * 24 * 60 * 60 * 1000;
  
  // Retain completed/dismissed for 30 days
  stateAlarms = data.filter(a => {
    if ((a.state === AlarmState.completed || a.state === AlarmState.dismissed) && a.completed_at) {
      return (now - a.completed_at) < thirtyDaysMs;
    }
    return true;
  });

  let changed = false;

  for (const alarm of stateAlarms) {
    if (!alarm.enabled) continue;

    if (alarm.state === AlarmState.snoozed) {
      if (alarm.snooze_fire_time && alarm.snooze_fire_time <= now) {
        // Fire immediately by scheduling slightly in the future or we can just emit
        publish('alarm-fired', { id: alarm.id });
      } else {
        // Reschedule snooze if still in future
        await schedulerSnooze(alarm);
      }
    } else if (alarm.state === AlarmState.active) {
      if (alarm.recurrence === Recurrence.once) {
        const fireTime = computeNextFireTime(alarm, alarm.created_at);
        if (fireTime && fireTime <= now) {
          // Missed one-time alarm
          alarm.state = AlarmState.completed;
          alarm.completed_at = now;
          alarm.enabled = false;
          changed = true;
          continue;
        }
      }
      
      // Reschedule recurring or upcoming one-time
      await scheduleAlarm(alarm);
    }
  }

  if (changed || stateAlarms.length !== data.length) {
    await saveAlarms();
  }
}

export function getAlarms(): Alarm[] {
  return [...stateAlarms];
}

export function getAlarm(id: string): Alarm | undefined {
  return stateAlarms.find(a => a.id === id);
}

async function saveAlarms() {
  await setStoreValue(ALARMS_STORE, ALARMS_KEY, stateAlarms);
  publish('alarms-changed', stateAlarms);
}

export async function addAlarm(alarm: Alarm): Promise<void> {
  const activeCount = stateAlarms.filter(a => (a.state === AlarmState.active || a.state === AlarmState.snoozed) && a.enabled).length;
  if (activeCount >= 50 && alarm.enabled && (alarm.state === AlarmState.active || alarm.state === AlarmState.snoozed)) {
    throw new Error('Maximum of 50 active alarms reached');
  }
  
  stateAlarms.push(alarm);
  if (alarm.enabled && alarm.state === AlarmState.active) {
    await scheduleAlarm(alarm);
  }
  await saveAlarms();
}

export async function updateAlarm(updated: Alarm): Promise<void> {
  const idx = stateAlarms.findIndex(a => a.id === updated.id);
  if (idx !== -1) {
    const old = stateAlarms[idx];
    
    // Check limit if turning on
    if (!old.enabled && updated.enabled) {
      const activeCount = stateAlarms.filter(a => (a.state === AlarmState.active || a.state === AlarmState.snoozed) && a.enabled && a.id !== updated.id).length;
      if (activeCount >= 50) {
        throw new Error('Maximum of 50 active alarms reached');
      }
    }

    stateAlarms[idx] = updated;
    
    await cancelAlarmSchedule(old);
    if (updated.enabled && updated.state === AlarmState.active) {
      await scheduleAlarm(updated);
    } else if (updated.enabled && updated.state === AlarmState.snoozed) {
      await schedulerSnooze(updated);
    }
    await saveAlarms();
  }
}

export async function removeAlarm(id: string): Promise<void> {
  const idx = stateAlarms.findIndex(a => a.id === id);
  if (idx !== -1) {
    const alarm = stateAlarms[idx];
    await cancelAlarmSchedule(alarm);
    stateAlarms.splice(idx, 1);
    await saveAlarms();
  }
}

export async function setAlarmEnabled(id: string, enabled: boolean): Promise<void> {
  const alarm = stateAlarms.find(a => a.id === id);
  if (alarm && alarm.enabled !== enabled) {
    alarm.enabled = enabled;
    if (enabled) {
      alarm.state = AlarmState.active;
      alarm.snooze_count = 0;
    } else {
      // If disabling a snoozed alarm, we cancel it and keep it disabled.
      await cancelAlarmSchedule(alarm);
      if (alarm.state === AlarmState.snoozed || alarm.state === AlarmState.active) {
        if (alarm.recurrence === Recurrence.once) {
            alarm.state = AlarmState.dismissed;
            alarm.completed_at = Date.now();
        } else {
            alarm.state = AlarmState.active; // It stays active but disabled, so it won't schedule
        }
      }
    }
    await updateAlarm(alarm);
  }
}

export async function handleAlarmSnoozed(id: string): Promise<void> {
  const alarm = stateAlarms.find(a => a.id === id);
  if (alarm && alarm.enabled) {
    alarm.state = AlarmState.snoozed;
    alarm.snooze_count += 1;
    alarm.snooze_fire_time = Date.now() + 5 * 60 * 1000;
    await schedulerSnooze(alarm);
    await saveAlarms();
  }
}

export async function handleAlarmFired(id: string): Promise<void> {
  const alarm = stateAlarms.find(a => a.id === id);
  if (alarm) {
    if (alarm.recurrence === Recurrence.once) {
      alarm.state = AlarmState.completed;
      alarm.completed_at = Date.now();
      alarm.enabled = false;
    } else {
      alarm.state = AlarmState.active;
      alarm.snooze_count = 0;
      await scheduleAlarm(alarm);
    }
    await saveAlarms();
  }
}

export async function handleAlarmDismissed(id: string): Promise<void> {
  const alarm = stateAlarms.find(a => a.id === id);
  if (alarm) {
    if (alarm.recurrence === Recurrence.once) {
      alarm.state = AlarmState.dismissed;
      alarm.completed_at = Date.now();
      alarm.enabled = false;
    } else {
      alarm.state = AlarmState.active;
      alarm.snooze_count = 0;
      await scheduleAlarm(alarm);
    }
    await saveAlarms();
  }
}
