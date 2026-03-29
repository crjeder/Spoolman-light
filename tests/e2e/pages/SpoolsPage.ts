import { type Page, type Locator } from '@playwright/test';

export interface SpoolData {
  /** Must match a filament display name visible in the dropdown. */
  filamentIndex?: number;
  initialWeight?: string;
  /** Must match a location name visible in the dropdown. Pass undefined to skip (location required). */
  locationIndex?: number;
  colorName?: string;
}

export class SpoolsPage {
  readonly page: Page;
  readonly table: Locator;
  readonly rows: Locator;

  constructor(page: Page) {
    this.page = page;
    this.table = page.locator('table.data-table');
    this.rows = page.locator('table.data-table tbody tr');
  }

  async goto(): Promise<void> {
    await this.page.goto('/spools');
    await this.waitForReady();
  }

  async waitForReady(): Promise<void> {
    await this.page.waitForLoadState('networkidle');
    await this.table.waitFor({ state: 'visible' });
  }

  /** Returns the count of visible (non-archived) spool rows in the table. */
  async getSpoolRows(): Promise<number> {
    return this.rows.count();
  }

  /** Returns the total spool count from the X-Total-Count response header. */
  async getSpoolCount(): Promise<number> {
    const response = await this.page.request.get('/api/v1/spool');
    const header = response.headers()['x-total-count'];
    return parseInt(header, 10);
  }

  /**
   * Navigate to the new-spool form, fill it in, and submit.
   * The form requires a location; `locationIndex` (0-based) selects from the location dropdown.
   * Defaults to the first available filament and first available location.
   */
  async addSpool(data: SpoolData = {}): Promise<void> {
    await this.page.click('a.btn-primary');                      // "+ New Spool"
    await this.page.waitForURL('**/spools/new');
    await this.page.waitForLoadState('networkidle');

    // Pick filament (select by index if specified, otherwise leave as-is).
    const filamentSelect = this.page.locator('.spool-create select').first();
    await filamentSelect.waitFor({ state: 'visible' });
    if (data.filamentIndex !== undefined) {
      await filamentSelect.selectOption({ index: data.filamentIndex });
    }

    if (data.initialWeight) {
      await this.page.fill('input[type="number"][step="0.1"]', data.initialWeight);
    }

    if (data.colorName) {
      await this.page.fill('input[type="text"]', data.colorName);
    }

    // Pick location (required by the form).
    const locationSelect = this.page.locator('.spool-create select').nth(1);
    await locationSelect.waitFor({ state: 'visible' });
    const index = data.locationIndex ?? 1;   // index 0 is "— none —"
    await locationSelect.selectOption({ index });

    await this.page.click('button[type="submit"]');              // "Create"
    // After create, navigates to /spools/:id
    await this.page.waitForURL(/\/spools\/\d+$/);
    await this.page.goto('/spools');
    await this.waitForReady();
  }

  /**
   * Delete the first spool row in the table. Clicks "Delete" → "Sure?" to confirm.
   */
  async deleteFirstSpool(): Promise<void> {
    const deleteBtn = this.rows.first().locator('button.btn-danger').first();
    await deleteBtn.click();                                     // "Delete"
    const confirmBtn = this.rows.first().locator('button.btn-danger').first();
    await confirmBtn.waitFor({ state: 'visible' });
    await confirmBtn.click();                                    // "Sure?"
    await this.page.waitForLoadState('networkidle');
  }
}
