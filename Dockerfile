# ORION Kernel — Industrialisierbar
# Python 3.10+, minimal, reproduzierbar

FROM python:3.11-slim

WORKDIR /app

# Keine Cache für reproduzierbare Builds
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

# Abhängigkeiten (inkl. PostgreSQL für Industrialisierung)
COPY pyproject.toml ./
RUN pip install --no-cache-dir -e ".[industrial]"

# App
COPY src/ src/
COPY app/ app/
COPY config/ config/
COPY CausalHome/ CausalHome/
COPY or1on/ or1on/

# Datenverzeichnis (Volume-Mount in Produktion)
RUN mkdir -p /app/data

ENV ORION_DATA_DIR=/app/data
ENV ORION_API_HOST=0.0.0.0
ENV ORION_API_PORT=8765

EXPOSE 8765

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8765/health')" || exit 1

CMD ["python", "-m", "uvicorn", "agents.api:app", "--host", "0.0.0.0", "--port", "8765"]
