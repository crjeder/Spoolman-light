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
});
