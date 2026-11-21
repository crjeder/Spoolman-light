import { test, expect } from '@playwright/test';

/**
 * Playwright cannot set <input type="color"> via fill() in headless Chromium,
 * and Leptos listens for 'change'. Set the value and fire both events.
 */
async function setRowColor(
  page: import('@playwright/test').Page,
  index: number,
  hex: string,
): Promise<void> {
  await page.evaluate(
    ({ i, h }) => {
      const inputs = document.querySelectorAll(
        '.color-rows .color-alpha-row input[type="color"]',
      );
      const input = inputs[i] as HTMLInputElement | undefined;
      if (!input) throw new Error(`color input ${i} not found`);
      input.value = h;
      input.dispatchEvent(new Event('input', { bubbles: true }));
      input.dispatchEvent(new Event('change', { bubbles: true }));
    },
    { i: index, h: hex },
  );
}

test.describe('Multi-color spool edit', () => {
  test('add a second color in the edit form and see both on the detail page', async ({ page }) => {
    await page.goto('/spools');
    await page.waitForLoadState('networkidle');
    await page.locator('table.data-table').waitFor({ state: 'visible' });

    const editLink = page
      .locator('table.data-table tbody tr')
      .first()
      .locator('a[href*="/edit"]');
    const href = await editLink.getAttribute('href');
    const id = href!.match(/\/spools\/(\d+)\/edit/)![1];

    await editLink.click();
    await page.waitForURL('**/spools/*/edit');
    await page.waitForLoadState('networkidle');
    await page.locator('details.advanced > summary').click();

    const rows =page.locator('.color-rows .color-alpha-row');
    await rows.first().waitFor({ state: 'visible' });
    const initialCount = await rows.count();
    test.skip(initialCount >= 4, 'first spool already has the max number of colors');

    await page.locator('.btn-color-row[title="Add color"]').click();
    await expect(rows).toHaveCount(initialCount + 1);

    await setRowColor(page, 0, '#ff0000');
    await setRowColor(page, initialCount, '#00ff00');
    await page.waitForTimeout(100);

    await page.click('button[type="submit"]');
    await page.waitForURL('**/spools');

    await page.goto(`/spools/${id}`);
    await page.waitForLoadState('networkidle');

    const hexes = await page.locator('dd .color-hex').allTextContents();
    expect(hexes).toContain('#ff0000');
    expect(hexes).toContain('#00ff00');
  });
});
