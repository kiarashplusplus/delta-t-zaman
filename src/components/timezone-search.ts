import { searchTimezones } from '../services/timezone-search';
import { getClocks, addClock } from '../state/clocks';
import { subscribe } from '../state/event-bus';
import type { TimezoneMetadata, TimeZoneEntry } from '../types/timezone';

export class TimezoneSearchModal {
  private element: HTMLElement;
  private input: HTMLInputElement;
  private resultsContainer: HTMLElement;
  private warningElement: HTMLElement;
  
  private results: TimezoneMetadata[] = [];
  private selectedIndex: number = -1;

  constructor() {
    this.element = document.createElement('div');
    this.element.className = 'search-modal';
    this.element.style.display = 'none';

    const content = document.createElement('div');
    content.className = 'search-modal__content';

    this.input = document.createElement('input');
    this.input.type = 'text';
    this.input.className = 'search-modal__input';
    this.input.placeholder = 'Search timezone (e.g., Tokyo, UTC+9)';
    
    this.warningElement = document.createElement('div');
    this.warningElement.className = 'search-modal__warning';
    this.warningElement.style.display = 'none';

    this.resultsContainer = document.createElement('ul');
    this.resultsContainer.className = 'search-modal__results';

    content.appendChild(this.input);
    content.appendChild(this.warningElement);
    content.appendChild(this.resultsContainer);
    this.element.appendChild(content);

    document.body.appendChild(this.element);

    this.bindEvents();
    
    subscribe('OPEN_SEARCH_MODAL', () => {
      this.open();
    });
  }

  private bindEvents() {
    document.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && (e.key === 'k' || e.key === 'n')) {
        e.preventDefault();
        this.open();
      }
    });

    this.element.addEventListener('click', (e) => {
      if (e.target === this.element) {
        this.close();
      }
    });

    this.input.addEventListener('input', () => {
      this.handleSearch();
    });

    this.input.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        this.close();
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        this.moveSelection(1);
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        this.moveSelection(-1);
      } else if (e.key === 'Enter') {
        e.preventDefault();
        this.addSelected();
      }
    });
  }

  private open() {
    this.element.style.display = 'flex';
    this.input.value = '';
    this.results = [];
    this.selectedIndex = -1;
    this.warningElement.style.display = 'none';
    this.renderResults();
    this.input.focus();
  }

  private close() {
    this.element.style.display = 'none';
  }

  private handleSearch() {
    const query = this.input.value;
    this.results = searchTimezones(query);
    this.selectedIndex = this.results.length > 0 ? 0 : -1;
    this.warningElement.style.display = 'none';
    this.renderResults();
  }

  private moveSelection(delta: number) {
    if (this.results.length === 0) return;
    this.selectedIndex += delta;
    if (this.selectedIndex < 0) this.selectedIndex = 0;
    if (this.selectedIndex >= this.results.length) this.selectedIndex = this.results.length - 1;
    this.renderResults();
  }

  private renderResults() {
    this.resultsContainer.innerHTML = '';
    this.results.forEach((tz, index) => {
      const li = document.createElement('li');
      li.className = 'search-modal__result-item';
      if (index === this.selectedIndex) {
        li.classList.add('selected');
      }
      
      const offsetHours = tz.utc_offset_minutes / 60;
      const offsetStr = offsetHours >= 0 ? `+${offsetHours}` : `${offsetHours}`;
      
      li.textContent = `${tz.city || tz.id} (${tz.country || 'N/A'}) UTC${offsetStr}`;
      li.addEventListener('click', () => {
        this.selectedIndex = index;
        this.addSelected();
      });
      li.addEventListener('mouseenter', () => {
        this.selectedIndex = index;
        this.renderResults();
      });
      this.resultsContainer.appendChild(li);
    });
  }

  private async addSelected() {
    if (this.selectedIndex >= 0 && this.selectedIndex < this.results.length) {
      const selectedTz = this.results[this.selectedIndex];
      const existingClocks = getClocks();
      
      if (existingClocks.some(c => c.iana_id === selectedTz.id)) {
        this.warningElement.textContent = 'This timezone is already in your list.';
        this.warningElement.style.display = 'block';
        return;
      }
      
      if (existingClocks.length >= 30) {
        this.warningElement.textContent = 'Maximum of 30 timezones reached.';
        this.warningElement.style.display = 'block';
        return;
      }

      const newClock: TimeZoneEntry = {
        id: crypto.randomUUID(),
        iana_id: selectedTz.id,
        display_label: selectedTz.city || selectedTz.id,
        sort_order: existingClocks.length,
        pinned_to_tray: false,
        created_at: Date.now()
      };

      await addClock(newClock);
      this.close();
    }
  }
}
