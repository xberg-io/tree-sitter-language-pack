import QtQuick 2.15

Rectangle {
    id: root

    component Badge: Text {
        color: "red"
    }

    property string title: "Card"
    property int count: 0
    signal clicked(int index)

    function increment() {
        count += 1
        root.clicked(count)
        console.log(qsTr("Count: %1").arg(count))
    }

    Text {
        id: label
        text: root.title
    }
}
