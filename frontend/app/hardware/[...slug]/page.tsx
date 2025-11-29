import { getPageBySlug } from "@/lib/content";
import MarkdownPage from "@/components/MarkdownPage";

interface HardwarePageProps {
  params: { slug?: string[] };
}

export default async function HardwarePage({ params }: HardwarePageProps) {
  const slug = params.slug ?? [];
  const page = getPageBySlug("hardware", slug);

  if (!page) {
    return <div className="p-8 text-goni-text">Page not found.</div>;
  }

  return <MarkdownPage page={page} />;
}
