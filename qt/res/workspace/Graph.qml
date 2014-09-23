import QtQuick 2.0
import QtQuick.Controls 1.0
import Panopticon 1.0

Item {
	id: root
	property color edgeColor: "black"
	property int edgeWidth: 3
	property var session: null
	property var nodes: []

	onSessionChanged: {
		if(session != null) {
			session.graph.jumpsChanged.connect(sugiyama.rebuildEdges)
			sugiyama.rebuildEdges()
		}
	}


	Component {
		id: node

		Rectangle {
			id: bblock

			property int lRank: lastRank
			property int fRank: firstRank
			property int ordNr: ordinal
			property real hue: {
				if(state == "prev") {
					return .4
				} else if(state == "next") {
					return .6
				} else {
					return 0
				}
			}

			color: { Qt.hsla(hue,.7617187500,.8125,1) }
			border { width: 1; color: Qt.hsla(hue,1,.2421875,1) }
			radius: 3
			smooth: true
			z: 2
			x: computedX - width / 2
			/*{
				var w = root.nodes.reduce(function(a,n,i,all) { if(n.fRank == firstRank) { return Math.max(a,n.width + 50) } else { return a } },0) + 50
				return (root.childrenRect.width - w) / 2 + root.nodes.reduce(function(a,n,i,all) { if(n.fRank == firstRank && n.ordNr < ordinal) { return Math.max(a,n.width + 50) } else { return a } },0) + 50
			}*/


			y: root.nodes.reduce(function(a,n,i,all) { if(n.lRank == firstRank - 1) { return Math.max(a,n.y + n.height) } else { return a } },0) + 50

			property int bbid: modelData

			Component.onCompleted: {
				if(root.nodes != undefined) {
					root.nodes.push(bblock)
				} else {
					root.nodes = [bblock]
				}
			}

			Column {
				id: col
				x: 15; y: 15
				spacing: 3
				property int mnemonicWidth: 0

				Repeater {
					model: eval(session.graph.mnemonics)[bbid]
					delegate: Row {
						spacing: 5

						Text {
							text: modelData.op
							width: col.mnemonicWidth
							font { family: "Monospace" }

							Component.onCompleted: {
								col.mnemonicWidth = Math.max(col.mnemonicWidth,contentWidth)
							}

						}

						Repeater {
							model: modelData.args
							delegate: Text {
								font { family: "Monospace" }
								text: modelData
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
						sugiyama.direct = true
						edgeColor = "gray"
						sugiyama.route()
					}
				}

				onReleased: {
					sugiyama.direct = false
					edgeColor = "black"
					sugiyama.route()
				}

				onEntered: {
					for(var i in incomingEdges) {
						incomingEdges[i].color = "blue"
						incomingNodes[i].state = "prev"
					}
					for(var i in outgoingEdges) {
						outgoingEdges[i].color = "red"
						outgoingNodes[i].state = "next"
					}

				}
				onExited: {
					for(var i in incomingEdges) {
						incomingEdges[i].color = "black"
						incomingNodes[i].state = ""
					}
					for(var i in outgoingEdges) {
						outgoingEdges[i].color = "black"
						outgoingNodes[i].state = ""
					}
				}
			}
		}
	}

	Component {
		id: arrow

		Canvas {
			id: cv
			height: 40; width: 20

			onPaint: {
				var ctx = cv.getContext("2d")

				if(ctx != null) {
					ctx.lineWidth = 0

					ctx.beginPath()
					ctx.fillStyle = "black"
					ctx.moveTo(.5 * cv.width,.5 * cv.height);
					ctx.lineTo(0,cv.height - 1);
					ctx.lineTo(.5 * cv.width,.75 * cv.height);
					ctx.lineTo(cv.width - 1,cv.height - 1);
					ctx.lineTo(.5 * cv.width,.5 * cv.height);
					ctx.fill()
				}
			}
		}
	}

	Component {
		id: edge

		Edge {
			color: root.edgeColor
			width: root.edgeWidth
			head: arrow
		}
	}

	Flickable {
		id: flick
		anchors.fill: parent
		clip: true
		contentWidth: sugiyama.width
		contentHeight: sugiyama.height

		Sugiyama {
			id: sugiyama

			height: childrenRect.height
			width: childrenRect.width
			delegate: node

			function rebuildEdges() {
				for(var a in root.session.graph.jumps) {
					var e = eval(root.session.graph.jumps[a])
					var x = edge.createObject(sugiyama,e)

					sugiyama.edges = [].concat(sugiyama.edges,[x])
				}
			}

			vertices: root.session ? root.session.graph.blocks.map(function(a) { return eval(a) }) : []
			edges: []
		}
	}

	MouseArea {
		anchors.fill: flick

		onPressed: {
			mouse.accepted = false
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
