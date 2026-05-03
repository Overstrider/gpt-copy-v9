import type { Metadata } from "next";
import "./globals.css";
import { QueryProvider } from "@/providers/query-provider";

export const metadata: Metadata = {
  title: "gpt-copy-v9",
  description: "ChatGPT-style application",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="h-full">
      <body className="h-full bg-gray-950 text-gray-100">
        <QueryProvider>{children}</QueryProvider>
      </body>
    </html>
  );
}
