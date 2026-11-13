import { test, expect } from '@playwright/test';

const profile = {
  name: 'Polymaker HT-PLA @User',
  filament_vendor: ['Polymaker'],
  filament_type: ['PLA'],
  filament_density: ['1.28'],
  filament_diameter: ['1.75'],
  nozzle_temperature: ['220'],
  bed_type: ['Cool Plate'],
  cool_plate_temp: ['60'],
};

test.describe('Filament profile import', () => {
  test('import populates the new filament form', async ({ page }) => {
    await page.goto('/filaments/new');
    await page.waitForLoadState('networkidle');

    await page.setInputFiles('input.profile-file', {
      name: 'profile.json',
      mimeType: 'application/json',
      buffer: Buffer.from(JSON.stringify(profile)),
    });

    const form = page.locator('.filament-create form');
    await expect(form.locator('select')).toHaveValue('PLA');
    await expect(form.locator('input[type="text"]').first()).toHaveValue('Polymaker');
    await expect(form.locator('input[step="0.001"]')).toHaveValue('1.28');
    const numbers = form.locator('input[type="number"]');
    // diameter, density, then min/max print, min/max bed
    await expect(numbers.nth(2)).toHaveValue('220');
    await expect(numbers.nth(3)).toHaveValue('220');
    await expect(numbers.nth(4)).toHaveValue('60');
    await expect(numbers.nth(5)).toHaveValue('60');
  });

  test('invalid file shows an error', async ({ page }) => {
    await page.goto('/filaments/new');
    await page.waitForLoadState('networkidle');

    await page.setInputFiles('input.profile-file', {
      name: 'bad.json',
      mimeType: 'application/json',
      buffer: Buffer.from('{"foo": 1}'),
    });

    await expect(page.locator('.filament-create p.error')).toBeVisible();
  });
});
