// frontend/app/layout.tsx

import "./globals.css";
import type { ReactNode } from "react";
import { getSections } from "@/lib/content";
import SidebarNav, { SectionNav } from "@/components/SidebarNav";

export const metadata = {
  title: "Goni",
  description: "Local-first AI node & mesh",
};

export default function RootLayout({ children }: { children: ReactNode }) {
  // This runs on the server at build/render time.
  const sectionsMap = getSections();

  const sections: SectionNav[] = [
    {
      id: "overview",
      label: "OVERVIEW",
      pages: sectionsMap.overview,
    },
    {
      id: "docs",
      label: "DOCS",
      pages: sectionsMap.docs,
    },
    {
      id: "hardware",
      label: "HARDWARE",
      pages: sectionsMap.hardware,
    },
    {
      id: "software",
      label: "SOFTWARE",
      pages: sectionsMap.software,
    },
  ];

  return (
    <html lang="en" className="h-full">
      <body className="h-full bg-goni-bg text-goni-text">
        <div className="flex h-screen">
          <SidebarNav sections={sections} />
          <main className="flex-1 overflow-y-auto px-8 py-6">{children}</main>
        </div>
      </body>
    </html>
  );
}
