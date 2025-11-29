// frontend/components/MarkdownPage.tsx

import React from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import type { Page } from "@/lib/content";
import { getTocFromMarkdown, TocItem } from "@/lib/content";

type MarkdownPageProps = {
  page: Page;
};

function slugify(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^\w\s-]/g, "")
    .trim()
    .replace(/\s+/g, "-");
}

export default function MarkdownPage({ page }: MarkdownPageProps) {
  const toc: TocItem[] = getTocFromMarkdown(page.markdown);

  return (
    <div className="flex w-full gap-8">
      {/* Main content column */}
      <div className="flex-1 max-w-3xl">
        {/* Title */}
        {page.title && (
          <h1 className="text-3xl font-semibold mb-6 text-goni-text">
            {page.title}
          </h1>
        )}

        {/* Markdown content */}
        <article className="prose prose-invert max-w-none prose-headings:text-goni-text prose-p:text-goni-text prose-strong:text-goni-text prose-code:font-mono prose-a:text-goni-accent prose-a:no-underline hover:prose-a:underline">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            components={{
              // Add ids to headings so TOC hashes work
              h2({ node, children, ...props }) {
                const text = String(children);
                const id = slugify(text);
                return (
                  <h2 id={id} {...props}>
                    {children}
                  </h2>
                );
              },
              h3({ node, children, ...props }) {
                const text = String(children);
                const id = slugify(text);
                return (
                  <h3 id={id} {...props}>
                    {children}
                  </h3>
                );
              },
              h4({ node, children, ...props }) {
                const text = String(children);
                const id = slugify(text);
                return (
                  <h4 id={id} {...props}>
                    {children}
                  </h4>
                );
              },
              code({ inline, className, children, ...props }) {
                const match = /language-(\w+)/.exec(className || "");
                if (!inline) {
                  return (
                    <pre className="bg-[#020617] rounded-lg p-4 overflow-x-auto border border-goni-border">
                      <code className={className} {...props}>
                        {children}
                      </code>
                    </pre>
                  );
                }
                return (
                  <code className="bg-[#020617] rounded px-1.5 py-0.5 text-sm font-mono border border-goni-border">
                    {children}
                  </code>
                );
              },
              a({ children, ...props }) {
                return (
                  <a {...props} className="text-goni-accent hover:underline">
                    {children}
                  </a>
                );
              },
            }}
          >
            {page.markdown}
          </ReactMarkdown>
        </article>

        {/* GitHub link */}
        <div className="mt-8 pt-4 border-t border-goni-border text-sm text-goni-text-muted">
          <a
            href={page.githubUrl}
            target="_blank"
            rel="noreferrer"
            className="inline-flex items-center gap-2 text-goni-accent hover:underline"
          >
            <span>View this page on GitHub</span>
            <span aria-hidden>?</span>
          </a>
        </div>
      </div>

      {/* TOC column (only on large screens, optional) */}
      {toc.length > 0 && (
        <nav className="hidden lg:block w-56 shrink-0 border-l border-goni-border pl-4">
          <div className="text-xs font-semibold tracking-wide text-goni-text-muted mb-2">
            ON THIS PAGE
          </div>
          <ul className="space-y-1 text-sm text-goni-text-muted">
            {toc.map((item) => (
              <li key={item.id} className={item.depth > 2 ? "ml-4" : ""}>
                <a
                  href={`#${item.id}`}
                  className="hover:text-goni-text hover:underline"
                >
                  {item.text}
                </a>
              </li>
            ))}
          </ul>
        </nav>
      )}
    </div>
  );
}
