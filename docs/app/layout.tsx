import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "MPP-NEAR - Machine Payments Protocol for NEAR",
  description:
    "Open protocol for machine-to-machine payments on NEAR blockchain",
  openGraph: {
    type: "website",
    title: "MPP-NEAR - Machine Payments Protocol for NEAR",
    description:
      "Open protocol for machine-to-machine payments on NEAR blockchain",
    url: "https://mpp-near-website.vercel.app/",
    siteName: "MPP-NEAR",
    images: [
      {
        url: "https://mpp-near-website.vercel.app/og.png",
        width: 1200,
        height: 630,
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "MPP-NEAR - Machine Payments Protocol for NEAR",
    description:
      "Open protocol for machine-to-machine payments on NEAR blockchain",
    images: ["https://mpp-near-website.vercel.app/og.png"],
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
