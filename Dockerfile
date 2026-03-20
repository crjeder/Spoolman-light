FROM python:3.12-slim-bookworm AS python-builder

ARG UV_VERSION=0.6.6

# Install dependencies
RUN apt-get update && apt-get install -y \
    g++ \
    python3-dev \
    libpq-dev \
    libffi-dev \
    curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install uv
RUN curl -LsSf https://astral.sh/uv/${UV_VERSION}/install.sh | sh

# Add local user so we don't run as root
RUN groupmod -g 1000 users \
    && useradd -u 911 -U app \
    && usermod -G users app

ENV PATH="/root/.local/bin:/home/app/.local/bin:${PATH}"

# Copy and install dependencies
COPY --chown=app:app pyproject.toml /home/app/spoolman/
COPY --chown=app:app uv.lock /home/app/spoolman/
WORKDIR /home/app/spoolman
RUN uv sync --frozen --no-dev

# Copy and install app
COPY --chown=app:app spoolman /home/app/spoolman/spoolman
COPY --chown=app:app README.md /home/app/spoolman/

FROM python:3.12-slim-bookworm AS python-runner

LABEL org.opencontainers.image.source=https://github.com/Donkie/Spoolman
LABEL org.opencontainers.image.description="Keep track of your inventory of 3D-printer filament spools."
LABEL org.opencontainers.image.licenses=MIT

# Install gosu for privilege dropping
RUN apt-get update && apt-get install -y \
    gosu \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Add local user so we don't run as root
RUN groupmod -g 1000 users \
    && useradd -u 1000 -U app \
    && usermod -G users app \
    && mkdir -p /home/app/.local/share/spoolman \
    && chown -R app:app /home/app/.local/share/spoolman

# Copy built client
COPY --chown=app:app ./client/dist /home/app/spoolman/client/dist

# Copy built app
COPY --chown=app:app --from=python-builder /home/app/spoolman /home/app/spoolman

COPY entrypoint.sh /home/app/spoolman/entrypoint.sh
RUN chmod +x /home/app/spoolman/entrypoint.sh

WORKDIR /home/app/spoolman

ENV PATH="/home/app/spoolman/.venv/bin:${PATH}"

ARG GIT_COMMIT=unknown
ARG BUILD_DATE=unknown
ENV GIT_COMMIT=${GIT_COMMIT}
ENV BUILD_DATE=${BUILD_DATE}

# Write GIT_COMMIT and BUILD_DATE to a build.txt file
RUN echo "GIT_COMMIT=${GIT_COMMIT}" > build.txt \
    && echo "BUILD_DATE=${BUILD_DATE}" >> build.txt

# Run command
EXPOSE 8000
ENTRYPOINT ["/home/app/spoolman/entrypoint.sh"]
