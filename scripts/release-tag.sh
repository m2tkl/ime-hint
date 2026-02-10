#!/usr/bin/env bash
set -euo pipefail

latest_tag="$(git tag --sort=version:refname | tail -n 1 || true)"

if [[ -n "${latest_tag}" ]]; then
  echo "Latest tag: ${latest_tag}"
else
  echo "Latest tag: (none)"
fi

while true; do
  read -r -p "Next version (without v): " next_version
  if [[ "${next_version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+([\-+][0-9A-Za-z.-]+)?$ ]]; then
    break
  fi
  echo "Invalid SemVer. Example: 1.2.3, 1.2.3-rc.1, 1.2.3+build.5"
done

new_tag="v${next_version}"

if git rev-parse -q --verify "refs/tags/${new_tag}" >/dev/null; then
  echo "Tag already exists: ${new_tag}"
  exit 1
fi

read -r -p "Create tag ${new_tag}? [y/N]: " confirm
case "${confirm}" in
  [yY][eE][sS]|[yY])
    git tag "${new_tag}"
    echo "Created tag: ${new_tag}"
    ;;
  *)
    echo "Canceled."
    ;;
esac
