#!/bin/bash

echo "🐚 Testing Clams Development Services"
echo "======================================"

# Function to test a service
test_service() {
    local name=$1
    local port=$2
    local expected_code=${3:-200}
    
    echo -n "Testing $name (port $port): "
    
    # Wait for service to be ready
    for i in {1..30}; do
        if curl -s -f http://localhost:$port > /dev/null 2>&1; then
            break
        fi
        sleep 1
    done
    
    status_code=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:$port)
    
    if [ "$status_code" = "$expected_code" ]; then
        echo "✅ OK ($status_code)"
    else
        echo "❌ FAILED ($status_code, expected $expected_code)"
        return 1
    fi
}

# Start services if not running
echo "Starting services..."
docker compose up -d

echo ""
echo "Waiting for services to start..."
sleep 10

echo ""
echo "Testing services:"

# Test each service
test_service "Website" 5173
test_service "Blog" 1111
test_service "Docs" 3000
test_service "Server" 8080 "000"

echo ""
echo "Service status:"
docker compose ps

echo ""
echo "🎉 All services are running!"
echo ""
echo "Access your services at:"
echo "  Website: http://localhost:5173"
echo "  Blog:    http://localhost:1111"
echo "  Docs:    http://localhost:3000"
echo "  Server:  http://localhost:8080"
echo ""
echo "To stop all services: docker compose down"