#!/bin/bash
# Deploy generated documentation to GitHub Pages or documentation hosting
# Supports versioned documentation with automatic archiving

set -euo pipefail

VERSION=${1:-v0.3.0}
DEPLOY_TARGET=${2:-github-pages}
BASE_URL=${3:-https://bunkerverse.github.io/platform-docs}

echo "🚀 Deploying documentation version $VERSION to $DEPLOY_TARGET..."

# Generate fresh documentation
./scripts/generate-docs.sh "$VERSION"

case "$DEPLOY_TARGET" in
    "github-pages")
        echo "📤 Deploying to GitHub Pages..."
        
        # Create deployment branch if it doesn't exist
        git checkout -B gh-pages || git checkout gh-pages
        
        # Clear existing content except .git
        find . -maxdepth 1 ! -name '.git' ! -name '.' -exec rm -rf {} \;
        
        # Copy generated documentation
        cp -r docs/generated/* .
        
        # Create version archive structure
        mkdir -p versions/$VERSION
        cp -r docs/generated/* versions/$VERSION/
        
        # Update version index
        cat > versions/index.html << EOF
<!DOCTYPE html>
<html>
<head>
    <title>BUNKERVERSE API Documentation - Version Archive</title>
    <link rel="stylesheet" href="static/custom.css">
</head>
<body>
    <div class="header">
        <h1>BUNKERVERSE API Documentation</h1>
        <p>Version Archive</p>
    </div>
    <div style="padding: 40px;">
        <h2>Available Versions</h2>
        <ul>
            <li><a href="../index.html">Latest ($VERSION)</a></li>
            <li><a href="$VERSION/index.html">$VERSION</a></li>
        </ul>
    </div>
</body>
</html>
EOF
        
        # Commit and push
        git add .
        git commit -m "Deploy documentation version $VERSION"
        git push origin gh-pages
        
        echo "✅ Documentation deployed to: $BASE_URL"
        ;;
        
    "local")
        echo "🌐 Starting local documentation server..."
        cd docs/generated
        python3 -m http.server 8000 || python -m http.server 8000
        ;;
        
    "docker")
        echo "🐳 Building Docker image for documentation..."
        cat > Dockerfile.docs << EOF
FROM nginx:alpine
COPY docs/generated /usr/share/nginx/html
COPY scripts/nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
EOF
        
        docker build -f Dockerfile.docs -t bunkerverse-docs:$VERSION .
        echo "✅ Docker image built: bunkerverse-docs:$VERSION"
        echo "Run with: docker run -p 8080:80 bunkerverse-docs:$VERSION"
        ;;
        
    *)
        echo "❌ Unknown deployment target: $DEPLOY_TARGET"
        echo "Available targets: github-pages, local, docker"
        exit 1
        ;;
esac