import { type Page, type Locator } from '@playwright/test';

export interface FilamentData {
  manufacturer?: string;
  /** Material abbreviation, e.g. 'PLA', 'PETG'. Must match an option value in the material select. */
  material?: string;
  density?: string;
}

export class FilamentsPage {
  readonly page: Page;
  readonly table: Locator;
  readonly rows: Locator;

  constructor(page: Page) {
    this.page = page;
    this.table = page.locator('table.data-table');
    this.rows = page.locator('table.data-table tbody tr');
  }

  async goto(): Promise<void> {
    await this.page.goto('/filaments');
    await this.waitForReady();
  }

  async waitForReady(): Promise<void> {
    await this.page.waitForLoadState('networkidle');
    await this.table.waitFor({ state: 'visible' });
  }

  /** Returns the count of filament rows currently displayed in the table. */
  async getFilamentRows(): Promise<number> {
    return this.rows.count();
  }

  /** Returns the total filament count from the X-Total-Count response header. */
  async getFilamentCount(): Promise<number> {
    const response = await this.page.request.get('/api/v1/filament');
    const header = response.headers()['x-total-count'];
    return parseInt(header, 10);
  }

  /**
   * Navigate to the new-filament form, fill it in, and submit.
   */
  async addFilament(data: FilamentData = {}): Promise<void> {
    await this.page.click('a.btn-primary');                       // "+ New Filament"
    await this.page.waitForURL('**/filaments/new');
    await this.page.waitForLoadState('networkidle');

    if (data.manufacturer) {
      // First text input on the create page is "Manufacturer".
      await this.page.fill('.filament-create input[type="text"]', data.manufacturer);
    }

    if (data.material) {
      // The material <select> is inside .filament-create.
      await this.page.selectOption('.filament-create select', data.material);
    }

    if (data.density) {
      await this.page.fill('input[step="0.001"]', data.density);
    }

    await this.page.click('button[type="submit"]');               // "Create"
    // After create, navigates to /filaments/:id
    await this.page.waitForURL(/\/filaments\/\d+$/);
    await this.page.goto('/filaments');
    await this.waitForReady();
  }

  /**
   * Delete the first filament row in the table.
   * Clicks "Delete" → "Sure?" to confirm.
   */
  async deleteFirstFilament(): Promise<void> {
    const deleteBtn = this.rows.first().locator('button.btn-danger').first();
    await deleteBtn.click();                                      // "Delete"
    const confirmBtn = this.rows.first().locator('button.btn-danger').first();
    await confirmBtn.waitFor({ state: 'visible' });
    await confirmBtn.click();                                     // "Sure?"
    await this.page.waitForLoadState('networkidle');
  }
}
