#!/bin/bash
set -e

echo "=== Cathedral ARKHE v28.3 - Inicializando a Stack ==="

# Verifica se o docker-compose existe na pasta runtime
if [ ! -f "runtime/docker-compose.yml" ]; then
    echo "Erro: runtime/docker-compose.yml não encontrado!"
    exit 1
fi

# Cria as pastas de volume caso não existam
echo "-> Preparando diretórios de volumes..."
mkdir -p models agent core trust

# Garante permissões adequadas
echo "-> Ajustando permissões..."
chmod -R 755 .

# Levanta a stack usando docker-compose
echo "-> Iniciando os serviços (LLM, Agent, Redis, DBs, Jaeger)..."
cd runtime
docker-compose up -d --build

echo ""
echo "=== Stack iniciada com sucesso! ==="
echo "Para ver os logs, execute:"
echo "cd runtime && docker-compose logs -f"
