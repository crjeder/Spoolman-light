import { test, expect } from '@playwright/test';

test.describe('Filters reset on menu change', () => {
  test('color tab material filter does not leak into spools tab', async ({ page }) => {
    await page.goto('/colors');
    await page.locator('.swatch-grid').waitFor({ state: 'visible' });
    // Untick one material so the shared filter differs from "all".
    await page.locator('.material-checks input[type="checkbox"]').first().uncheck();

    await page.locator('.nav-links a[href="/spools"]').click();
    await page.locator('table.data-table').waitFor({ state: 'visible' });
    expect(await page.locator('.material-filter-select').inputValue()).toBe('');
    await expect(page.locator('button:has-text("Clear filters")')).toHaveCount(0);
  });

  test('filter survives a detail-page round trip', async ({ page }) => {
    await page.goto('/spools');
    await page.locator('table.data-table').waitFor({ state: 'visible' });
    await page.selectOption('.material-filter-select', { index: 1 });
    const chosen = await page.locator('.material-filter-select').inputValue();

    await page.locator('table.data-table a[href^="/spools/"]').first().click();
    await page.locator('.spool-show').waitFor({ state: 'visible' });
    await page.goBack();
    await page.locator('table.data-table').waitFor({ state: 'visible' });
    expect(await page.locator('.material-filter-select').inputValue()).toBe(chosen);
  });
});
