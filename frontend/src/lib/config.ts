const configuredApiBaseUrl = process.env.NEXT_PUBLIC_API_URL;
const configuredApiAuthToken = process.env.NEXT_PUBLIC_API_AUTH_TOKEN;

if (process.env.NODE_ENV === "production" && !configuredApiBaseUrl) {
  throw new Error("NEXT_PUBLIC_API_URL must be set in production");
}

if (process.env.NODE_ENV === "production" && !configuredApiAuthToken) {
  throw new Error("NEXT_PUBLIC_API_AUTH_TOKEN must be set in production");
}

export const API_BASE_URL = configuredApiBaseUrl ?? "http://localhost:3001";

export const API_AUTH_HEADERS: Record<string, string> = configuredApiAuthToken
  ? { Authorization: `Bearer ${configuredApiAuthToken}` }
  : {};
