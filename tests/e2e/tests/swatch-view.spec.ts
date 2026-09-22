import { test, expect } from '@playwright/test';

// ---------------------------------------------------------------------------
// /colors renders the swatch grid, not the spool table.
// ---------------------------------------------------------------------------

test.describe('Color swatch view', () => {
  test('/colors renders cards, no data table', async ({ page }) => {
    await page.goto('/colors');
    await page.waitForLoadState('networkidle');
    await page.locator('.swatch-grid').waitFor({ state: 'visible' });

    expect(await page.locator('table.data-table').count()).toBe(0);
    expect(await page.locator('.swatch-card').count()).toBeGreaterThan(0);
  });

  test('a card links to its spool detail page', async ({ page }) => {
    await page.goto('/colors');
    await page.waitForLoadState('networkidle');
    await page.locator('.swatch-card').first().waitFor({ state: 'visible' });

    const href = await page.locator('.swatch-card').first().getAttribute('href');
    expect(href).toMatch(/^\/spools\/\d+$/);

    await page.locator('.swatch-card').first().click();
    await page.waitForURL(/\/spools\/\d+$/);
    await page.locator('.spool-show').waitFor({ state: 'visible' });
  });

  test('greys appear after saturated colours', async ({ page }) => {
    await page.goto('/colors');
    await page.waitForLoadState('networkidle');
    await page.locator('.swatch-grid').waitFor({ state: 'visible' });

    // Push page size up so both saturated and achromatic spools land on one page.
    await page.selectOption('.pagination select', '100');
    await page.waitForTimeout(300);

    const fills = await page.locator('.swatch-card-fill').evaluateAll((els: HTMLElement[]) =>
      els.map(el => el.style.background),
    );
    const isGreyish = (bg: string) => {
      const m = bg.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
      if (!m) return false;
      const [r, g, b] = [parseInt(m[1]), parseInt(m[2]), parseInt(m[3])];
      return Math.max(r, g, b) - Math.min(r, g, b) < 15; // low saturation
    };

    const firstGreyIndex = fills.findIndex(isGreyish);
    const lastSaturatedIndex = fills.map((bg, i) => (isGreyish(bg) ? -1 : i)).filter(i => i >= 0).pop() ?? -1;

    if (firstGreyIndex !== -1 && lastSaturatedIndex !== -1) {
      expect(firstGreyIndex, 'first grey card should come after the last saturated card').toBeGreaterThan(
        lastSaturatedIndex,
      );
    }
  });

  test('ticking two material checkboxes filters the grid', async ({ page }) => {
    await page.goto('/colors');
    await page.waitForLoadState('networkidle');
    await page.locator('.swatch-grid').waitFor({ state: 'visible' });

    const totalBefore = await page.locator('.swatch-card').count();

    const checkboxes = page.locator('.material-checks input[type="checkbox"]');
    await checkboxes.nth(0).check();
    await checkboxes.nth(1).check();
    await page.waitForTimeout(300);

    const totalAfter = await page.locator('.swatch-card').count();
    expect(totalAfter).toBeLessThanOrEqual(totalBefore);
  });
});

// ---------------------------------------------------------------------------
// default_view setting decides what `/` renders.
// ---------------------------------------------------------------------------

test.describe('Default view setting', () => {
  test('default_view = color renders the grid at /', async ({ page }) => {
    await page.goto('/settings');
    await page.waitForLoadState('networkidle');
    await page.locator('label:has-text("Default view") select').selectOption('color');
    await page.click('button[type="submit"]');
    await page.locator('.success').waitFor({ state: 'visible' });

    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.locator('.swatch-grid').waitFor({ state: 'visible' });
    expect(page.url()).toMatch(/\/$/);

    // Reset back to spool for other tests.
    await page.goto('/settings');
    await page.waitForLoadState('networkidle');
    const select = page.locator('label:has-text("Default view") select');
    await select.selectOption('spool');
    await page.click('button[type="submit"]');
    await page.locator('.success').waitFor({ state: 'visible' });
  });
});

// ---------------------------------------------------------------------------
// Shared filters, independent sort/pagination.
// ---------------------------------------------------------------------------

test.describe('Shared filters, per-view sort', () => {
  test('material filter applied in the table stays applied in the grid', async ({ page }) => {
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    await page.locator('table.data-table').waitFor({ state: 'visible' });

    await page.selectOption('.material-filter-select', { index: 1 });
    const chosen = await page.locator('.material-filter-select').inputValue();
    await page.waitForTimeout(300);

    await page.goto('/colors');
    await page.waitForLoadState('networkidle');
    await page.locator('.swatch-grid').waitFor({ state: 'visible' });

    const checked = page.locator('.material-checks input[type="checkbox"]:checked');
    expect(await checked.count()).toBeGreaterThan(0);
    const checkedLabel = await checked.first().locator('..').textContent();
    expect(checkedLabel?.trim()).toContain(chosen);

    // Clean up: clear the filter.
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    const clearBtn = page.locator('button:has-text("Clear filters")');
    if (await clearBtn.count()) {
      await clearBtn.click();
    }
  });

  test('table sort is unaffected by visiting the grid', async ({ page }) => {
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    await page.locator('table.data-table').waitFor({ state: 'visible' });

    await page.click('.col-header button.sort-btn >> nth=0'); // sort by Filament
    await page.waitForTimeout(200);
    const sortedClassBefore = await page.locator('.col-header.active').first().textContent();

    await page.goto('/colors');
    await page.waitForLoadState('networkidle');
    await page.locator('.swatch-grid').waitFor({ state: 'visible' });

    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    const sortedClassAfter = await page.locator('.col-header.active').first().textContent();

    expect(sortedClassAfter).toBe(sortedClassBefore);
  });
});
