import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root
	property bool edgeSelection: false
	property int edgeWidth: 2
	property var session: null
	property var nodes: []
	property var rankY: []

	Component {
		id: node

		Rectangle {
			id: bblock

			property int lRank: lastRank != undefined ? lastRank : 0
			property int fRank: firstRank != undefined ? firstRank : 0
			property int centerX: computedX != undefined ? computedX : 0
			property real hue: {
				if(state == "prev") {
					return .4
				} else if(state == "next") {
					return .6
				} else {
					return 0
				}
			}
			property color textColor: state == "" && root.edgeSelection ? "#aaa" : "black"

			color: { state == "" && root.edgeSelection ? "#eee" : Qt.hsla(hue,.7617187500,.8125,1) }
			border { width: 1; color: state == "" && root.edgeSelection ? "#aaa" : Qt.hsla(hue,1,.2421875,1) }
			radius: 3
			smooth: true
			z: 3
			x: computedX - (bblock.width / 2) + 100
			y: {
				if(root.rankY[firstRank] != undefined) {
					return root.rankY[fRank];
				} else {
					return 0;
				}
			}

			Component.onCompleted: {
				var p = session.activeProcedure

				if(root.nodes[p] != undefined) {
					root.nodes[p].push(bblock)
				} else {
					root.nodes[p] = [bblock]
				}
			}

			Column {
				id: col
				x: 15; y: 15
				spacing: 3
				property int mnemonicWidth: 0

				Repeater {
					model: payload == undefined ? [] : eval(payload).payload
					delegate: Row {
						spacing: 5

						Text {
							text: modelData.opcode
							width: col.mnemonicWidth
							font { family: "Monospace" }
							color: textColor

							Component.onCompleted: {
								col.mnemonicWidth = Math.max(col.mnemonicWidth,contentWidth)
							}
						}

						Repeater {
							model: modelData.operands
							delegate: Text {
								font { family: "Monospace" }
								text: modelData
								color: textColor
							}
						}
					}
				}
			}

			height: col.height + 30
			width: col.width + 30

			MouseArea {
				anchors.fill: parent
				drag.target: parent
				hoverEnabled: true

				onPositionChanged: {
					if(pressed) {
						//sugiyama.direct = true
						//edgeColor = "gray"
					}
					sugiyama.route()
				}

				onReleased: {
						//sugiyama.direct = false
						sugiyama.route()
				}

				onEntered: {
					for(var i in incomingEdges) {
						incomingNodes[i].state = "prev"
						incomingEdges[i].state = "prev"
					}
					for(var i in outgoingEdges) {
						outgoingNodes[i].state = "next"
						outgoingEdges[i].state = "next"
					}
					root.edgeSelection = true
					bblock.state = "active"
				}
				onExited: {
					for(var i in incomingEdges) {
						incomingNodes[i].state = ""
						incomingEdges[i].state = ""
					}
					for(var i in outgoingEdges) {
						outgoingNodes[i].state = ""
						outgoingEdges[i].state = ""
					}
					root.edgeSelection = false
					bblock.state = ""
				}
			}
		}
	}

	Component {
		id: arrow

		Canvas {
			id: arrow_cv
			height: 30; width: 15
			z: 4

			onPaint: {
				var ctx = arrow_cv.getContext("2d")

				if(ctx != null) {
					ctx.lineWidth = 1

					ctx.beginPath()
					ctx.fillStyle = edge.color;
					ctx.strokeStyle = edge.color;
					ctx.moveTo(.5 * arrow_cv.width,.5 * arrow_cv.height);
					ctx.lineTo(0,arrow_cv.height - 1);
					ctx.lineTo(.5 * arrow_cv.width,.75 * arrow_cv.height);
					ctx.lineTo(arrow_cv.width - 1,arrow_cv.height - 1);
					ctx.lineTo(.5 * arrow_cv.width,.5 * arrow_cv.height);
					ctx.fill()
				}
			}

			Component.onCompleted: {
				edge.colorChanged.connect(function() { arrow_cv.requestPaint() })
			}
		}
	}

	Component {
		id: edge

		Edge {
			property string type: ""
			property string condition: ""
			property string state: ""

			color: {
				if(state == "" && root.edgeSelection) {
					return "gray"
				} else {
					if(type == "true") {
						return "green"
					} else if(type == "false") {
						return "red"
					} else {
						return "blue"
					}
				}
			}

			width: state == "" && root.edgeSelection ? root.edgeWidth / 2 : root.edgeWidth
			head: arrow
			label: Component {
				Item {
					height: label.height + 4
					width: label.width + 4
					z: 5
					visible: edge.type != "unconditional" && edge.state != ""

					Rectangle {
						anchors.fill: parent
						color: "gray"
						opacity: 0.8
						border { width: 1; color: "black" }
						radius: 3
					}

					Text {
						x: 2; y: 2
						id: "label"
						text: edge.condition
						font { pixelSize: 14 }
					}
				}
			}
		}
	}

	Flickable {
		id: flick
		width: parent.width
		height: parent.height
		clip: true
		contentWidth: Math.max(sugiyama.width,root.width * 2)
		contentHeight: Math.max(sugiyama.height,root.height)

		Sugiyama {
			id: sugiyama

			property var rankStart: []

			x: (childrenRect.width < root.width * 2 ? ((root.width - childrenRect.width) / 2) : 0)
			width: Math.max(childrenRect.width,root.width + 100)
			height: Math.max(2*childrenRect.height,root.height)

			procedure: root.session.activeProcedure
			vertex: node
			edge: edge

			onLayoutDone: {
				var p = session.activeProcedure
				var rankHeights = []

				root.rankY[p] = []

				for(var n in root.nodes[p]) {
					var node = root.nodes[p][n]
					rankHeights[node.fRank] = Math.max(rankHeights[node.fRank] != undefined ? rankHeights[node.fRank] : 0,node.height);
				}

				for(var m in rankHeights) {
					root.rankY[p][m] = rankHeights.reduce(function(a,n,i,all) {
						if(i < m) {
							return a + n + 100
						} else {
							return a
						}
					},100);
				}

				for(var n in root.nodes[p]) {
					var node = root.nodes[p][n]
					node.y = rankY[p][node.fRank];
				}
			}

			onProcedureChanged: {
				var p = session.activeProcedure

				if(root.rankY[p] == undefined) {
					root.rankY[p] = []
				}
				if(root.nodes[p] == undefined) {
					root.nodes[p] = []
				}
			}
		}
	}

	MouseArea {
		anchors.fill: flick

		onPressed: {
			mouse.accepted = false;
		}

		onWheel: {
			if(wheel.modifiers & Qt.ControlModifier) {
				sugiyama.scale += wheel.angleDelta.y / 1000
				wheel.accepted = true
			} else {
				wheel.accepted = false
			}
		}
	}
}
