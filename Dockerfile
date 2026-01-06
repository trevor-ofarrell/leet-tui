# Fully isolated sandbox for Claude Code development
# Clones project from git - no host filesystem access

FROM debian:bookworm-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    gcc \
    g++ \
    python3 \
    python3-pip \
    python3-venv \
    neovim \
    git \
    tmux \
    curl \
    wget \
    unzip \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20 (required for Claude Code)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Claude Code CLI globally
RUN npm install -g @anthropic-ai/claude-code

# Create non-root user for Claude Code
RUN useradd -m -s /bin/bash claude \
    && mkdir -p /app \
    && chown -R claude:claude /app

# Switch to non-root user
USER claude
WORKDIR /home/claude

# Install Rust for non-root user
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . ~/.cargo/env \
    && rustup component add rustfmt clippy
ENV PATH="/home/claude/.cargo/bin:${PATH}"

# Install Bun for non-root user
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/home/claude/.bun/bin:${PATH}"

# Set up git config for commits
RUN git config --global user.email "claude@container" \
    && git config --global user.name "Claude"

# Clone the project from git
WORKDIR /app
RUN git clone https://github.com/trevor-ofarrell/leet-tui.git .

# Set environment for interactive terminal
ENV TERM=xterm-256color
ENV SHELL=/bin/bash

# Default command
CMD ["/bin/bash"]
