import { test, expect } from '@playwright/test';

test('home page serves the Spoolman HTML shell', async ({ page }) => {
  const response = await page.goto('/');
  expect(response?.ok()).toBe(true);
  await expect(page).toHaveTitle(/Spoolman/);
});

test('nav links reach all main sections', async ({ page }) => {
  await page.goto('/spools');
  await expect(page.locator('h1')).toContainText('Spools');

  await page.goto('/filaments');
  await expect(page.locator('h1')).toContainText('Filaments');

  await page.goto('/locations');
  await expect(page.locator('h1')).toContainText('Locations');
});

test('clicking Spools nav link navigates correctly', async ({ page }) => {
  await page.goto('/filaments');
  await page.getByRole('link', { name: 'Spools' }).click();
  await expect(page).toHaveURL(/\/spools/);
  await expect(page.locator('h1')).toContainText('Spools');
});

test('clicking Filaments nav link navigates correctly', async ({ page }) => {
  await page.goto('/spools');
  await page.getByRole('link', { name: 'Filaments' }).click();
  await expect(page).toHaveURL(/\/filaments/);
  await expect(page.locator('h1')).toContainText('Filaments');
});
