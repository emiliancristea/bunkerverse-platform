import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

ApplicationWindow {
    id: window
    width: 1200
    height: 800
    visible: true
    title: "BUNKERVERSE Control Center"
    
    // Placeholder main window for Control Center Client
    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 20
        
        Label {
            text: "BUNKERVERSE Control Center"
            font.pointSize: 24
            font.bold: true
            horizontalAlignment: Text.AlignHCenter
            Layout.fillWidth: true
        }
        
        Label {
            text: "Client UI Framework - CXX-Qt Integration Pending"
            font.pointSize: 12
            horizontalAlignment: Text.AlignHCenter
            Layout.fillWidth: true
        }
        
        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }
}