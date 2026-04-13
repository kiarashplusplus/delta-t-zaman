import * as SunCalc from 'suncalc';

interface DayNightStatus {
  polar?: boolean;
  isDaylight: boolean;
  sunrise?: Date;
  sunset?: Date;
}

const cache = new Map<string, { time: number; status: DayNightStatus | null }>();
const CACHE_TTL = 60 * 1000;

export function getDayNightStatus(lat: number, lng: number, date: Date): DayNightStatus | null {
  if (isNaN(lat) || isNaN(lng) || lat === 0 && lng === 0) return null;

  const cacheKey = `${lat.toFixed(2)},${lng.toFixed(2)}`;
  const cached = cache.get(cacheKey);
  if (cached && (Date.now() - cached.time < CACHE_TTL)) {
    return cached.status;
  }

  let status: DayNightStatus | null = null;

  if (Math.abs(lat) > 66.5) {
    const times = SunCalc.getTimes(date, lat, lng);
    if (isNaN(times.sunrise.getTime()) || isNaN(times.sunset.getTime())) {
      const position = SunCalc.getPosition(date, lat, lng);
      status = { polar: true, isDaylight: position.altitude > 0 };
    }
  }

  if (!status) {
    const times = SunCalc.getTimes(date, lat, lng);
    if (isNaN(times.sunrise.getTime()) || isNaN(times.sunset.getTime())) {
       // fallback for invalid dates when not strictly > 66.5 but suncalc returns invalid
       const position = SunCalc.getPosition(date, lat, lng);
       status = { polar: true, isDaylight: position.altitude > 0 };
    } else {
      const now = date.getTime();
      status = {
        isDaylight: now > times.sunrise.getTime() && now < times.sunset.getTime(),
        sunrise: times.sunrise,
        sunset: times.sunset
      };
    }
  }

  cache.set(cacheKey, { time: Date.now(), status });
  return status;
}
