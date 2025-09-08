// BUNKERVERSE CXX-Qt Client - Main QML Interface
// Demonstrates Qt 6 QML integration with Rust backend via CXX-Qt

import QtQuick 6.9
import QtQuick.Controls 6.9
import QtQuick.Layouts 6.9
import QtQuick.Window 6.9

ApplicationWindow {
    id: mainWindow
    title: "BUNKERVERSE Control Center"
    width: 1400
    height: 900
    visible: true
    flags: Qt.Window | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowMinMaxButtonsHint | Qt.WindowCloseButtonHint

    property bool authenticated: false
    property string currentUser: ""
    property var dashboardData: null
    property bool darkMode: true

    // Color scheme
    property color primaryColor: darkMode ? "#1e1e2e" : "#ffffff"
    property color secondaryColor: darkMode ? "#313244" : "#f5f5f5"
    property color accentColor: "#89b4fa"
    property color textColor: darkMode ? "#cdd6f4" : "#11111b"
    property color successColor: "#a6e3a1"
    property color errorColor: "#f38ba8"

    background: Rectangle {
        color: primaryColor
        
        // Animated gradient background
        LinearGradient {
            anchors.fill: parent
            start: Qt.point(0, 0)
            end: Qt.point(width, height)
            gradient: Gradient {
                GradientStop { position: 0.0; color: Qt.rgba(30/255, 30/255, 46/255, 1.0) }
                GradientStop { position: 1.0; color: Qt.rgba(49/255, 50/255, 68/255, 1.0) }
            }
        }
    }

    // Header Bar
    Rectangle {
        id: headerBar
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 80
        color: Qt.rgba(0, 0, 0, 0.2)
        
        RowLayout {
            anchors.fill: parent
            anchors.margins: 20

            // Logo and Title
            RowLayout {
                spacing: 15
                
                Rectangle {
                    width: 50
                    height: 50
                    color: accentColor
                    radius: 10
                    
                    Text {
                        anchors.centerIn: parent
                        text: "🏰"
                        font.pixelSize: 24
                    }
                }

                Column {
                    Text {
                        text: "BUNKERVERSE"
                        font.pixelSize: 24
                        font.bold: true
                        color: textColor
                    }
                    Text {
                        text: "CXX-Qt Client v1.0"
                        font.pixelSize: 12
                        color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.7)
                    }
                }
            }

            Item { Layout.fillWidth: true }

            // User Profile Section
            RowLayout {
                spacing: 10
                visible: authenticated

                Text {
                    text: currentUser || "User"
                    color: textColor
                    font.pixelSize: 14
                }

                Button {
                    text: "Settings"
                    Material.background: secondaryColor
                    Material.foreground: textColor
                    onClicked: settingsDialog.open()
                }

                Button {
                    text: "Logout"
                    Material.background: errorColor
                    Material.foreground: "white"
                    onClicked: handleLogout()
                }
            }

            // Connection Status
            Rectangle {
                width: 12
                height: 12
                radius: 6
                color: authenticated ? successColor : errorColor
                
                SequentialAnimation on opacity {
                    running: !authenticated
                    loops: Animation.Infinite
                    NumberAnimation { to: 0.3; duration: 1000 }
                    NumberAnimation { to: 1.0; duration: 1000 }
                }
            }
        }
    }

    // Main Content Area
    StackView {
        id: contentStack
        anchors.top: headerBar.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        anchors.margins: 20

        initialItem: authenticated ? dashboardView : loginView
    }

    // Login View Component
    Component {
        id: loginView
        
        Rectangle {
            color: "transparent"
            
            Item {
                anchors.centerIn: parent
                width: 400
                height: 500

                Rectangle {
                    anchors.fill: parent
                    color: secondaryColor
                    radius: 15
                    opacity: 0.9

                    Column {
                        anchors.centerIn: parent
                        spacing: 30
                        width: parent.width - 80

                        Text {
                            text: "zkLogin Authentication"
                            font.pixelSize: 24
                            font.bold: true
                            color: textColor
                            anchors.horizontalCenter: parent.horizontalCenter
                        }

                        Text {
                            text: "Choose your preferred authentication provider"
                            font.pixelSize: 14
                            color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.8)
                            anchors.horizontalCenter: parent.horizontalCenter
                            wrapMode: Text.WordWrap
                            width: parent.width
                            horizontalAlignment: Text.AlignHCenter
                        }

                        Column {
                            anchors.horizontalCenter: parent.horizontalCenter
                            spacing: 15
                            width: parent.width

                            AuthButton {
                                provider: "Google"
                                icon: "🔵"
                                onClicked: handleAuthentication("google")
                            }

                            AuthButton {
                                provider: "GitHub"
                                icon: "⚫"
                                onClicked: handleAuthentication("github")
                            }

                            AuthButton {
                                provider: "Discord"
                                icon: "🟣"
                                onClicked: handleAuthentication("discord")
                            }

                            AuthButton {
                                provider: "Ethereum Wallet"
                                icon: "🔗"
                                onClicked: handleAuthentication("ethereum")
                            }
                        }

                        Text {
                            text: "Secured with zero-knowledge proofs"
                            font.pixelSize: 12
                            color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.6)
                            anchors.horizontalCenter: parent.horizontalCenter
                        }
                    }
                }
            }
        }
    }

    // Dashboard View Component
    Component {
        id: dashboardView
        
        ScrollView {
            clip: true
            
            ColumnLayout {
                width: contentStack.width - 40
                spacing: 30

                // Welcome Section
                Rectangle {
                    Layout.fillWidth: true
                    height: 120
                    color: secondaryColor
                    radius: 12

                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 30

                        Column {
                            Layout.fillWidth: true
                            spacing: 10

                            Text {
                                text: `Welcome back, ${currentUser}!`
                                font.pixelSize: 28
                                font.bold: true
                                color: textColor
                            }

                            Text {
                                text: "Your BUNKERVERSE dashboard is ready"
                                font.pixelSize: 16
                                color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.8)
                            }
                        }

                        Button {
                            text: "Refresh Data"
                            Material.background: accentColor
                            Material.foreground: "white"
                            onClicked: refreshDashboard()
                        }
                    }
                }

                // Quick Stats Grid
                GridLayout {
                    Layout.fillWidth: true
                    columns: 4
                    columnSpacing: 20
                    rowSpacing: 20

                    StatCard {
                        title: "Account Balance"
                        value: "2,450.75 BNK"
                        icon: "💰"
                        trend: "+5.2%"
                        trendPositive: true
                    }

                    StatCard {
                        title: "AI Queries Today"
                        value: "127"
                        icon: "🤖"
                        trend: "+12"
                        trendPositive: true
                    }

                    StatCard {
                        title: "Storage Used"
                        value: "4.2 GB"
                        icon: "💾"
                        trend: "89% of limit"
                        trendPositive: false
                    }

                    StatCard {
                        title: "NFT Collections"
                        value: "23"
                        icon: "🎨"
                        trend: "+3 this week"
                        trendPositive: true
                    }
                }

                // Main Features Grid
                GridLayout {
                    Layout.fillWidth: true
                    columns: 2
                    columnSpacing: 20
                    rowSpacing: 20

                    FeatureCard {
                        title: "AI Assistant"
                        description: "Chat with NAR/Gemma3 AI model"
                        icon: "🧠"
                        color: "#a6e3a1"
                        onClicked: openAIAssistant()
                    }

                    FeatureCard {
                        title: "NFT Marketplace"
                        description: "Buy, sell, and trade digital assets"
                        icon: "🏪"
                        color: "#f9e2af"
                        onClicked: openMarketplace()
                    }

                    FeatureCard {
                        title: "Data Storage"
                        description: "Manage your decentralized storage"
                        icon: "🗄️"
                        color: "#89dceb"
                        onClicked: openStorage()
                    }

                    FeatureCard {
                        title: "Analytics"
                        description: "View platform usage statistics"
                        icon: "📊"
                        color: "#cba6f7"
                        onClicked: openAnalytics()
                    }
                }

                // Recent Activity
                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 300
                    color: secondaryColor
                    radius: 12

                    Column {
                        anchors.fill: parent
                        anchors.margins: 20

                        Text {
                            text: "Recent Activity"
                            font.pixelSize: 20
                            font.bold: true
                            color: textColor
                            anchors.horizontalCenter: parent.horizontalCenter
                        }

                        ListView {
                            width: parent.width
                            height: parent.height - 40
                            model: recentActivityModel
                            
                            delegate: Item {
                                width: ListView.view.width
                                height: 60

                                Rectangle {
                                    anchors.fill: parent
                                    anchors.margins: 5
                                    color: Qt.rgba(accentColor.r, accentColor.g, accentColor.b, 0.1)
                                    radius: 8

                                    RowLayout {
                                        anchors.fill: parent
                                        anchors.margins: 15

                                        Text {
                                            text: model.icon || "📄"
                                            font.pixelSize: 20
                                        }

                                        Column {
                                            Layout.fillWidth: true
                                            Text {
                                                text: model.action || "Unknown Action"
                                                color: textColor
                                                font.pixelSize: 14
                                                font.bold: true
                                            }
                                            Text {
                                                text: model.timestamp || "Unknown Time"
                                                color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.6)
                                                font.pixelSize: 12
                                            }
                                        }

                                        Text {
                                            text: model.status || ""
                                            color: model.success ? successColor : errorColor
                                            font.pixelSize: 12
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Custom Components
    component AuthButton: Button {
        property string provider: ""
        property string icon: ""
        
        width: parent.width
        height: 50
        
        background: Rectangle {
            color: parent.hovered ? Qt.lighter(accentColor, 1.1) : accentColor
            radius: 8
            border.width: parent.focus ? 2 : 0
            border.color: Qt.lighter(accentColor, 1.3)
            
            Behavior on color {
                ColorAnimation { duration: 200 }
            }
        }
        
        contentItem: RowLayout {
            anchors.centerIn: parent
            spacing: 10
            
            Text {
                text: icon
                font.pixelSize: 16
            }
            
            Text {
                text: `Continue with ${provider}`
                color: "white"
                font.pixelSize: 14
                font.bold: true
            }
        }
    }

    component StatCard: Rectangle {
        property string title: ""
        property string value: ""
        property string icon: ""
        property string trend: ""
        property bool trendPositive: true
        
        Layout.fillWidth: true
        Layout.preferredHeight: 120
        color: secondaryColor
        radius: 10
        
        Column {
            anchors.centerIn: parent
            spacing: 8
            
            Text {
                text: icon
                font.pixelSize: 24
                anchors.horizontalCenter: parent.horizontalCenter
            }
            
            Text {
                text: value
                font.pixelSize: 20
                font.bold: true
                color: textColor
                anchors.horizontalCenter: parent.horizontalCenter
            }
            
            Text {
                text: title
                font.pixelSize: 12
                color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.8)
                anchors.horizontalCenter: parent.horizontalCenter
            }
            
            Text {
                text: trend
                font.pixelSize: 10
                color: trendPositive ? successColor : errorColor
                anchors.horizontalCenter: parent.horizontalCenter
            }
        }
    }

    component FeatureCard: Rectangle {
        property string title: ""
        property string description: ""
        property string icon: ""
        property color color: accentColor
        signal clicked()
        
        Layout.fillWidth: true
        Layout.preferredHeight: 150
        color: parent.color
        radius: 12
        border.width: hovered ? 2 : 1
        border.color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.2)
        
        property bool hovered: false
        
        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onEntered: parent.hovered = true
            onExited: parent.hovered = false
            onClicked: parent.clicked()
        }
        
        Column {
            anchors.centerIn: parent
            spacing: 12
            
            Rectangle {
                width: 50
                height: 50
                color: parent.parent.color
                radius: 25
                anchors.horizontalCenter: parent.horizontalCenter
                
                Text {
                    anchors.centerIn: parent
                    text: icon
                    font.pixelSize: 24
                }
            }
            
            Text {
                text: title
                font.pixelSize: 16
                font.bold: true
                color: textColor
                anchors.horizontalCenter: parent.horizontalCenter
            }
            
            Text {
                text: description
                font.pixelSize: 12
                color: Qt.rgba(textColor.r, textColor.g, textColor.b, 0.8)
                anchors.horizontalCenter: parent.horizontalCenter
                wrapMode: Text.WordWrap
                width: 180
                horizontalAlignment: Text.AlignHCenter
            }
        }
    }

    // Data Models
    ListModel {
        id: recentActivityModel
        
        ListElement {
            icon: "🔐"
            action: "User authenticated via Google"
            timestamp: "2 minutes ago"
            success: true
            status: "✓"
        }
        
        ListElement {
            icon: "🤖"
            action: "AI query processed"
            timestamp: "15 minutes ago"
            success: true
            status: "✓"
        }
        
        ListElement {
            icon: "💰"
            action: "Token transfer completed"
            timestamp: "1 hour ago"
            success: true
            status: "✓"
        }
        
        ListElement {
            icon: "🗄️"
            action: "Data backup initiated"
            timestamp: "3 hours ago"
            success: false
            status: "⚠"
        }
    }

    // Settings Dialog
    Dialog {
        id: settingsDialog
        title: "Settings"
        width: 500
        height: 400
        anchors.centerIn: parent
        modal: true

        Column {
            anchors.fill: parent
            spacing: 20

            Text {
                text: "Application Settings"
                font.pixelSize: 18
                font.bold: true
                color: textColor
            }

            CheckBox {
                text: "Dark Mode"
                checked: darkMode
                onCheckedChanged: darkMode = checked
            }

            CheckBox {
                text: "Enable Notifications"
                checked: true
            }

            CheckBox {
                text: "Auto-refresh Dashboard"
                checked: true
            }
        }
    }

    // JavaScript Functions
    function handleAuthentication(provider) {
        console.log(`Authenticating with ${provider}...`)
        // Simulate authentication process
        authTimer.provider = provider
        authTimer.start()
    }

    function handleLogout() {
        authenticated = false
        currentUser = ""
        contentStack.replace(loginView)
    }

    function refreshDashboard() {
        console.log("Refreshing dashboard data...")
        // Simulate data refresh
    }

    function openAIAssistant() {
        console.log("Opening AI Assistant...")
    }

    function openMarketplace() {
        console.log("Opening NFT Marketplace...")
    }

    function openStorage() {
        console.log("Opening Data Storage...")
    }

    function openAnalytics() {
        console.log("Opening Analytics...")
    }

    // Authentication Timer
    Timer {
        id: authTimer
        interval: 2000
        property string provider: ""
        onTriggered: {
            authenticated = true
            currentUser = `${provider.charAt(0).toUpperCase()}${provider.slice(1)} User`
            contentStack.replace(dashboardView)
            console.log(`Successfully authenticated with ${provider}`)
        }
    }
}