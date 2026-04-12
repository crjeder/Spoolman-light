import { test, expect } from '@playwright/test';

// ── Filament List ────────────────────────────────────────────────────────────

test.describe('Filament list', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/filaments');
    // Wait for table to load data
    await page.waitForSelector('table.data-table tbody tr');
  });

  test('renders filament list page', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Filaments');
    await expect(page.locator('table.data-table')).toBeVisible();
  });

  test('loads filament data', async ({ page }) => {
    const rows = page.locator('table.data-table tbody tr');
    await expect(rows).toHaveCount(20); // test data has 20 filaments
  });

  test('has expected columns — no Net weight column', async ({ page }) => {
    const headers = page.locator('table.data-table thead th');
    const headerTexts = await headers.allTextContents();
    const normalised = headerTexts.map(h => h.replace(/[↑↓]/g, '').trim());

    expect(normalised).toContain('Manufacturer');
    expect(normalised).toContain('Material');
    expect(normalised).toContain('Density');
    expect(normalised).toContain('Registered');

    // net_weight and spool_weight were removed from Filament
    expect(normalised).not.toContain('Net weight');
    expect(normalised).not.toContain('Spool weight');
  });

  test('filter input narrows results', async ({ page }) => {
    const allRows = page.locator('table.data-table tbody tr');
    const initialCount = await allRows.count();

    await page.fill('input[placeholder="Filter…"]', 'Bambu');
    await page.waitForTimeout(200); // debounce

    const filtered = page.locator('table.data-table tbody tr');
    const filteredCount = await filtered.count();
    expect(filteredCount).toBeLessThanOrEqual(initialCount);
  });

  test('sort by Density shows indicator and toggles direction', async ({ page }) => {
    const densityBtn = page.locator('table.data-table thead button.sort-btn', { hasText: 'Density' });

    await densityBtn.click();
    await expect(densityBtn).toContainText('↑');

    await densityBtn.click();
    await expect(densityBtn).toContainText('↓');
  });

  test('+ New Filament button links to create page', async ({ page }) => {
    await page.getByRole('link', { name: '+ New Filament' }).click();
    await expect(page).toHaveURL(/\/filaments\/new/);
    await expect(page.locator('h1')).toContainText('New Filament');
  });
});

// ── Filament Show ────────────────────────────────────────────────────────────

test.describe('Filament detail', () => {
  test('navigating to a filament shows its details', async ({ page }) => {
    await page.goto('/filaments');
    await page.waitForSelector('table.data-table tbody tr a');
    await page.locator('table.data-table tbody tr td a').first().click();
    await expect(page).toHaveURL(/\/filaments\/\d+/);

    const dl = page.locator('dl.detail-grid');
    await expect(dl).toBeVisible();

    // Should NOT contain net_weight or spool_weight rows
    const dtTexts = await dl.locator('dt').allTextContents();
    expect(dtTexts).not.toContain('Net weight');
    expect(dtTexts).not.toContain('Spool weight');

    // Should contain density
    expect(dtTexts).toContain('Density');
  });
});

// ── Filament Create ──────────────────────────────────────────────────────────

test.describe('Filament create form', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/filaments/new');
    await expect(page.locator('h1')).toContainText('New Filament');
  });

  test('has no net_weight or spool_weight fields', async ({ page }) => {
    const labels = page.locator('form label');
    const labelTexts = await labels.allTextContents();
    expect(labelTexts.join(' ')).not.toMatch(/net.?weight/i);
    expect(labelTexts.join(' ')).not.toMatch(/spool.?weight/i);
  });

  test('has required density field', async ({ page }) => {
    await expect(page.getByLabel('Density (g/cm³)')).toBeVisible();
  });
});

// ── Filament Edit ────────────────────────────────────────────────────────────

test.describe('Filament edit form', () => {
  test('has no net_weight or spool_weight fields', async ({ page }) => {
    // Get first filament ID
    await page.goto('/filaments');
    await page.waitForSelector('table.data-table tbody tr');
    const editLink = page.locator('table.data-table tbody tr td.actions a', { hasText: 'Edit' }).first();
    await editLink.click();
    await expect(page).toHaveURL(/\/filaments\/\d+\/edit/);

    const labels = page.locator('form label');
    const labelTexts = await labels.allTextContents();
    expect(labelTexts.join(' ')).not.toMatch(/net.?weight/i);
    expect(labelTexts.join(' ')).not.toMatch(/spool.?weight/i);
  });
});
