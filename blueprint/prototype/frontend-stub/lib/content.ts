// frontend/lib/content.ts
// Generic content loader for Goni – scans ../README.md, ../docs, ../hardware, ../software.
// Only use from server-side code (Next.js server components / route handlers).

import fs from "fs";
import path from "path";

// If you use gray-matter for frontmatter later, you can add it;
// for now we only use plain markdown + first heading.
import matter from "gray-matter";

export type SectionId = "overview" | "docs" | "hardware" | "software";

export interface PageMeta {
  section: SectionId;
  title: string;
  slug: string[];    // ['50-bom-experiments', 'bom-v1-apu-node']
  relPath: string;   // 'hardware/50-bom-experiments/bom-v1-apu-node.md'
  absPath: string;   // '/Users/you/projects/goni/hardware/...'
  githubUrl: string; // 'https://github.com/duracell04/goni/blob/main/hardware/...'
}

export interface Page extends PageMeta {
  markdown: string;
}

// Next.js runs in /frontend-stub as cwd; the blueprint root is two levels up.
const REPO_ROOT = path.resolve(process.cwd(), "..", "..");

// Configuration for each section
const SECTION_CONFIG: Record<SectionId, { baseDir: string | null; baseRoute: string }> = {
  overview: {
    baseDir: null,   // single README.md
    baseRoute: "/",
  },
  docs: {
    baseDir: "docs",
    baseRoute: "/docs",
  },
  hardware: {
    baseDir: "hardware",
    baseRoute: "/hardware",
  },
  software: {
    baseDir: "software",
    baseRoute: "/software",
  },
};

function toGithubUrl(relPath: string): string {
  // Ensure posix-style slashes
  const norm = relPath.replace(/\\/g, "/");
  return `https://github.com/duracell04/goni/blob/main/blueprint/${norm}`;
}

function extractTitleFromContent(content: string, fallback: string): string {
  // Take the first H1 or H2 as title if present
  const h1 = content.match(/^#\s+(.+)$/m);
  if (h1 && h1[1]) return h1[1].trim();
  const h2 = content.match(/^##\s+(.+)$/m);
  if (h2 && h2[1]) return h2[1].trim();
  return fallback;
}

/**
 * Scan a directory (e.g. docs/, hardware/) for .md files and return PageMeta[].
 */
function scanSectionDir(section: SectionId, dirName: string): PageMeta[] {
  const absDir = path.join(REPO_ROOT, dirName);
  if (!fs.existsSync(absDir)) {
    return [];
  }

  const results: PageMeta[] = [];

  function walk(currentRel: string) {
    const currentAbs = path.join(REPO_ROOT, currentRel);
    const entries = fs.readdirSync(currentAbs, { withFileTypes: true });

    for (const entry of entries) {
      const entryRel = path.join(currentRel, entry.name);
      const entryAbs = path.join(REPO_ROOT, entryRel);

      if (entry.isDirectory()) {
        walk(entryRel);
      } else if (entry.isFile() && entry.name.toLowerCase().endsWith(".md")) {
        const raw = fs.readFileSync(entryAbs, "utf8");
        const parsed = matter(raw);
        const content = parsed.content;
        const fileNameWithoutExt = entry.name.replace(/\.md$/i, "");
        const relFromSection = path.relative(dirName, entryRel); // e.g. '50-bom-experiments/bom-v1-apu-node.md'
        const slug = relFromSection.replace(/\.md$/i, "").split(path.sep);

        const title = extractTitleFromContent(content, fileNameWithoutExt);

        results.push({
          section,
          title,
          slug,
          relPath: entryRel.replace(/\\/g, "/"),
          absPath: entryAbs,
          githubUrl: toGithubUrl(entryRel),
        });
      }
    }
  }

  walk(dirName);
  return results;
}

/**
 * Get the "overview" page meta from README.md, if it exists.
 */
function getOverviewMeta(): PageMeta[] {
  const readmePath = path.join(REPO_ROOT, "README.md");
  if (!fs.existsSync(readmePath)) return [];

  const raw = fs.readFileSync(readmePath, "utf8");
  const parsed = matter(raw);
  const content = parsed.content;
  const title = extractTitleFromContent(content, "Goni");

  return [
    {
      section: "overview",
      title,
      slug: [], // root
      relPath: "README.md",
      absPath: readmePath,
      githubUrl: toGithubUrl("README.md"),
    },
  ];
}

// Cache pages so we don't rescan directories on every call during one build.
let _cachedPages: PageMeta[] | null = null;

/**
 * Return all pages (meta only) discovered in README.md, docs/, hardware/, software/.
 */
export function getAllPages(): PageMeta[] {
  if (_cachedPages) return _cachedPages;

  const overview = getOverviewMeta();
  const docs = scanSectionDir("docs", "docs");
  const hardware = scanSectionDir("hardware", "hardware");
  const software = scanSectionDir("software", "software");

  _cachedPages = [...overview, ...docs, ...hardware, ...software];
  return _cachedPages;
}

/**
 * Group pages by section for navigation.
 */
export function getSections(): Record<SectionId, PageMeta[]> {
  const all = getAllPages();
  const grouped: Record<SectionId, PageMeta[]> = {
    overview: [],
    docs: [],
    hardware: [],
    software: [],
  };
  for (const page of all) {
    grouped[page.section].push(page);
  }
  return grouped;
}

/**
 * Find a page by section + slug segments. For overview (home), slug is [].
 */
export function getPageBySlug(section: SectionId, slug: string[]): Page | null {
  const all = getAllPages().filter((p) => p.section === section);
  const target = all.find((p) => p.slug.join("/") === slug.join("/"));
  if (!target) return null;

  const raw = fs.readFileSync(target.absPath, "utf8");
  const parsed = matter(raw);
  const markdown = parsed.content;

  return {
    ...target,
    markdown,
  };
}

// Build the route href for a page so routing stays in one place.
export function getPageHref(meta: PageMeta): string {
  switch (meta.section) {
    case "overview":
      return "/";
    case "docs":
      return "/docs/" + meta.slug.join("/");
    case "hardware":
      return "/hardware/" + meta.slug.join("/");
    case "software":
      return "/software/" + meta.slug.join("/");
    default:
      return "/";
  }
}

/**
 * Extract a simple table-of-contents (ToC) from markdown content.
 * Only H2+ headings (##, ###, ...) are considered.
 */
export interface TocItem {
  depth: number;   // 2 for ##, 3 for ###, etc.
  text: string;
  id: string;      // slugified id (used for hash links)
}

export function getTocFromMarkdown(markdown: string): TocItem[] {
  const lines = markdown.split("\n");
  const toc: TocItem[] = [];

  for (const line of lines) {
    const match = /^(#{2,6})\s+(.+)$/.exec(line);
    if (!match) continue;
    const hashes = match[1];
    const text = match[2].trim();
    const depth = hashes.length;

    const id = text
      .toLowerCase()
      .replace(/[^\w\s-]/g, "")
      .trim()
      .replace(/\s+/g, "-");

    toc.push({ depth, text, id });
  }

  return toc;
}
