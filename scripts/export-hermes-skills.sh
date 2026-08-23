#!/usr/bin/env bash
# Export saved Hermes Agent skills into the Aimee project-local skills tree.
#
# Usage: scripts/export-hermes-skills.sh [SOURCE_DIR] [DEST_DIR]
#   SOURCE_DIR defaults to ~/.hermes/skills (category/name/SKILL.md nesting).
#   DEST_DIR   defaults to ./.aimee/skills (flat name/ per Aimee's loader,
#              depth 1 — see aimee_repo::skill::load_skills_from_dir).
#
# Rules:
# - Only directories containing a SKILL.md are exported.
# - Name collisions across categories keep the FIRST occurrence (sorted order)
#   and a warning is printed; Aimee resolves conflicts last-wins, so we
#   deduplicate here to make the export deterministic.
# - Re-running is idempotent: each export fully refreshes the destination.
set -euo pipefail

SOURCE="${1:-${HOME}/.hermes/skills}"
DEST="${2:-./.aimee/skills}"

if [[ ! -d "${SOURCE}" ]]; then
  echo "error: source skills directory not found: ${SOURCE}" >&2
  exit 1
fi

exported=0
skipped=0
seen_names=""

while IFS= read -r skill_file; do
  skill_dir="$(dirname "${skill_file}")"
  name="$(basename "${skill_dir}")"

  if printf '%s\n' "${seen_names}" | grep -qx -- "${name}"; then
    echo "warn: duplicate skill name '${name}' at ${skill_dir} — keeping first occurrence" >&2
    skipped=$((skipped + 1))
    continue
  fi
  seen_names="${seen_names}${name}"$'\n'

  mkdir -p "${DEST}/${name}"
  rm -rf "${DEST:?}/${name:?}"
  cp -R "${skill_dir}" "${DEST}/${name}"
  exported=$((exported + 1))
done < <(find "${SOURCE}" -mindepth 2 -maxdepth 4 -name SKILL.md -type f | sort)

echo "Exported ${exported} skill(s) to ${DEST} (${skipped} duplicate(s) skipped)."
