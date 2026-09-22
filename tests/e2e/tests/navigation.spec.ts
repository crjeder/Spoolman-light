import { test, expect } from '@playwright/test';

test.describe('Navigation', () => {
  test('Spools page loads', async ({ page }) => {
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Spools');
    await expect(page.locator('table.data-table')).toBeVisible();
  });

  test('Filaments page loads', async ({ page }) => {
    await page.goto('/filaments');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Filaments');
    await expect(page.locator('table.data-table')).toBeVisible();
  });

  test('Locations page loads', async ({ page }) => {
    await page.goto('/locations');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('h1')).toContainText('Locations');
    await expect(page.locator('table.data-table')).toBeVisible();
  });

  test('Sidebar nav links exist', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const nav = page.locator('nav.sidebar ul.nav-links');
    await expect(nav.locator('a', { hasText: 'Spools' })).toBeVisible();
    await expect(nav.locator('a', { hasText: 'Filaments' })).toBeVisible();
    await expect(nav.locator('a', { hasText: 'Locations' })).toBeVisible();
  });

  test('Clicking Filaments nav link navigates to /filaments', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('nav.sidebar a[href="/filaments"]');
    await page.waitForURL('**/filaments');
    await expect(page.locator('h1')).toContainText('Filaments');
  });

  test('Clicking Locations nav link navigates to /locations', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('nav.sidebar a[href="/locations"]');
    await page.waitForURL('**/locations');
    await expect(page.locator('h1')).toContainText('Locations');
  });
});

test.describe('Navigation at desktop width', () => {
  test('No burger button is shown', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('nav.sidebar')).toBeVisible();
    await expect(page.locator('button.burger')).toBeHidden();
  });
});

test.describe('Navigation at phone width', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Sidebar is hidden on load and the burger is shown', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await expect(page.locator('nav.sidebar ul.nav-links a').first()).toBeHidden();
    const burger = page.locator('button.burger');
    await expect(burger).toBeVisible();
    await expect(burger).toHaveAttribute('aria-expanded', 'false');
  });

  test('Burger opens the sidebar as an overlay', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    const main = page.locator('main.main-content');
    const before = await main.boundingBox();

    await page.click('button.burger');

    const nav = page.locator('nav.sidebar ul.nav-links');
    await expect(nav.locator('a', { hasText: 'Spools' })).toBeVisible();
    await expect(nav.locator('a', { hasText: 'Filaments' })).toBeVisible();
    await expect(nav.locator('a', { hasText: 'Locations' })).toBeVisible();
    await expect(page.locator('button.burger')).toHaveAttribute('aria-expanded', 'true');
    await expect(page.locator('.sidebar-backdrop')).toBeVisible();

    // Content is overlaid, not pushed aside.
    expect(await main.boundingBox()).toEqual(before);
  });

  test('Backdrop click closes the sidebar', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('button.burger');
    await expect(page.locator('.sidebar-backdrop')).toBeVisible();

    await page.locator('.sidebar-backdrop').click({ position: { x: 350, y: 400 } });

    await expect(page.locator('.sidebar-backdrop')).toBeHidden();
    await expect(page.locator('nav.sidebar ul.nav-links a').first()).toBeHidden();
    await expect(page.locator('button.burger')).toHaveAttribute('aria-expanded', 'false');
  });

  test('Escape closes the sidebar', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('button.burger');
    await expect(page.locator('nav.sidebar ul.nav-links a').first()).toBeVisible();

    await page.keyboard.press('Escape');

    await expect(page.locator('nav.sidebar ul.nav-links a').first()).toBeHidden();
  });

  test('Activating a nav entry navigates and closes the sidebar', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('button.burger');
    await page.click('nav.sidebar a[href="/filaments"]');

    await page.waitForURL('**/filaments');
    await expect(page.locator('h1')).toContainText('Filaments');
    await expect(page.locator('nav.sidebar ul.nav-links a').first()).toBeHidden();
  });

  test('Open state does not survive a reload', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.click('button.burger');
    await expect(page.locator('nav.sidebar ul.nav-links a').first()).toBeVisible();

    await page.reload();
    await page.waitForLoadState('networkidle');

    await expect(page.locator('nav.sidebar ul.nav-links a').first()).toBeHidden();
    await expect(page.locator('button.burger')).toHaveAttribute('aria-expanded', 'false');
  });
});
