import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRUTH_MAP = ROOT / "docs" / "meta" / "truth-map.json"
TEMPLATE_ROOT = ROOT / "docs" / "meta" / "agents.root.template.md"
TEMPLATE_SOFTWARE = ROOT / "docs" / "meta" / "agents.software.template.md"
TEMPLATE_KERNEL = ROOT / "docs" / "meta" / "agents.kernel.template.md"
TEMPLATE_HARDWARE = ROOT / "docs" / "meta" / "agents.hardware.template.md"

OUTPUTS = {
    "root": ROOT / "AGENTS.md",
    "software": ROOT / "software" / "AGENTS.md",
    "kernel": ROOT / "software" / "kernel" / "AGENTS.md",
    "hardware": ROOT / "hardware" / "AGENTS.md",
}


def load_truth_map():
    with TRUTH_MAP.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def render_truth_map_block(data):
    groups = {group["id"]: group for group in data.get("groups", [])}
    entries = data.get("entries", [])

    grouped = {group_id: [] for group_id in groups}
    for entry in entries:
        group_id = entry.get("group")
        if group_id not in grouped:
            grouped[group_id] = []
        grouped[group_id].append(entry)

    lines = []
    for group_id, group in groups.items():
        lines.append(f"### {group['title']}")
        for entry in sorted(grouped.get(group_id, []), key=lambda item: item["id"]):
            lines.append(f"- {entry['id']} - {entry['title']}: `{entry['path']}`")
        lines.append("")

    return "\n".join(lines).rstrip() + "\n"


def render_template(template_path, truth_block=None):
    content = template_path.read_text(encoding="utf-8")
    if truth_block is not None:
        content = content.replace("{{TRUTH_MAP}}", truth_block)
    return content


def write_file(path, content):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def main():
    data = load_truth_map()
    truth_block = render_truth_map_block(data)

    write_file(OUTPUTS["root"], render_template(TEMPLATE_ROOT, truth_block))
    write_file(OUTPUTS["software"], render_template(TEMPLATE_SOFTWARE))
    write_file(OUTPUTS["kernel"], render_template(TEMPLATE_KERNEL))
    write_file(OUTPUTS["hardware"], render_template(TEMPLATE_HARDWARE))


if __name__ == "__main__":
    main()
