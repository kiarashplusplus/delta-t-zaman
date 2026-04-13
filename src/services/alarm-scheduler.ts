import { Alarm, Recurrence } from '../types/alarm';
import { scheduleAlarmNotification, cancelAlarmNotification } from './ipc';
import { publish } from '../state/event-bus';

function getLocalParts(timestamp: number, timeZone: string) {
  const parts = new Intl.DateTimeFormat('en-US', {
    timeZone,
    year: 'numeric', month: 'numeric', day: 'numeric',
    hour: 'numeric', minute: 'numeric', second: 'numeric',
    weekday: 'short',
    hour12: false
  }).formatToParts(new Date(timestamp));
  
  const map: Record<string, string> = {};
  for (const part of parts) map[part.type] = part.value;
  
  return {
    year: parseInt(map.year),
    month: parseInt(map.month),
    day: parseInt(map.day),
    hour: map.hour === '24' ? 0 : parseInt(map.hour),
    minute: parseInt(map.minute),
    second: parseInt(map.second),
    weekday: map.weekday
  };
}

function getDayOfWeekInt(weekdayShort: string): number {
  const days = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  return days.indexOf(weekdayShort);
}

function matchesRecurrence(alarm: Alarm, dayOfWeek: number): boolean {
  switch (alarm.recurrence) {
    case Recurrence.once:
    case Recurrence.daily:
      return true;
    case Recurrence.weekdays:
      return dayOfWeek >= 1 && dayOfWeek <= 5;
    case Recurrence.weekends:
      return dayOfWeek === 0 || dayOfWeek === 6;
    case Recurrence.custom:
      return alarm.custom_days.includes(dayOfWeek);
    default:
      return false;
  }
}

function getOffsetMins(timestamp: number, timeZone: string): number {
  try {
    const parts = new Intl.DateTimeFormat('en-US', { timeZone, timeZoneName: 'longOffset' }).formatToParts(new Date(timestamp));
    const tzName = parts.find(p => p.type === 'timeZoneName')?.value;
    if (!tzName || tzName === 'GMT') return 0;
    const match = tzName.match(/GMT([+-])(\d{1,2}):(\d{2})/);
    if (match) {
      const sign = match[1] === '+' ? 1 : -1;
      return sign * (parseInt(match[2]) * 60 + parseInt(match[3]));
    }
  } catch (e) {
    // ignore
  }
  return 0;
}

export function computeNextFireTime(alarm: Alarm, fromTime: number = Date.now()): number | null {
  const [h, m] = alarm.target_time.split(':').map(Number);
  const timeZone = alarm.timezone_id;

  // Check up to 8 days in the future
  for (let i = 0; i <= 8; i++) {
    const testTime = fromTime + i * 24 * 3600 * 1000;
    let local;
    try {
      local = getLocalParts(testTime, timeZone);
    } catch (e) {
      // fallback to system timezone or just skip if invalid timezone_id
      continue;
    }
    
    const dayOfWeek = getDayOfWeekInt(local.weekday);
    if (!matchesRecurrence(alarm, dayOfWeek)) continue;
    
    const estimatedUtc = Date.UTC(local.year, local.month - 1, local.day, h, m);
    const offset1 = getOffsetMins(estimatedUtc, timeZone);
    let targetUtc = estimatedUtc - offset1 * 60 * 1000;
    
    let verifyLocal = getLocalParts(targetUtc, timeZone);
    if (verifyLocal.hour !== h || verifyLocal.minute !== m) {
      const offset2 = getOffsetMins(targetUtc, timeZone);
      if (offset1 !== offset2) {
         targetUtc = estimatedUtc - offset2 * 60 * 1000;
         verifyLocal = getLocalParts(targetUtc, timeZone);
      }
      if (verifyLocal.hour !== h || verifyLocal.minute !== m) {
         // skip invalid time
         continue;
      }
    }
    
    if (targetUtc > fromTime) {
      return targetUtc;
    }
  }
  return null;
}

export async function scheduleAlarm(alarm: Alarm): Promise<void> {
  if (alarm.notification_id) {
    await cancelAlarmNotification(alarm.notification_id.toString());
  }

  const fireTime = computeNextFireTime(alarm);
  if (!fireTime) return;

  const isoDate = new Date(fireTime).toISOString();
  
  const result = await scheduleAlarmNotification({
    alarm_id: alarm.id,
    title: alarm.title,
    body: `Alarm in ${alarm.timezone_id}`,
    schedule_at: isoDate,
    sound_enabled: alarm.sound_enabled,
    action_type: alarm.recurrence === Recurrence.once ? 'one_time' : 'recurring'
  });

  if (result && result.notification_id) {
    // In actual implementation we might want to save this to state, but for now we dispatch an event
    publish('alarm-scheduled', { id: alarm.id, notification_id: result.notification_id, next_fire_time: fireTime });
  }
}

export async function snoozeAlarm(alarm: Alarm): Promise<void> {
  if (alarm.notification_id) {
    await cancelAlarmNotification(alarm.notification_id.toString());
  }

  const snoozeFireTime = Date.now() + 5 * 60 * 1000; // 5 mins
  const isoDate = new Date(snoozeFireTime).toISOString();
  
  const result = await scheduleAlarmNotification({
    alarm_id: alarm.id,
    title: `Snoozed: ${alarm.title}`,
    body: `Alarm in ${alarm.timezone_id}`,
    schedule_at: isoDate,
    sound_enabled: alarm.sound_enabled,
    action_type: 'snoozed'
  });

  if (result && result.notification_id) {
    publish('alarm-scheduled', { id: alarm.id, notification_id: result.notification_id, next_fire_time: snoozeFireTime });
  }
}

export async function cancelAlarmSchedule(alarm: Alarm): Promise<void> {
  if (alarm.notification_id) {
    await cancelAlarmNotification(alarm.notification_id.toString());
  }
}
