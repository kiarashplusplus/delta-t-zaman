export enum AlarmState {
  active = 'active',
  snoozed = 'snoozed',
  completed = 'completed',
  dismissed = 'dismissed'
}

export enum Recurrence {
  once = 'once',
  daily = 'daily',
  weekdays = 'weekdays',
  weekends = 'weekends',
  custom = 'custom'
}

export interface Alarm {
  id: string;
  title: string;
  target_time: string; // HH:mm
  timezone_id: string;
  recurrence: Recurrence;
  custom_days: number[]; // 0-6 where 0 is Sunday
  enabled: boolean;
  state: AlarmState;
  snooze_count: number;
  snooze_fire_time?: number;
  sound_enabled: boolean;
  notification_id?: number;
  created_at: number;
  completed_at?: number;
}
