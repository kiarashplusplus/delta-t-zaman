import timezonesData from '../data/timezones.json';
import type { TimezoneMetadata } from '../types/timezone';

const timezones = timezonesData as TimezoneMetadata[];

function levenshtein(a: string, b: string): number {
  const an = a ? a.length : 0;
  const bn = b ? b.length : 0;
  if (an === 0) return bn;
  if (bn === 0) return an;
  const matrix = new Array<number[]>(bn + 1);
  for (let i = 0; i <= bn; ++i) {
    let row = matrix[i] = new Array<number>(an + 1);
    row[0] = i;
  }
  const firstRow = matrix[0];
  for (let j = 1; j <= an; ++j) {
    firstRow[j] = j;
  }
  for (let i = 1; i <= bn; ++i) {
    for (let j = 1; j <= an; ++j) {
      if (b.charAt(i - 1) === a.charAt(j - 1)) {
        matrix[i][j] = matrix[i - 1][j - 1];
      } else {
        matrix[i][j] = Math.min(
          matrix[i - 1][j - 1] + 1, // substitution
          Math.min(
            matrix[i][j - 1] + 1, // insertion
            matrix[i - 1][j] + 1 // deletion
          )
        );
      }
    }
  }
  return matrix[bn][an];
}

export function searchTimezones(query: string): TimezoneMetadata[] {
  if (!query) return [];
  const q = query.toLowerCase().trim();

  // UTC offset parsing (e.g. UTC+3, +3, -05:00)
  let targetOffsetMinutes: number | null = null;
  const offsetMatch = q.match(/^(?:utc|gmt)?\s*([+-]\d{1,2})(?::?(\d{2}))?$/i);
  if (offsetMatch) {
    const hours = parseInt(offsetMatch[1], 10);
    const minutes = offsetMatch[2] ? parseInt(offsetMatch[2], 10) : 0;
    targetOffsetMinutes = (hours * 60) + (hours < 0 ? -minutes : minutes);
  }

  const results = timezones.map(tz => {
    let score = 0;
    const lowerCity = tz.city ? tz.city.toLowerCase() : '';
    const lowerId = tz.id ? tz.id.toLowerCase() : '';
    const lowerCountry = tz.country ? tz.country.toLowerCase() : '';
    
    // 1. Exact substring match
    if (lowerCity.includes(q)) {
      score += 100 + (q.length / Math.max(1, lowerCity.length)) * 10;
      if (lowerCity.startsWith(q)) score += 50;
    } else if (lowerId.includes(q)) {
      score += 80;
    } else if (lowerCountry.includes(q)) {
      score += 70;
    } else if (tz.aliases && tz.aliases.some(a => a.toLowerCase().includes(q))) {
      score += 60;
    }

    // 2. Fuzzy Levenshtein
    if (score === 0 && q.length > 2) {
      const qWords = q.split(/\s+/);
      const cWords = lowerCity.split(/\s+/);
      let minDistance = 999;
      for (const qw of qWords) {
        if (qw.length <= 2) continue;
        for (const cw of cWords) {
          if (Math.abs(qw.length - cw.length) <= 1) {
            const dist = levenshtein(qw, cw);
            if (dist < minDistance) minDistance = dist;
          }
        }
      }
      if (minDistance <= 1) {
         score += 40 - minDistance * 10;
      }
    }

    // 3. UTC offset match
    if (targetOffsetMinutes !== null && tz.utc_offset_minutes === targetOffsetMinutes) {
      score += 200;
    }

    return { tz, score };
  });

  return results
    .filter(r => r.score > 0)
    .sort((a, b) => {
      if (b.score !== a.score) return b.score - a.score;
      const aCity = a.tz.city || a.tz.id || '';
      const bCity = b.tz.city || b.tz.id || '';
      return aCity.localeCompare(bCity);
    })
    .map(r => r.tz)
    .slice(0, 20);
}
