# Fully isolated sandbox for Claude Code development
# Clones project from git - no host filesystem access

FROM rust:latest

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
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20 (required for Claude Code)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Bun (JavaScript runtime for testing)
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

# Install Claude Code CLI globally
RUN npm install -g @anthropic-ai/claude-code

# Set up Rust components
RUN rustup component add rustfmt clippy

# Clone the project from git
WORKDIR /app
RUN git clone https://github.com/trevor-ofarrell/leet-tui.git .

# Build dependencies (cached layer)
RUN cargo fetch

# Set environment for interactive terminal
ENV TERM=xterm-256color
ENV SHELL=/bin/bash

# Default command
CMD ["/bin/bash"]
