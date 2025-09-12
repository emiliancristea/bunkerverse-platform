#!/bin/bash
# BunkerVerse Control Center macOS DMG Installer Creator
# Creates a distributable DMG file with the application bundle

set -e

# Configuration
APP_NAME="BunkerVerse Control Center"
BUNDLE_NAME="BunkerVerse Control Center.app"
VERSION="${1:-1.0.0}"
BUILD_DIR="${2:-../../build/macos/release}"
QT_DIR="${3:-$HOME/Qt/6.9.2/clang_64}"
OUTPUT_DIR="${4:-./output}"
IDENTITY="${5:-}" # Developer ID Application certificate

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}BunkerVerse Control Center macOS Installer Builder${NC}"
echo -e "${CYAN}===================================================${NC}"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"
mkdir -p "build"

echo -e "${GREEN}Building installer for version: $VERSION${NC}"
echo -e "${GREEN}Build directory: $BUILD_DIR${NC}"
echo -e "${GREEN}Qt directory: $QT_DIR${NC}"
echo ""

# Step 1: Create app bundle structure
echo -e "${YELLOW}Step 1: Creating app bundle structure...${NC}"
APP_BUNDLE="build/$BUNDLE_NAME"
rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/"{MacOS,Resources,Frameworks,PlugIns}

# Step 2: Copy Info.plist
echo -e "${YELLOW}Step 2: Creating Info.plist...${NC}"
cat > "$APP_BUNDLE/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>bunkerverse-control-center</string>
    <key>CFBundleIconFile</key>
    <string>BunkerVerse.icns</string>
    <key>CFBundleIdentifier</key>
    <string>io.bunkerverse.controlcenter</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>BunkerVerse Control Center</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2024 BunkerVerse Corporation. All rights reserved.</string>
    <key>NSMainStoryboardFile</key>
    <string>Main</string>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.games</string>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeExtensions</key>
            <array>
                <string>bvproj</string>
            </array>
            <key>CFBundleTypeIconFile</key>
            <string>DocumentIcon.icns</string>
            <key>CFBundleTypeName</key>
            <string>BunkerVerse Project</string>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>io.bunkerverse.project</string>
            </array>
        </dict>
    </array>
</dict>
</plist>
EOF

# Step 3: Copy executable and libraries
echo -e "${YELLOW}Step 3: Copying executable and libraries...${NC}"

# Copy main executable (placeholder for now)
if [ -f "$BUILD_DIR/bunkerverse-control-center" ]; then
    cp "$BUILD_DIR/bunkerverse-control-center" "$APP_BUNDLE/Contents/MacOS/"
else
    echo -e "${YELLOW}Warning: Main executable not found, creating placeholder${NC}"
    echo '#!/bin/bash' > "$APP_BUNDLE/Contents/MacOS/bunkerverse-control-center"
    echo 'echo "BunkerVerse Control Center"' >> "$APP_BUNDLE/Contents/MacOS/bunkerverse-control-center"
    chmod +x "$APP_BUNDLE/Contents/MacOS/bunkerverse-control-center"
fi

# Copy Rust libraries
if [ -f "$BUILD_DIR/libclient_rust_app_logic.dylib" ]; then
    cp "$BUILD_DIR/libclient_rust_app_logic.dylib" "$APP_BUNDLE/Contents/Frameworks/"
fi

if [ -f "$BUILD_DIR/libnar_rust_wrapper_for_llama_cpp.dylib" ]; then
    cp "$BUILD_DIR/libnar_rust_wrapper_for_llama_cpp.dylib" "$APP_BUNDLE/Contents/Frameworks/"
fi

# Step 4: Copy Qt frameworks and plugins
echo -e "${YELLOW}Step 4: Deploying Qt frameworks...${NC}"
if [ -d "$QT_DIR" ]; then
    # Use macdeployqt if available
    if command -v "$QT_DIR/bin/macdeployqt" &> /dev/null; then
        "$QT_DIR/bin/macdeployqt" "$APP_BUNDLE" \
            -qmldir="$BUILD_DIR/qml" \
            -verbose=2 \
            -always-overwrite
    else
        echo -e "${YELLOW}macdeployqt not found, copying Qt libraries manually${NC}"
        
        # Copy Qt frameworks manually
        for framework in Core Gui Widgets Qml Quick Network; do
            if [ -d "$QT_DIR/lib/Qt${framework}.framework" ]; then
                cp -R "$QT_DIR/lib/Qt${framework}.framework" "$APP_BUNDLE/Contents/Frameworks/"
            fi
        done
        
        # Copy Qt plugins
        mkdir -p "$APP_BUNDLE/Contents/PlugIns/platforms"
        if [ -f "$QT_DIR/plugins/platforms/libqcocoa.dylib" ]; then
            cp "$QT_DIR/plugins/platforms/libqcocoa.dylib" "$APP_BUNDLE/Contents/PlugIns/platforms/"
        fi
        
        # Copy Qt QML modules
        if [ -d "$BUILD_DIR/qml" ]; then
            cp -R "$BUILD_DIR/qml" "$APP_BUNDLE/Contents/Resources/"
        fi
    fi
fi

# Step 5: Copy resources
echo -e "${YELLOW}Step 5: Copying resources...${NC}"

# Create and copy icon
if [ ! -f "assets/BunkerVerse.icns" ]; then
    echo -e "${YELLOW}Creating placeholder icon...${NC}"
    mkdir -p assets
    # In production, use iconutil to create proper .icns from PNG files
    touch "assets/BunkerVerse.icns"
fi
cp "assets/BunkerVerse.icns" "$APP_BUNDLE/Contents/Resources/" 2>/dev/null || true

# Copy configuration files
if [ -f "$BUILD_DIR/config.toml" ]; then
    cp "$BUILD_DIR/config.toml" "$APP_BUNDLE/Contents/Resources/"
fi

# Step 6: Fix library paths
echo -e "${YELLOW}Step 6: Fixing library paths...${NC}"
if command -v install_name_tool &> /dev/null; then
    # Fix paths for bundled libraries
    for lib in "$APP_BUNDLE/Contents/Frameworks/"*.dylib; do
        if [ -f "$lib" ]; then
            lib_name=$(basename "$lib")
            install_name_tool -id "@executable_path/../Frameworks/$lib_name" "$lib"
            
            # Update references in the main executable
            if [ -f "$APP_BUNDLE/Contents/MacOS/bunkerverse-control-center" ]; then
                install_name_tool -change "$lib_name" "@executable_path/../Frameworks/$lib_name" \
                    "$APP_BUNDLE/Contents/MacOS/bunkerverse-control-center" 2>/dev/null || true
            fi
        fi
    done
fi

# Step 7: Code signing
if [ -n "$IDENTITY" ]; then
    echo -e "${YELLOW}Step 7: Signing app bundle...${NC}"
    
    # Sign frameworks first
    find "$APP_BUNDLE/Contents/Frameworks" -name "*.framework" -o -name "*.dylib" | while read framework; do
        codesign --force --deep --sign "$IDENTITY" "$framework"
    done
    
    # Sign plugins
    find "$APP_BUNDLE/Contents/PlugIns" -name "*.dylib" | while read plugin; do
        codesign --force --sign "$IDENTITY" "$plugin"
    done
    
    # Sign the main app bundle
    codesign --force --deep --sign "$IDENTITY" \
        --entitlements entitlements.plist \
        --options runtime \
        "$APP_BUNDLE"
    
    echo -e "${GREEN}✓ App bundle signed successfully${NC}"
else
    echo -e "${YELLOW}Step 7: Skipping code signing (no identity provided)${NC}"
fi

# Step 8: Create DMG
echo -e "${YELLOW}Step 8: Creating DMG installer...${NC}"

DMG_NAME="BunkerVerseControlCenter-$VERSION"
DMG_TEMP="$OUTPUT_DIR/${DMG_NAME}-temp.dmg"
DMG_FINAL="$OUTPUT_DIR/${DMG_NAME}.dmg"
VOLUME_NAME="BunkerVerse Control Center"

# Remove old DMG if exists
rm -f "$DMG_TEMP" "$DMG_FINAL"

# Create DMG
if command -v create-dmg &> /dev/null; then
    # Use create-dmg if available (install with: npm install -g create-dmg)
    create-dmg \
        --volname "$VOLUME_NAME" \
        --volicon "assets/BunkerVerse.icns" \
        --background "assets/dmg-background.png" \
        --window-pos 200 120 \
        --window-size 800 400 \
        --icon-size 100 \
        --icon "$BUNDLE_NAME" 200 190 \
        --hide-extension "$BUNDLE_NAME" \
        --app-drop-link 600 190 \
        --no-internet-enable \
        "$DMG_FINAL" \
        "build/"
else
    # Fallback to manual DMG creation
    echo -e "${YELLOW}create-dmg not found, using hdiutil...${NC}"
    
    # Calculate size needed (app size + 50MB buffer)
    APP_SIZE=$(du -sk "$APP_BUNDLE" | cut -f1)
    DMG_SIZE=$((APP_SIZE + 51200))
    
    # Create temporary DMG
    hdiutil create -size "${DMG_SIZE}k" -fs HFS+ -volname "$VOLUME_NAME" "$DMG_TEMP"
    
    # Mount temporary DMG
    MOUNT_POINT=$(hdiutil attach -readwrite -noverify -noautoopen "$DMG_TEMP" | \
                  grep "/Volumes" | sed 's/.*\/Volumes/\/Volumes/')
    
    # Copy app bundle to DMG
    cp -R "$APP_BUNDLE" "$MOUNT_POINT/"
    
    # Create Applications symlink
    ln -s /Applications "$MOUNT_POINT/Applications"
    
    # Create .DS_Store for window settings (optional)
    # This would require additional tooling to set proper window appearance
    
    # Unmount
    hdiutil detach "$MOUNT_POINT"
    
    # Convert to compressed DMG
    hdiutil convert "$DMG_TEMP" -format UDZO -o "$DMG_FINAL"
    rm -f "$DMG_TEMP"
fi

echo -e "${GREEN}✓ DMG created successfully${NC}"

# Step 9: Notarization (requires Apple Developer account)
if [ -n "$IDENTITY" ] && [ -n "$APPLE_ID" ] && [ -n "$APPLE_PASSWORD" ]; then
    echo -e "${YELLOW}Step 9: Notarizing DMG...${NC}"
    
    # Submit for notarization
    xcrun altool --notarize-app \
        --primary-bundle-id "io.bunkerverse.controlcenter" \
        --username "$APPLE_ID" \
        --password "$APPLE_PASSWORD" \
        --file "$DMG_FINAL"
    
    # Note: In production, you'd wait for notarization to complete and then staple
    # xcrun stapler staple "$DMG_FINAL"
    
    echo -e "${GREEN}✓ Notarization submitted${NC}"
else
    echo -e "${YELLOW}Step 9: Skipping notarization (credentials not provided)${NC}"
fi

# Clean up
echo -e "${YELLOW}Cleaning up temporary files...${NC}"
# Keep the app bundle for debugging if needed
# rm -rf "$APP_BUNDLE"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build completed successfully!${NC}"
echo -e "${GREEN}Output file: $DMG_FINAL${NC}"
echo -e "${GREEN}========================================${NC}"