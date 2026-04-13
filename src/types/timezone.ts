export interface TimeZoneEntry {
  id: string;
  iana_id: string;
  display_label: string;
  sort_order: number;
  pinned_to_tray: boolean;
  created_at: number;
}

export interface TimezoneMetadata {
  id: string; // The primary ID (usually the IANA id, sometimes distinct)
  city: string;
  country: string;
  country_code: string;
  utc_offset_minutes: number;
  latitude: number;
  longitude: number;
  aliases: string[];
}
