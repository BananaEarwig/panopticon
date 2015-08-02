import QtQuick 2.0
import Panopticon 1.0
import QtQuick.Controls 1.3

Item {
	id: root

	signal activated(string uuid);

	readonly property int nodePadding: 3;
	readonly property int labelHeight: 12;
	readonly property string nodeColor: "#a7a37e";
	readonly property string edgeColor: "#046380";
	readonly property int edgeWidth: 3;

	property string selection: "";

	onSelectionChanged: callgraph.requestPaint()

	Component.onCompleted: {
		layoutTask.sendMessage({"type":"resize","width":callgraph.width,"height":callgraph.height});
		timer.running = true;

		Panopticon.finishedFunction.connect(function(uu) {
			var obj = eval(Panopticon.functionInfo(uu));

			functionModel.append(obj);
			layoutTask.sendMessage({"type":"add","item":obj});
			timer.running = true;
		});
	}

	ListModel {
		id: functionModel
	}

	Timer {
		id: timer;
		interval: 0
		running: false;
		onTriggered: layoutTask.sendMessage({"type":"tick"});
	}

	Rectangle {
		color: "#efecca"
		anchors.fill: parent

		Canvas {
			id: callgraph
			anchors.fill: parent

			function nodeBoundingBox(i) {
				var func = functionModel.get(i);
				var ctx = callgraph.getContext('2d');
				var lb_w = ctx.measureText(func.name).width;

				return Qt.rect(func.x - lb_w / 2 - root.nodePadding,func.y - root.labelHeight / 2 - root.nodePadding,
											 lb_w + root.nodePadding * 2,root.labelHeight + root.nodePadding * 2);
			}

			onPaint: {
				var ctx = callgraph.getContext('2d');
				ctx.textAlign = "center";
				ctx.textBaseline = "middle";
				ctx.font = root.labelHeight + "px monospace";

				// clear background
				ctx.clearRect(0,0,width,height);

				// edges
				ctx.beginPath();
				for(var i = 0; i < functionModel.count; ++i) {
					var from = functionModel.get(i);

					for(var e in from.calls) {
						var edge = from.calls[e];

						for(var j = 0; j < functionModel.count; ++j) {
							var to = functionModel.get(j);

							if(to.uuid == edge) {
								ctx.moveTo(from.x,from.y);
								ctx.lineTo(to.x,to.y);
							}
						}
					}
				}
				ctx.strokeStyle = root.edgeColor;
				ctx.lineWidth = root.edgeWidth;
				ctx.stroke();

				// nodes
				for(var i = 0; i < functionModel.count; ++i) {
					var func = functionModel.get(i);
					var bb = nodeBoundingBox(i);

					if(root.selection != func.uuid) {
						ctx.clearRect(bb.x,bb.y,bb.width,bb.height);
					} else {
						ctx.fillStyle = nodeColor;
						ctx.fillRect(bb.x,bb.y,bb.width,bb.height);
					}

					ctx.fillStyle = "black";
					ctx.fillText(func.name,func.x,func.y);
				}
			}

			MouseArea {
				anchors.fill: parent

				function nodeAt(x,y) {
					for(var i = 0; i < functionModel.count; ++i) {
						var func = functionModel.get(i);
						var bb = callgraph.nodeBoundingBox(i);

						if(bb.x <= x && bb.x + bb.width >= x &&
							 bb.y <= y && bb.y + bb.height >= y) {
							return i;
						}
					}

					return -1;
				}

				onClicked: {
					var i = nodeAt(mouse.x,mouse.y);

					if(i > -1) {
						var func = functionModel.get(i);
						root.selection = func.uuid;
					} else {
						root.selection = "";
					}
				}

				onDoubleClicked: {
					var i = nodeAt(mouse.x,mouse.y);

					if(i > -1) {
						var func = functionModel.get(i);

						root.activated(func.uuid);
					}
				}
			}
		}
	}

	WorkerScript {
		id: layoutTask
		source: "../layout.js"
		onMessage: {
			//console.log("MS: " + JSON.stringify(messageObject));

			if(messageObject.type == "tock") {
				for(var i = 0; i < functionModel.count; i++) {
					var node = functionModel.get(i);

					if(messageObject.nodes[node.uuid] !== undefined) {
						functionModel.setProperty(i,"x",messageObject.nodes[node.uuid].x);
						functionModel.setProperty(i,"y",messageObject.nodes[node.uuid].y);
					}
				}
			}

			timer.running = messageObject.type !== "stop";
			callgraph.requestPaint();
		}
	}
}
