import { getPageBySlug } from "@/lib/content";
import MarkdownPage from "@/components/MarkdownPage";

interface DocsPageProps {
  params: { slug?: string[] };
}

export default async function DocsPage({ params }: DocsPageProps) {
  const slug = params.slug ?? [];
  const page = getPageBySlug("docs", slug);

  if (!page) {
    return <div className="p-8 text-goni-text">Page not found.</div>;
  }

  return <MarkdownPage page={page} />;
}
