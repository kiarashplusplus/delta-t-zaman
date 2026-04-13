export class AboutPanel {
  public element: HTMLElement;

  constructor(container: HTMLElement) {
    this.element = document.createElement('div');
    this.element.className = 'about-panel';
    
    this.render();
    container.appendChild(this.element);
  }

  private render() {
    this.element.innerHTML = `
      <div class="about-section" style="text-align: center; margin-bottom: 2rem;">
        <h2>Tauri World Clock</h2>
        <p>Version: 1.0.0 (Beta)</p>
      </div>

      <div class="about-section">
        <h3>Licenses</h3>
        <p>This software uses various open source libraries.</p>
        <p>Released under the MIT License.</p>
      </div>

      <div class="about-section">
        <h3>Privacy Policy</h3>
        <p>
          <a href="#" id="link-privacy">View Privacy Policy</a>
        </p>
      </div>
    `;

    this.element.querySelector('#link-privacy')?.addEventListener('click', (e) => {
      e.preventDefault();
      console.log('Privacy policy opened');
    });
  }

  destroy() {
    this.element.remove();
  }
}
