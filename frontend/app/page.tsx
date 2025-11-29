// frontend/app/page.tsx

import MarkdownPage from "@/components/MarkdownPage";
import { getPageBySlug } from "@/lib/content";

export default async function HomePage() {
  const page = getPageBySlug("overview", []);

  if (!page) {
    return <div className="p-8 text-goni-text">README.md not found.</div>;
  }

  return <MarkdownPage page={page} />;
}
