import { test, expect } from '@playwright/test';

test.describe('Locations page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/locations');
    await expect(page.locator('h1')).toContainText('Locations');
  });

  test('renders locations page without error', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Locations');
    // Should not show a raw macro source or error
    await expect(page.locator('body')).not.toContainText('{{ ');
  });

  test('Add Location button is visible', async ({ page }) => {
    await expect(
      page.getByRole('button', { name: /add|new/i }).or(page.getByRole('link', { name: /add|new/i }))
    ).toBeVisible();
  });
});
