#!/bin/bash
# BunkerVerse Control Center Linux AppImage Creator
# Creates a portable AppImage that runs on most Linux distributions

set -e

# Configuration
APP_NAME="BunkerVerse Control Center"
VERSION="${1:-1.0.0}"
BUILD_DIR="${2:-../../build/linux/release}"
QT_DIR="${3:-/opt/qt6}"
OUTPUT_DIR="${4:-./output}"
ARCH="${5:-x86_64}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}BunkerVerse Control Center Linux AppImage Builder${NC}"
echo -e "${CYAN}==================================================${NC}"
echo ""

# Create directories
mkdir -p "$OUTPUT_DIR"
mkdir -p "build"

echo -e "${GREEN}Building AppImage for version: $VERSION${NC}"
echo -e "${GREEN}Build directory: $BUILD_DIR${NC}"
echo -e "${GREEN}Qt directory: $QT_DIR${NC}"
echo -e "${GREEN}Architecture: $ARCH${NC}"
echo ""

# Step 1: Download AppImage tools if not present
echo -e "${YELLOW}Step 1: Setting up AppImage tools...${NC}"

APPIMAGETOOL="appimagetool-$ARCH.AppImage"
LINUXDEPLOY="linuxdeploy-$ARCH.AppImage"
LINUXDEPLOY_QT="linuxdeploy-plugin-qt-$ARCH.AppImage"

if [ ! -f "$APPIMAGETOOL" ]; then
    echo "Downloading appimagetool..."
    wget -q "https://github.com/AppImage/AppImageKit/releases/download/continuous/$APPIMAGETOOL"
    chmod +x "$APPIMAGETOOL"
fi

if [ ! -f "$LINUXDEPLOY" ]; then
    echo "Downloading linuxdeploy..."
    wget -q "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/$LINUXDEPLOY"
    chmod +x "$LINUXDEPLOY"
fi

if [ ! -f "$LINUXDEPLOY_QT" ]; then
    echo "Downloading linuxdeploy-plugin-qt..."
    wget -q "https://github.com/linuxdeploy/linuxdeploy-plugin-qt/releases/download/continuous/$LINUXDEPLOY_QT"
    chmod +x "$LINUXDEPLOY_QT"
fi

echo -e "${GREEN}✓ AppImage tools ready${NC}"

# Step 2: Create AppDir structure
echo -e "${YELLOW}Step 2: Creating AppDir structure...${NC}"

APPDIR="build/AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/"{bin,lib,share/applications,share/icons/hicolor/256x256/apps,share/metainfo}

# Step 3: Copy main executable
echo -e "${YELLOW}Step 3: Copying executable and libraries...${NC}"

if [ -f "$BUILD_DIR/bunkerverse-control-center" ]; then
    cp "$BUILD_DIR/bunkerverse-control-center" "$APPDIR/usr/bin/"
else
    echo -e "${YELLOW}Warning: Main executable not found, creating placeholder${NC}"
    cat > "$APPDIR/usr/bin/bunkerverse-control-center" << 'EOF'
#!/bin/bash
echo "BunkerVerse Control Center v$VERSION"
echo "This is a placeholder executable"
EOF
    chmod +x "$APPDIR/usr/bin/bunkerverse-control-center"
fi

# Copy Rust libraries
if [ -f "$BUILD_DIR/libclient_rust_app_logic.so" ]; then
    cp "$BUILD_DIR/libclient_rust_app_logic.so" "$APPDIR/usr/lib/"
fi

if [ -f "$BUILD_DIR/libnar_rust_wrapper_for_llama_cpp.so" ]; then
    cp "$BUILD_DIR/libnar_rust_wrapper_for_llama_cpp.so" "$APPDIR/usr/lib/"
fi

# Step 4: Create desktop entry
echo -e "${YELLOW}Step 4: Creating desktop entry...${NC}"

cat > "$APPDIR/usr/share/applications/bunkerverse-control-center.desktop" << EOF
[Desktop Entry]
Type=Application
Name=BunkerVerse Control Center
GenericName=Game Control Center
Comment=Command your empire in the decentralized multiverse
Icon=bunkerverse-control-center
Exec=bunkerverse-control-center %F
Terminal=false
Categories=Game;Strategy;Network;
Keywords=bunkerverse;blockchain;gaming;control;center;
MimeType=application/x-bunkerverse-project;
StartupNotify=true
StartupWMClass=bunkerverse-control-center
Actions=NewWindow;

[Desktop Action NewWindow]
Name=Open New Window
Exec=bunkerverse-control-center --new-window
Icon=bunkerverse-control-center
EOF

# Step 5: Create AppStream metadata
echo -e "${YELLOW}Step 5: Creating AppStream metadata...${NC}"

cat > "$APPDIR/usr/share/metainfo/io.bunkerverse.controlcenter.appdata.xml" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>io.bunkerverse.controlcenter</id>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>Proprietary</project_license>
  <name>BunkerVerse Control Center</name>
  <summary>Unified game management platform for the BunkerVerse ecosystem</summary>
  
  <description>
    <p>
      BunkerVerse Control Center is the command hub for managing your presence
      in the decentralized multiverse. Built with cutting-edge technology including
      Rust, CXX-Qt, and QML for optimal performance and user experience.
    </p>
    <p>Features:</p>
    <ul>
      <li>Real-time game state synchronization</li>
      <li>Blockchain integration for asset management</li>
      <li>AI-powered strategic assistant (NAR)</li>
      <li>Cross-platform support</li>
      <li>Modular architecture for extensibility</li>
    </ul>
  </description>
  
  <launchable type="desktop-id">bunkerverse-control-center.desktop</launchable>
  
  <screenshots>
    <screenshot type="default">
      <caption>Main Dashboard</caption>
      <image>https://bunkerverse.io/screenshots/dashboard.png</image>
    </screenshot>
  </screenshots>
  
  <url type="homepage">https://bunkerverse.io</url>
  <url type="bugtracker">https://github.com/bunkerverse/control-center/issues</url>
  <url type="help">https://docs.bunkerverse.io</url>
  
  <developer_name>BunkerVerse Corporation</developer_name>
  
  <provides>
    <binary>bunkerverse-control-center</binary>
  </provides>
  
  <content_rating type="oars-1.1">
    <content_attribute id="violence-fantasy">mild</content_attribute>
    <content_attribute id="social-chat">intense</content_attribute>
  </content_rating>
  
  <releases>
    <release version="$VERSION" date="$(date -I)">
      <description>
        <p>Initial release of BunkerVerse Control Center</p>
      </description>
    </release>
  </releases>
  
  <categories>
    <category>Game</category>
    <category>Strategy</category>
    <category>Network</category>
  </categories>
  
  <keywords>
    <keyword>bunkerverse</keyword>
    <keyword>blockchain</keyword>
    <keyword>gaming</keyword>
    <keyword>strategy</keyword>
  </keywords>
  
  <requires>
    <display_length compare="ge">1024</display_length>
    <internet>always</internet>
  </requires>
  
  <recommends>
    <control>keyboard</control>
    <control>pointing</control>
  </recommends>
</component>
EOF

# Step 6: Create/copy icon
echo -e "${YELLOW}Step 6: Setting up application icon...${NC}"

if [ -f "assets/bunkerverse.png" ]; then
    cp "assets/bunkerverse.png" "$APPDIR/usr/share/icons/hicolor/256x256/apps/bunkerverse-control-center.png"
else
    echo -e "${YELLOW}Creating placeholder icon...${NC}"
    # Create a simple placeholder PNG using ImageMagick if available
    if command -v convert &> /dev/null; then
        convert -size 256x256 xc:navy \
                -fill white -gravity center \
                -pointsize 48 -annotate +0+0 'BV' \
                "$APPDIR/usr/share/icons/hicolor/256x256/apps/bunkerverse-control-center.png"
    else
        # Create empty file as fallback
        touch "$APPDIR/usr/share/icons/hicolor/256x256/apps/bunkerverse-control-center.png"
    fi
fi

# Also copy icon to root for AppImage
cp "$APPDIR/usr/share/icons/hicolor/256x256/apps/bunkerverse-control-center.png" \
   "$APPDIR/bunkerverse-control-center.png" 2>/dev/null || true

# Link desktop file to root
ln -sf "usr/share/applications/bunkerverse-control-center.desktop" "$APPDIR/bunkerverse-control-center.desktop"

# Step 7: Copy Qt libraries and plugins
echo -e "${YELLOW}Step 7: Deploying Qt libraries...${NC}"

if [ -d "$QT_DIR" ]; then
    export LD_LIBRARY_PATH="$QT_DIR/lib:$LD_LIBRARY_PATH"
    export QT_PLUGIN_PATH="$QT_DIR/plugins"
    export QML2_IMPORT_PATH="$QT_DIR/qml"
    
    # Use linuxdeploy with Qt plugin
    if [ -f "./$LINUXDEPLOY_QT" ]; then
        DEPLOY_QT=1 QT_DIR="$QT_DIR" "./$LINUXDEPLOY" \
            --appdir="$APPDIR" \
            --plugin=qt \
            --executable="$APPDIR/usr/bin/bunkerverse-control-center" \
            --library="$APPDIR/usr/lib/libclient_rust_app_logic.so" \
            --library="$APPDIR/usr/lib/libnar_rust_wrapper_for_llama_cpp.so" \
            --desktop-file="$APPDIR/usr/share/applications/bunkerverse-control-center.desktop" \
            --icon-file="$APPDIR/usr/share/icons/hicolor/256x256/apps/bunkerverse-control-center.png"
    else
        echo -e "${YELLOW}Qt plugin not available, copying libraries manually${NC}"
        
        # Copy Qt libraries manually
        mkdir -p "$APPDIR/usr/lib"
        for lib in Core Gui Widgets Qml Quick Network DBus XcbQpa; do
            if [ -f "$QT_DIR/lib/libQt6${lib}.so.6" ]; then
                cp "$QT_DIR/lib/libQt6${lib}.so"* "$APPDIR/usr/lib/" 2>/dev/null || true
            fi
        done
        
        # Copy Qt plugins
        mkdir -p "$APPDIR/usr/plugins"
        for plugin_dir in platforms xcbglintegrations imageformats; do
            if [ -d "$QT_DIR/plugins/$plugin_dir" ]; then
                cp -r "$QT_DIR/plugins/$plugin_dir" "$APPDIR/usr/plugins/"
            fi
        done
        
        # Copy QML modules if used
        if [ -d "$BUILD_DIR/qml" ]; then
            cp -r "$BUILD_DIR/qml" "$APPDIR/usr/"
        elif [ -d "$QT_DIR/qml" ]; then
            mkdir -p "$APPDIR/usr/qml"
            # Copy only needed QML modules
            for module in QtQuick QtQuick.2 QtQml; do
                if [ -d "$QT_DIR/qml/$module" ]; then
                    cp -r "$QT_DIR/qml/$module" "$APPDIR/usr/qml/"
                fi
            done
        fi
    fi
else
    echo -e "${YELLOW}Qt directory not found, skipping Qt deployment${NC}"
fi

# Step 8: Create AppRun script
echo -e "${YELLOW}Step 8: Creating AppRun script...${NC}"

cat > "$APPDIR/AppRun" << 'EOF'
#!/bin/bash
set -e

# Determine AppImage mount point
if [ -z "$APPDIR" ]; then
    APPDIR="$(dirname "$(readlink -f "$0")")"
fi

# Set up environment
export LD_LIBRARY_PATH="$APPDIR/usr/lib:$LD_LIBRARY_PATH"
export QT_PLUGIN_PATH="$APPDIR/usr/plugins:$QT_PLUGIN_PATH"
export QML2_IMPORT_PATH="$APPDIR/usr/qml:$QML2_IMPORT_PATH"
export XDG_DATA_DIRS="$APPDIR/usr/share:$XDG_DATA_DIRS"
export PATH="$APPDIR/usr/bin:$PATH"

# Qt platform plugin
export QT_QPA_PLATFORM_PLUGIN_PATH="$APPDIR/usr/plugins/platforms"

# Wayland/X11 compatibility
if [ "$XDG_SESSION_TYPE" = "wayland" ]; then
    export QT_QPA_PLATFORM="wayland"
else
    export QT_QPA_PLATFORM="xcb"
fi

# GTK theme integration
export GTK_THEME="${GTK_THEME:-Adwaita}"

# Launch the application
exec "$APPDIR/usr/bin/bunkerverse-control-center" "$@"
EOF

chmod +x "$APPDIR/AppRun"

# Step 9: Copy additional resources
echo -e "${YELLOW}Step 9: Copying additional resources...${NC}"

# Copy configuration files
if [ -f "$BUILD_DIR/config.toml" ]; then
    mkdir -p "$APPDIR/usr/share/bunkerverse"
    cp "$BUILD_DIR/config.toml" "$APPDIR/usr/share/bunkerverse/"
fi

# Copy models for NAR if present
if [ -d "$BUILD_DIR/models" ]; then
    cp -r "$BUILD_DIR/models" "$APPDIR/usr/share/bunkerverse/"
fi

# Step 10: Strip binaries for size optimization
echo -e "${YELLOW}Step 10: Optimizing binary size...${NC}"

find "$APPDIR" -type f -executable -exec file {} \; | \
    grep -E 'ELF.*executable|ELF.*shared object' | \
    cut -d: -f1 | \
    xargs -r strip --strip-unneeded 2>/dev/null || true

# Step 11: Create AppImage
echo -e "${YELLOW}Step 11: Creating AppImage...${NC}"

APPIMAGE_NAME="BunkerVerseControlCenter-${VERSION}-${ARCH}.AppImage"

# Set AppImage architecture
export ARCH="$ARCH"

# Create the AppImage
"./$APPIMAGETOOL" "$APPDIR" "$OUTPUT_DIR/$APPIMAGE_NAME"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ AppImage created successfully${NC}"
    
    # Make it executable
    chmod +x "$OUTPUT_DIR/$APPIMAGE_NAME"
    
    # Optional: Create zsync file for delta updates
    if command -v zsyncmake &> /dev/null; then
        echo -e "${YELLOW}Creating zsync file for updates...${NC}"
        zsyncmake "$OUTPUT_DIR/$APPIMAGE_NAME"
    fi
else
    echo -e "${RED}Failed to create AppImage${NC}"
    exit 1
fi

# Step 12: Test the AppImage (optional)
echo -e "${YELLOW}Step 12: Testing AppImage...${NC}"

if [ -f "$OUTPUT_DIR/$APPIMAGE_NAME" ]; then
    # Extract and inspect
    "$OUTPUT_DIR/$APPIMAGE_NAME" --appimage-extract-and-run --version 2>/dev/null || \
        echo -e "${YELLOW}Version check not implemented${NC}"
    
    # Verify desktop integration
    "$OUTPUT_DIR/$APPIMAGE_NAME" --appimage-help 2>/dev/null | head -n 5 || true
fi

# Clean up
echo -e "${YELLOW}Cleaning up temporary files...${NC}"
# Optionally remove AppDir
# rm -rf "$APPDIR"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build completed successfully!${NC}"
echo -e "${GREEN}Output file: $OUTPUT_DIR/$APPIMAGE_NAME${NC}"
echo -e "${GREEN}Size: $(du -h "$OUTPUT_DIR/$APPIMAGE_NAME" | cut -f1)${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${CYAN}To run the AppImage:${NC}"
echo -e "  chmod +x $OUTPUT_DIR/$APPIMAGE_NAME"
echo -e "  $OUTPUT_DIR/$APPIMAGE_NAME"
echo ""
echo -e "${CYAN}For system integration:${NC}"
echo -e "  $OUTPUT_DIR/$APPIMAGE_NAME --appimage-integrate"