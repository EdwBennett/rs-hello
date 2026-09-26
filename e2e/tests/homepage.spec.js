// @ts-check
const { test, expect } = require("@playwright/test");

test("homepage shows two consecutive primes", async ({ page }) => {
  await page.goto("/");

  // The number pair is random and only appears once the browser has
  // fetched and run the WASM module (see site/templates/index.html), so
  // this exercises the whole Zola-build-plus-WASM pipeline, not just that
  // the page loads. toHaveText retries until init() has finished.
  await expect(page.locator("#primes")).toHaveText(
    /^Today's prime numbers are \d+ and \d+$/,
  );
});
