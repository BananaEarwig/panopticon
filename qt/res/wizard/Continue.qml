import QtQuick 2.0
import Panopticon 1.0
import "../"
import Qt.labs.settings 1.0

Item {
	Loader {
		property variant session: null

		id: loader
		height: parent.height
		width: parent.width
		anchors.left: root.right
	}

	Page {
		id: root
		anchors.fill: parent
		primaryTitle: "Continue Session"
		secondaryTitle: "Recent sessions"
		primaryAction: "Quit"

		Behavior on x {
			NumberAnimation { duration: 300 }
		}

		Settings {
			id: settings
			property variant recent: []
		}

		Item {
			height: childrenRect.height
			width: childrenRect.width
			anchors.centerIn: parent

			Column {
				spacing: 100

				Repeater {
					model: settings.recent
					delegate: Item {
						height: 80
						width: 300

						Text {
							centerIn: parent
							text: modelData
						}

						MouseArea {
							anchors.fill: parent

							onPressed: {
								root.anchors.fill = undefined
								root.x = -1 * root.width
								loader.session = Panopticon.openSession("old.panop")
								loader.source = "../workspace/Workspace.qml"
							}
						}

						Rectangle {
							color: "green"
							anchors.fill: parent
						}
					}
				}
			}
		}
	}
}
