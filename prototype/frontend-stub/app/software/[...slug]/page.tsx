import { getPageBySlug } from "@/lib/content";
import MarkdownPage from "@/components/MarkdownPage";

interface SoftwarePageProps {
  params: { slug?: string[] };
}

export default async function SoftwarePage({ params }: SoftwarePageProps) {
  const slug = params.slug ?? [];
  const page = getPageBySlug("software", slug);

  if (!page) {
    return <div className="p-8 text-goni-text">Page not found.</div>;
  }

  return <MarkdownPage page={page} />;
}
