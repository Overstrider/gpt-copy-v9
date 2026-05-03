import { NextRequest } from "next/server";

const configuredApiBaseUrl = process.env.API_BASE_URL;
const configuredApiAuthToken = process.env.API_AUTH_TOKEN;

if (process.env.NODE_ENV === "production" && !configuredApiBaseUrl) {
  throw new Error("API_BASE_URL must be set in production");
}

if (process.env.NODE_ENV === "production" && !configuredApiAuthToken) {
  throw new Error("API_AUTH_TOKEN must be set in production");
}

const BACKEND_API_URL = configuredApiBaseUrl ?? "http://localhost:3001";

type RouteContext = {
  params: {
    path: string[];
  };
};

const HOP_BY_HOP_HEADERS = [
  "connection",
  "keep-alive",
  "proxy-authenticate",
  "proxy-authorization",
  "te",
  "trailer",
  "transfer-encoding",
  "upgrade",
];

async function proxyToBackend(request: NextRequest, context: RouteContext) {
  if (!configuredApiAuthToken) {
    return Response.json(
      { code: "CONFIGURATION_ERROR", message: "API_AUTH_TOKEN must be set" },
      { status: 500 }
    );
  }

  const incomingUrl = new URL(request.url);
  const targetPath = `/api/${context.params.path.join("/")}${incomingUrl.search}`;
  const targetUrl = new URL(targetPath, BACKEND_API_URL);
  const headers = new Headers(request.headers);
  for (const header of HOP_BY_HOP_HEADERS) {
    headers.delete(header);
  }
  headers.delete("host");
  headers.set("Authorization", `Bearer ${configuredApiAuthToken}`);

  const body = ["GET", "HEAD"].includes(request.method)
    ? undefined
    : await request.arrayBuffer();

  const upstream = await fetch(targetUrl, {
    method: request.method,
    headers,
    body,
    cache: "no-store",
  });

  const responseHeaders = new Headers(upstream.headers);
  for (const header of HOP_BY_HOP_HEADERS) {
    responseHeaders.delete(header);
  }

  return new Response(upstream.body, {
    status: upstream.status,
    statusText: upstream.statusText,
    headers: responseHeaders,
  });
}

export function GET(request: NextRequest, context: RouteContext) {
  return proxyToBackend(request, context);
}

export function POST(request: NextRequest, context: RouteContext) {
  return proxyToBackend(request, context);
}

export function PATCH(request: NextRequest, context: RouteContext) {
  return proxyToBackend(request, context);
}

export function DELETE(request: NextRequest, context: RouteContext) {
  return proxyToBackend(request, context);
}
