import { test, expect } from '@playwright/test';
import { FilamentsPage } from '../pages';

test.describe('Filaments', () => {
  test('fixture data is visible on load', async ({ page }) => {
    const filaments = new FilamentsPage(page);
    await filaments.goto();
    const count = await filaments.getFilamentRows();
    expect(count).toBeGreaterThanOrEqual(1);
  });

  test('add filament appears in list', async ({ page }) => {
    const filaments = new FilamentsPage(page);
    await filaments.goto();
    const before = await filaments.getFilamentCount();

    await filaments.addFilament({
      manufacturer: 'E2E Maker',
      material: 'PLA',
    });

    const after = await filaments.getFilamentCount();
    expect(after).toBe(before + 1);
  });

  test('delete filament removes it from list', async ({ page }) => {
    const filaments = new FilamentsPage(page);
    await filaments.goto();
    const before = await filaments.getFilamentCount();
    expect(before).toBeGreaterThanOrEqual(1);

    await filaments.deleteFirstFilament();

    const after = await filaments.getFilamentCount();
    expect(after).toBe(before - 1);
  });
});
