import { test, expect, Page } from "@playwright/test";

// Mock backend API responses
async function setupMocks(page: Page) {
  // Mock conversations list
  await page.route("**/api/conversations", async (route) => {
    if (route.request().method() === "GET") {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify([
          {
            id: "c1",
            title: "Test Chat",
            created_at: "2024-01-01T00:00:00Z",
            updated_at: "2024-01-01T00:00:00Z",
          },
        ]),
      });
    } else if (route.request().method() === "POST") {
      await route.fulfill({
        status: 201,
        contentType: "application/json",
        body: JSON.stringify({
          id: "c-new",
          title: "New Chat",
          created_at: "2024-01-01T00:00:00Z",
          updated_at: "2024-01-01T00:00:00Z",
        }),
      });
    } else {
      await route.continue();
    }
  });

  // Mock messages list
  await page.route("**/api/conversations/*/messages", async (route) => {
    if (route.request().method() === "GET") {
      await route.fulfill({
        status: 200,
        contentType: "application/json",
        body: JSON.stringify([]),
      });
    } else if (route.request().method() === "POST") {
      await route.fulfill({
        status: 201,
        contentType: "application/json",
        body: JSON.stringify({
          id: "m1",
          conversation_id: "c1",
          role: "user",
          content: "Hello",
          created_at: "2024-01-01T00:00:00Z",
        }),
      });
    } else {
      await route.continue();
    }
  });

  // Mock stream endpoint with deterministic SSE
  await page.route("**/api/conversations/*/stream", async (route) => {
    const sseBody =
      'data: {"type":"delta","content":"Hello"}\n\n' +
      'data: {"type":"delta","content":" world"}\n\n' +
      'data: {"type":"done"}\n\n';
    await route.fulfill({
      status: 200,
      contentType: "text/event-stream",
      body: sseBody,
    });
  });
}

test.describe("Chat smoke test", () => {
  test.beforeEach(async ({ page }) => {
    await setupMocks(page);
    await page.goto("/");
  });

  test("app loads with chat shell", async ({ page }) => {
    await expect(page.getByTestId("chat-shell")).toBeVisible();
    await expect(page.getByTestId("sidebar")).toBeVisible();
  });

  test("sidebar shows conversations", async ({ page }) => {
    await expect(page.getByTestId("sidebar")).toBeVisible();
    await expect(page.getByText("Test Chat")).toBeVisible();
  });

  test("can select a conversation", async ({ page }) => {
    await page.getByText("Test Chat").click();
    await expect(page.getByTestId("transcript")).toBeVisible();
    await expect(page.getByTestId("composer")).toBeVisible();
  });

  test("can create new conversation", async ({ page }) => {
    await page.getByTestId("new-chat-btn").click();
    await expect(page.getByTestId("transcript")).toBeVisible();
  });

  test("can send a message via stream", async ({ page }) => {
    await page.getByText("Test Chat").click();
    const composer = page.getByTestId("composer-textarea");
    await composer.fill("Hello");
    await page.getByTestId("composer-send").click();
  });
});

test.describe("Mobile sidebar interaction", () => {
  test.use({ viewport: { width: 390, height: 844 } });

  test("mobile sidebar toggle works", async ({ page }) => {
    await setupMocks(page);
    await page.goto("/");

    // On mobile, sidebar toggle should be visible
    const toggle = page.getByTestId("sidebar-toggle");
    await expect(toggle).toBeVisible();

    // Open sidebar
    await toggle.click();
    await expect(page.getByTestId("sidebar-panel")).toBeVisible();

    // Close sidebar via overlay
    const overlay = page.getByTestId("sidebar-overlay");
    if (await overlay.isVisible()) {
      await overlay.click();
    }
  });
});
