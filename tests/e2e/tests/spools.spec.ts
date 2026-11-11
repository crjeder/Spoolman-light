import { test, expect } from '@playwright/test';
import { SpoolsPage, LocationsPage } from '../pages';

test.describe('Spools', () => {
  test('fixture data is visible on load', async ({ page }) => {
    const spools = new SpoolsPage(page);
    await spools.goto();
    const count = await spools.getSpoolRows();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test('add spool appears in list', async ({ page }) => {
    // Ensure there is at least one location (required by spool create form).
    const locations = new LocationsPage(page);
    await locations.goto();
    const locCount = await locations.getLocationRows();
    if (locCount === 0) {
      await locations.addLocation('Test Location');
    }

    const spools = new SpoolsPage(page);
    await spools.goto();
    const before = await spools.getSpoolCount();

    await spools.addSpool({ initialWeight: '1000', colorName: 'E2E Test Color' });

    const after = await spools.getSpoolCount();
    expect(after).toBe(before + 1);
  });

  test('delete spool removes it from list', async ({ page }) => {
    const spools = new SpoolsPage(page);
    await spools.goto();
    const before = await spools.getSpoolCount();
    expect(before).toBeGreaterThanOrEqual(1);

    await spools.deleteFirstSpool();

    const after = await spools.getSpoolCount();
    expect(after).toBe(before - 1);
  });

  test.describe('edit form last used', () => {
    async function openEditWithLocation(page, request) {
      const res = await request.get('/api/v1/spool');
      const spool = (await res.json()).find((s) => !s.archived && s.location_id != null);
      expect(spool).toBeTruthy();
      await page.goto(`/spools/${spool.id}/edit`);
      await expect(page.getByLabel('Current weight (g)')).not.toHaveValue('');
      return spool.id;
    }

    test('saving a weight change sets last used to today', async ({ page, request }) => {
      const id = await openEditWithLocation(page, request);
      await page.getByLabel('Last used').fill('2020-01-01');
      await page.getByRole('button', { name: 'Save' }).click();
      await page.waitForURL('**/spools');

      await page.goto(`/spools/${id}/edit`);
      await expect(page.getByLabel('Last used')).toHaveValue('2020-01-01');
      await page.getByLabel('Current weight (g)').fill('123');
      await page.getByRole('button', { name: 'Save' }).click();
      await page.waitForURL('**/spools');

      const spool = await (await request.get(`/api/v1/spool/${id}`)).json();
      expect(spool.last_used.slice(0, 10)).toBe(new Date().toISOString().slice(0, 10));
    });

    test('manually chosen last used is kept', async ({ page, request }) => {
      const id = await openEditWithLocation(page, request);
      await page.getByLabel('Last used').fill('2020-02-02');
      await page.getByRole('button', { name: 'Save' }).click();
      await page.waitForURL('**/spools');

      const spool = await (await request.get(`/api/v1/spool/${id}`)).json();
      expect(spool.last_used.slice(0, 10)).toBe('2020-02-02');
    });
  });
});
