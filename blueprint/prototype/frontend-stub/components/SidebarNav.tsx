// frontend/components/SidebarNav.tsx

"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import type { PageMeta, SectionId } from "@/lib/content";
import { getPageHref } from "@/lib/content";

export type SectionNav = {
  id: SectionId;
  label: string;
  pages: PageMeta[];
};

type SidebarNavProps = {
  sections: SectionNav[];
};

export default function SidebarNav({ sections }: SidebarNavProps) {
  const pathname = usePathname();

  return (
    <aside className="h-screen sticky top-0 flex flex-col border-r border-goni-border bg-goni-surface text-goni-text w-72">
      <div className="px-5 pt-5 pb-4 border-b border-goni-border">
        <Link href="/" className="text-xl font-semibold tracking-tight">
          Goni
        </Link>
        <p className="mt-1 text-xs text-goni-text-muted">
          Local-first AI node & mesh
        </p>
      </div>

      <nav className="flex-1 overflow-y-auto px-4 py-4 space-y-6">
        {sections.map((section) => {
          if (section.pages.length === 0 && section.id !== "overview") {
            return null; // hide empty sections except overview
          }

          return (
            <div key={section.id}>
              <div className="text-[0.65rem] font-semibold tracking-[0.15em] text-goni-text-muted uppercase mb-2">
                {section.label}
              </div>
              <ul className="space-y-1">
                {section.pages.map((page) => {
                  const href = getPageHref(page);
                  const isActive =
                    href === "/"
                      ? pathname === "/"
                      : pathname === href || pathname.startsWith(href + "/");

                  return (
                    <li key={page.githubUrl}>
                      <Link
                        href={href}
                        className={[
                          "flex items-center gap-2 rounded-md px-2 py-1.5 text-sm",
                          isActive
                            ? "bg-[#020617] border-l-2 border-goni-accent text-goni-text"
                            : "text-goni-text-muted hover:text-goni-text hover:bg-[#020617]/60",
                        ].join(" ")}
                      >
                        <span className="truncate">{page.title}</span>
                      </Link>
                    </li>
                  );
                })}
              </ul>
            </div>
          );
        })}
      </nav>

      <div className="px-4 pb-4 border-t border-goni-border text-xs text-goni-text-muted">
        <a
          href="https://github.com/duracell04/goni"
          target="_blank"
          rel="noreferrer"
          className="inline-flex items-center gap-2 hover:text-goni-text"
        >
          <span>View repo on GitHub</span>
          <span aria-hidden>?</span>
        </a>
      </div>
    </aside>
  );
}
