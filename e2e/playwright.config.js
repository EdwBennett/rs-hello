// @ts-check
const { defineConfig } = require("@playwright/test");

// Dedicated to this test suite's own throwaway `zola serve` (started by
// serve-site.sh, below) — distinct from the personal dev instance on :8080
// (see ../tools/rebuild-my-zola-site) and from zola's own interactive
// default of :1111, in case both happen to be running at the same time.
const PORT = 1191;

module.exports = defineConfig({
  testDir: "./tests",
  fullyParallel: true,
  use: {
    baseURL: `http://127.0.0.1:${PORT}`,
  },
  // Builds hello-core to WASM and starts a real `zola serve` before the
  // tests run, then stops it afterwards — nothing needs to be running
  // beforehand, and nothing personal-machine-specific is assumed.
  webServer: {
    command: "bash serve-site.sh",
    url: `http://127.0.0.1:${PORT}/`,
    timeout: 120_000,
    reuseExistingServer: !process.env.CI,
  },
});
