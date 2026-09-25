import { test, expect, type Page } from '@playwright/test';

/** Creates a spool via the API. `remaining` is the filament left in grams. */
async function createSpool(page: Page, remaining: number): Promise<number> {
  const filaments = await (await page.request.get('/api/v1/filament')).json();
  const res = await page.request.post('/api/v1/spool', {
    data: {
      filament_id: filaments[0].id,
      colors: [{ r: 10, g: 20, b: 30, a: 255 }],
      initial_weight: 1200,
      net_weight: 1000,
    },
  });
  const id = (await res.json()).spool.id;
  await page.request.patch(`/api/v1/spool/${id}`, { data: { current_weight: 200 + remaining } });
  return id;
}

async function isArchived(page: Page, id: number): Promise<boolean> {
  return (await (await page.request.get(`/api/v1/spool/${id}`)).json()).spool.archived;
}

test.describe('Archive spool', () => {
  test('empty spool archives without warning, then unarchives', async ({ page }) => {
    const id = await createSpool(page, 0);
    await page.goto(`/spools/${id}`);
    await page.getByTitle('Archive', { exact: true }).click();
    await expect(page.getByTitle('Unarchive')).toBeVisible();
    expect(await isArchived(page, id)).toBe(true);

    await page.getByTitle('Unarchive').click();
    await expect(page.getByTitle('Archive', { exact: true })).toBeVisible();
    expect(await isArchived(page, id)).toBe(false);
  });

  test('non-empty spool warns; cancel keeps it, confirm archives', async ({ page }) => {
    const id = await createSpool(page, 500);
    await page.goto(`/spools/${id}`);

    await page.getByTitle('Archive', { exact: true }).click();
    await expect(page.getByText('Spool is not empty')).toBeVisible();
    expect(await isArchived(page, id)).toBe(false);

    await page.getByTitle('Cancel').click();
    await expect(page.getByText('Spool is not empty')).toHaveCount(0);
    expect(await isArchived(page, id)).toBe(false);

    await page.getByTitle('Archive', { exact: true }).click();
    await page.getByTitle('Confirm archive').click();
    await expect(page.getByTitle('Unarchive')).toBeVisible();
    expect(await isArchived(page, id)).toBe(true);
  });

  test('list row archive of non-empty spool needs confirmation', async ({ page }) => {
    const id = await createSpool(page, 500);
    await page.goto('/spools');
    const row = page.locator('table.data-table tbody tr', { has: page.locator(`a[href="/spools/${id}"]`) });
    await row.getByTitle('Archive', { exact: true }).click();
    expect(await isArchived(page, id)).toBe(false);
    await row.getByTitle('Spool is not empty - archive anyway?').click();
    await expect(row).toHaveCount(0);
    expect(await isArchived(page, id)).toBe(true);
  });
});
