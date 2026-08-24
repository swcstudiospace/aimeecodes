# syntax=docker/dockerfile:1.7
# Aimee Codes workspace image (Debian bookworm).
#
# Toolchains that pin a version (Rust 1.97, Node 24, GitHub CLI, Git) are
# installed by Dev Container Features, not this file. This layer only carries
# native build dependencies that match flake.nix + Cross.toml + CI:
# cmake, nasm, perl, pkg-config, libsqlite3-dev, protobuf's unzip/curl
# prerequisites, and the Linux TUI/X11 libs from flake.nix.
#
# Do not COPY the workspace into the image. The spec bind-mounts the repo.
# Do not run as privileged. Do not install docker-outside-of-docker here:
# `aimee pod` is the isolation boundary for untrusted PRs.

FROM mcr.microsoft.com/devcontainers/base:1-bookworm@sha256:ce2e9e611939e611b737362c045bb6d3449bb3efb84898525d724aace1737b90

ARG USERNAME=vscode
ARG DEBIAN_FRONTEND=noninteractive

LABEL org.opencontainers.image.title="Aimee Codes devcontainer" \
      org.opencontainers.image.description="Development environment for the Aimee Codes Rust 2024 workspace (MSRV 1.94, pin 1.97)." \
      org.opencontainers.image.source="https://github.com/swcstudiospace/omegaloops" \
      org.opencontainers.image.licenses="Apache-2.0" \
      org.opencontainers.image.vendor="Spectrum Web Co LLC" \
      org.opencontainers.image.documentation="https://github.com/swcstudiospace/omegaloops/tree/main/.devcontainer"

USER root

# hadolint ignore=DL3008 -- distro package versions follow the bookworm image pin
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        clang \
        cmake \
        curl \
        file \
        gnupg \
        less \
        libsqlite3-dev \
        libssl-dev \
        libwayland-dev \
        libx11-dev \
        libxcb1-dev \
        libxext-dev \
        libxfixes-dev \
        libxkbcommon-dev \
        lldb \
        locales \
        nasm \
        perl \
        pkg-config \
        python3 \
        python3-venv \
        sqlite3 \
        unzip \
        xz-utils \
        zip \
    && rm -rf /var/lib/apt/lists/* \
    && localedef -i en_US -c -f UTF-8 -A /usr/share/locale/locale.alias en_US.UTF-8 || true \
    && mkdir -p /commandhistory \
    && chown "${USERNAME}:${USERNAME}" /commandhistory

ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    TZ=UTC \
    APP_VERSION=0.1.0-dev

USER ${USERNAME}
WORKDIR /workspaces/aimeecodes
