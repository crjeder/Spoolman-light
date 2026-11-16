import { test, expect } from '@playwright/test';
import { SpoolsPage } from '../pages';

test.describe('Spool location grouping', () => {
  test('Location header groups into collapsed rows with counts', async ({ page }) => {
    const spools = new SpoolsPage(page);
    await spools.goto();
    const flat = await spools.getSpoolRows();

    await page.locator('th.location-head .sort-btn').click();
    const groups = page.locator('tr.group-row');
    await expect(groups.first()).toBeVisible();
    await expect(groups.first()).toHaveText(/^\s*\+ .+: \d+ (entry|entries)\s*$/);
    expect(await spools.getSpoolRows()).toBe(await groups.count());
    expect(await groups.count()).toBeLessThan(flat);
  });

  test('clicking a group expands and collapses it', async ({ page }) => {
    const spools = new SpoolsPage(page);
    await spools.goto();
    await page.locator('th.location-head .sort-btn').click();

    const first = page.locator('tr.group-row').first();
    const n = Number((await first.innerText()).match(/: (\d+) /)![1]);
    const groupCount = await page.locator('tr.group-row').count();

    await first.click();
    await expect(first).toHaveText(/^\s*- /);
    await expect(spools.rows).toHaveCount(groupCount + n);

    await first.click();
    await expect(first).toHaveText(/^\s*\+ /);
    await expect(spools.rows).toHaveCount(groupCount);
  });

  test('active search filter is shown in group rows', async ({ page }) => {
    const spools = new SpoolsPage(page);
    await spools.goto();
    await page.locator('th.location-head .sort-btn').click();
    await page.getByPlaceholder('Search…').fill('pla');
    const first = page.locator('tr.group-row').first();
    await expect(first.locator('td').first()).toHaveText('pla');
  });

  test('sorting by another column restores the flat table', async ({ page }) => {
    const spools = new SpoolsPage(page);
    await spools.goto();
    await page.locator('th.location-head .sort-btn').click();
    await expect(page.locator('tr.group-row').first()).toBeVisible();
    await page.getByRole('button', { name: /^Filament/ }).click();
    await expect(page.locator('tr.group-row')).toHaveCount(0);
  });
});
