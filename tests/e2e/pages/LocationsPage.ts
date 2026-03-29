import { type Page, type Locator } from '@playwright/test';

export class LocationsPage {
  readonly page: Page;
  readonly table: Locator;
  readonly rows: Locator;
  /** Inline create form input. */
  readonly nameInput: Locator;

  constructor(page: Page) {
    this.page = page;
    this.table = page.locator('table.data-table');
    this.rows = page.locator('table.data-table tbody tr');
    this.nameInput = page.locator('form.inline-create input[type="text"]');
  }

  async goto(): Promise<void> {
    await this.page.goto('/locations');
    await this.waitForReady();
  }

  async waitForReady(): Promise<void> {
    await this.page.waitForLoadState('networkidle');
    await this.table.waitFor({ state: 'visible' });
  }

  /** Returns the count of location rows currently displayed in the table. */
  async getLocationRows(): Promise<number> {
    return this.rows.count();
  }

  /**
   * Create a new location using the inline create form.
   */
  async addLocation(name: string): Promise<void> {
    await this.nameInput.fill(name);
    await this.page.click('form.inline-create button[type="submit"]'); // "Add"
    await this.page.waitForLoadState('networkidle');
    // Wait for the new row to appear in the table.
    await this.page.waitForFunction(
      (n: string) => [...document.querySelectorAll('table.data-table tbody td span')].some(el => el.textContent === n),
      name,
    );
  }

  /**
   * Delete the location row whose name matches `name`.
   * Only works for locations with 0 spools (delete is disabled otherwise).
   * Clicks "Delete" → "Sure?" to confirm.
   */
  async deleteLocation(name: string): Promise<void> {
    // Find the row containing the location name.
    const row = this.rows.filter({ has: this.page.locator('td span', { hasText: name }) });
    const deleteBtn = row.locator('button.btn-danger').first();
    await deleteBtn.click();                                      // "Delete"
    const confirmBtn = row.locator('button.btn-danger').first();
    await confirmBtn.waitFor({ state: 'visible' });
    await confirmBtn.click();                                     // "Sure?"
    // Wait for the row to be removed from the DOM. waitForLoadState('networkidle')
    // is not reliable here: spawn_local defers the DELETE fetch so the network
    // can appear idle before the request even starts.
    await row.waitFor({ state: 'detached' });
  }
}
