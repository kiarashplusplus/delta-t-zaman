export function trapFocus(element: HTMLElement) {
  const focusableEls = element.querySelectorAll<HTMLElement>(
    'a[href], button, textarea, input[type="text"], input[type="radio"], input[type="checkbox"], select, [tabindex]:not([tabindex="-1"])'
  );
  
  if (focusableEls.length === 0) return () => {};

  const firstFocusableEl = focusableEls[0];
  const lastFocusableEl = focusableEls[focusableEls.length - 1];

  const handleKeyDown = (e: KeyboardEvent) => {
    const isTabPressed = e.key === 'Tab' || e.keyCode === 9;

    if (!isTabPressed) {
      return;
    }

    if (e.shiftKey) { 
      if (document.activeElement === firstFocusableEl) {
        lastFocusableEl.focus();
        e.preventDefault();
      }
    } else { 
      if (document.activeElement === lastFocusableEl) {
        firstFocusableEl.focus();
        e.preventDefault();
      }
    }
  };

  element.addEventListener('keydown', handleKeyDown);
  return () => {
    element.removeEventListener('keydown', handleKeyDown);
  };
}

export function setupMatchMediaListeners(
  onReducedMotionChange: (matches: boolean) => void,
  onContrastChange: (matches: boolean) => void
) {
  const motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
  onReducedMotionChange(motionQuery.matches);
  const motionListener = (e: MediaQueryListEvent) => onReducedMotionChange(e.matches);
  motionQuery.addEventListener('change', motionListener);

  const contrastQuery = window.matchMedia('(prefers-contrast: more)');
  onContrastChange(contrastQuery.matches);
  const contrastListener = (e: MediaQueryListEvent) => onContrastChange(e.matches);
  contrastQuery.addEventListener('change', contrastListener);

  return () => {
    motionQuery.removeEventListener('change', motionListener);
    contrastQuery.removeEventListener('change', contrastListener);
  };
}

export function announceToScreenReader(message: string) {
  let announcer = document.getElementById('aria-announcer');
  if (!announcer) {
    announcer = document.createElement('div');
    announcer.id = 'aria-announcer';
    announcer.className = 'sr-only';
    announcer.setAttribute('aria-live', 'polite');
    announcer.setAttribute('aria-atomic', 'true');
    document.body.appendChild(announcer);
  }
  announcer.textContent = message;
}
