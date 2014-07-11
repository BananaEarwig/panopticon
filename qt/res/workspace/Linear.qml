import QtQuick 2.1
import QtQuick.Controls 1.2

Item {
	id: root

	property var session: null
	property int cursorUpperCol: -1
	property int cursorLowerCol: -1
	property int cursorUpperRow: -1
	property int cursorLowerRow: -1
	property int addressColumnWidth: 0

	readonly property string fontFamily: "Monospace"
	readonly property real fontPointSize: 7
	readonly property real cellSize: 23
	readonly property real halfCellSize: 12

	anchors.fill: parent
	focus: true

	Component {
		id: row

		Row {
			id: rowContents
			spacing: 25

			property var contents: eval(model.display)
			property int rowIndex: index

			function columnAt(x) {
				var i = 0
				var cols = [hexCol, textCol]
				var rep = 0

				while(i < cols.length)
				{
					var c = cols[i++]
					if(c.x <= x && c.x + c.width > x)
					{
						rep = c
						break;
					}
				}

				i = 0
				while(rep != 0 && i < rep.children.length) {
					var itm = rep.children[i]

					if(itm.x + rep.x <= x && itm.x + rep.x + itm.width > x && contents.hex[i] != '')
						return i
					++i
				}

				return -1
			}

			Item {
				id: addrCol
				height: root.cellSize
				width: root.addressColumnWidth

				Text {
					height: parent.height
					text: contents.address
					verticalAlignment: Text.AlignVCenter
					font { family: root.fontFamily; pointSize: root.fontPointSize }

					onWidthChanged: { root.addressColumnWidth = Math.max(root.addressColumnWidth,width) }
				}
			}

			Row {
				id: hexCol

				Repeater {
					model: contents.hex
					delegate: Rectangle {
						property int colIndex: index
						height: root.cellSize
						width: root.cellSize
						color: {
							var c = ((cursorUpperRow < rowIndex && cursorLowerRow > rowIndex) ||
											 (cursorUpperRow == rowIndex && cursorLowerRow != rowIndex && cursorUpperCol <= colIndex) ||
											 (cursorLowerRow == rowIndex && cursorUpperRow != rowIndex && cursorLowerCol >= colIndex) ||
											 (cursorLowerRow == rowIndex && cursorUpperRow == rowIndex && cursorUpperCol <= colIndex && cursorLowerCol >= colIndex))
							return c ? "red" : "white"
						}

						Text {
							anchors.centerIn: parent
							text: modelData
							font { family: root.fontFamily; pointSize: root.fontPointSize }
						}
					}
				}
			}

			Row {
				id: textCol

				Repeater {
					model: contents.text
					delegate: Rectangle {
						property int colIndex: index
						height: root.cellSize
						width: root.halfCellSize
						color: {
							var c = ((cursorUpperRow < rowIndex && cursorLowerRow > rowIndex) ||
											 (cursorUpperRow == rowIndex && cursorLowerRow != rowIndex && cursorUpperCol <= colIndex) ||
											 (cursorLowerRow == rowIndex && cursorUpperRow != rowIndex && cursorLowerCol >= colIndex) ||
											 (cursorLowerRow == rowIndex && cursorUpperRow == rowIndex && cursorUpperCol <= colIndex && cursorLowerCol >= colIndex))
							return c ? "red" : "white"
						}

						Text {
							anchors.centerIn: parent
							text: modelData
							font { family: root.fontFamily; pointSize: root.fontPointSize }
						}
					}
				}
			}

			Item {
				width: childrenRect.width; height: childrenRect.height
				x:childrenRect.x; y: childrenRect.y

				Rectangle {
					anchors.fill: commentCol
					color: commentCol.activeFocus ? "red" : "#00000000"
				}

				TextEdit {
					id: commentCol

					readOnly: false
					text: contents.comment
					width: 300
					height: root.cellSize * Math.max(1,lineCount)

					Keys.enabled: true
					Keys.priority: Keys.BeforeItem
					Keys.onPressed: {
						if((event.key == Qt.Key_Enter && (event.modifiers & Qt.ShiftModifier) == 0) ||
							 (event.key == Qt.Key_Return && (event.modifiers & Qt.ShiftModifier) == 0)) {
							root.session.postComment(rowIndex,text)
							text = contents.comment
							focus = false
							event.accepted = true
						}
						else if(event.key == Qt.Key_Escape) {
							text = contents.comment
							focus = false
							event.accepted = true
						}
					}
				}
			}
		}
	}

	ListView {
		id: lst
		anchors.fill: parent
		model: root.session.linear
		delegate: row
	}

	MouseArea {
		property var cursor: null
		property int anchorRow: -1
		property int anchorCol: -1

		anchors.fill: lst

		onPressed: {
			if(state == "") {
				var row = lst.indexAt(mouse.x + lst.contentX,mouse.y + lst.contentY)
				var rowItm = lst.itemAt(mouse.x + lst.contentX,mouse.y + lst.contentY)

				if(row >= 0 && rowItm.columnAt != undefined) {
					var col = rowItm.columnAt(mouse.x + lst.contentX)
					if(col >= 0) {
						anchorRow = row
						anchorCol = col
						state = "selecting"
						updateCursor()
						mouse.accepted = true
					} else {
						mouse.accepted = false
					}
				}
			} else if(state == "selected") {
				state = ""
				cursorUpperRow = -1
				cursorLowerRow = -1
				cursorUpperCol = -1
				cursorLowerCol = -1
				mouse.accepted = true
			}
		}

		onReleased: {
			if(state == "selecting") {
				mouse.accepted = true
				anchorRow = -1
				anchorCol = -1
				state = "selected"
			}
		}

		onPositionChanged: {
			if(state == "selecting") {
				updateCursor()
				mouse.accepted = true
			}
		}

		function updateCursor() {
			if(state == "selecting") {
				var row = lst.indexAt(mouseX + lst.contentX,mouseY + lst.contentY)
				var rowItm = lst.itemAt(mouseX + lst.contentX,mouseY + lst.contentY)

				if(row >= 0 && rowItm.columnAt != undefined) {
					var col = rowItm.columnAt(mouseX + lst.contentX)
					var u = -1, d = -1, f = -1, l = -1

					if(col >= 0) {
						if(row < anchorRow) {
							u = row
							f = col
							d = anchorRow
							l = anchorCol
						} else if(row > anchorRow) {
							d = row
							l = col
							u = anchorRow
							f = anchorCol
						} else if(row == anchorRow) {
							u = row
							d = row
							f = Math.min(col,anchorCol)
							l = Math.max(col,anchorCol)
						}

						root.cursorUpperRow = u
						root.cursorLowerRow = d
						root.cursorUpperCol = f
						root.cursorLowerCol = l
					}
				}
			}
		}
	}
}
