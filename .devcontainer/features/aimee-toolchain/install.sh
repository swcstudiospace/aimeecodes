#!/usr/bin/env bash
# Local Dev Container Feature: aimee-toolchain.
# Runs as root during image build. Does not print secrets.
set -euo pipefail

PROTOC_VERSION="${PROTOCVERSION:-28.3}"

if [[ ! -f /etc/os-release ]] || ! grep -Eiq 'debian|ubuntu' /etc/os-release; then
  echo "aimee-toolchain requires a Debian or Ubuntu base image" >&2
  exit 1
fi

export DEBIAN_FRONTEND=noninteractive

apt-get update
apt-get install -y --no-install-recommends \
  ca-certificates \
  curl \
  fd-find \
  fzf \
  jq \
  ripgrep \
  unzip
rm -rf /var/lib/apt/lists/*

if command -v fdfind >/dev/null 2>&1 && [[ ! -e /usr/local/bin/fd ]]; then
  ln -s "$(command -v fdfind)" /usr/local/bin/fd
fi

arch="$(uname -m)"
case "${arch}" in
  x86_64)
    protoc_zip="protoc-${PROTOC_VERSION}-linux-x86_64.zip"
    protoc_sha="0ad949f04a6a174da83cdcbdb36dee0a4925272a5b6d83f79a6bf9852076d53f"
    ;;
  aarch64 | arm64)
    protoc_zip="protoc-${PROTOC_VERSION}-linux-aarch_64.zip"
    protoc_sha="1de522032a8b194002fe35cab86d747848238b5e4de4f99648372079f5b46f9a"
    ;;
  *)
    echo "unsupported architecture for pinned protoc: ${arch}" >&2
    exit 1
    ;;
esac

work="$(mktemp -d)"
trap 'rm -rf "${work}"' EXIT
url="https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/${protoc_zip}"
curl -fsSL -o "${work}/${protoc_zip}" "${url}"
echo "${protoc_sha}  ${work}/${protoc_zip}" | sha256sum -c -
unzip -o "${work}/${protoc_zip}" -d /usr/local bin/protoc
unzip -o "${work}/${protoc_zip}" -d /usr/local 'include/*'
chmod 0755 /usr/local/bin/protoc

/usr/local/bin/protoc --version
