#include <mutex>

#include <QtQml>
#include <QtQuick>

#include <boost/optional.hpp>

#include <panopticon/procedure.hh>

#include "session.hh"

#pragma once

struct node_proxy
{
	node_proxy(QQmlComponent* comp,QQuickItem* parent)
	: item(nullptr), context(nullptr)
	{
		if(comp)
		{
			context = new QQmlContext(QQmlEngine::contextForObject(parent));
			context->setContextProperty("modelData",QVariant());
			context->setContextProperty("incomingEdges",QVariantList());
			context->setContextProperty("incomingNodes",QVariantList());
			context->setContextProperty("outgoingNodes",QVariantList());
			context->setContextProperty("outgoingEdges",QVariantList());
			context->setContextProperty("firstRank",QVariant());
			context->setContextProperty("lastRank",QVariant());
			context->setContextProperty("computedX",QVariant());
			context->setContextProperty("payload",QVariant());
			item = qobject_cast<QQuickItem*>(comp->create(context));
			item->setParentItem(parent);
		}
	}

	QQuickItem* item;
	QQmlContext* context;
};

struct edge_proxy
{
	edge_proxy(QQmlComponent* comp,QQuickItem* parent)
	: label(nullptr), head(nullptr), tail(nullptr),
		label_context(nullptr), head_context(nullptr), tail_context(nullptr)
	{
		if(comp)
		{
			edge_context = new QQmlContext(QQmlEngine::contextForObject(parent));
			edge = qobject_cast<QObject*>(comp->create(edge_context));
			edge->setParent(parent);
		}
	}

	QObject *edge;
	QQuickItem *label, *head, *tail;
	QQmlContext *edge_context, *label_context, *head_context, *tail_context;
};

using itmgraph = po::digraph<node_proxy,edge_proxy>;

struct point
{
	enum Type : uint8_t
	{
		Entry,
		Exit,
		Corner,
		Center
	};

	bool operator==(point const& p) const { return p.node == node && p.x == x && p.y == y && type == p.type; }
	bool operator!=(point const& p) const { return !(p == *this); }

	itmgraph::vertex_descriptor node;
	Type type;
	int x, y;
};

using visgraph = std::unordered_multimap<point,point>;

namespace std
{
	template<>
	struct hash<point>
	{
		size_t operator()(struct point const& p) const
		{
			return po::hash_struct<int,int,int,uint8_t>(p.node.id,p.x,p.y,p.type);
		}
	};

	template<>
	struct hash<node_proxy>
	{
		size_t operator()(node_proxy const& p) const
		{
			return po::hash_struct(p.item,p.context);
		}
	};
}

class Sugiyama : public QQuickPaintedItem
{
	Q_OBJECT

	Q_PROPERTY(QQmlComponent* vertex READ vertex WRITE setVertex NOTIFY vertexChanged)
	Q_PROPERTY(QQmlComponent* edge READ edge WRITE setEdge NOTIFY edgeChanged)
	Q_PROPERTY(QObject* procedure READ procedure WRITE setProcedure NOTIFY procedureChanged)

public:
	using layout_type = std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>>;
	using route_type = std::unordered_map<itmgraph::edge_descriptor,std::pair<QPainterPath,QPointF>>;
	using cache_type = std::tuple<itmgraph,boost::optional<layout_type>,boost::optional<route_type>>;

	static const int nodeBorderPadding;
	static const int edgeRadius;
	static const int nodePortPadding;

	Sugiyama(QQuickItem *parent = nullptr);
	virtual ~Sugiyama(void);

	QQmlComponent* vertex(void) const { return _vertex; }
	QQmlComponent* edge(void) const { return _edge; }
	QObject* procedure(void) const { return _procedure; }

	void setVertex(QQmlComponent* c);
	void setEdge(QQmlComponent* c);
	void setProcedure(QObject* o);

	virtual void paint(QPainter*) override;

public slots:
	void layout(void);
	void route(void);
	void updateEdge(QObject*);
	void processRoute(void);
	void processLayout(void);

signals:
	void vertexChanged(void);
	void edgeChanged(void);
	void procedureChanged(void);

	void layoutStart(void);
	void layoutDone(void);
	void routeStart(void);
	void routeDone(void);

private:
	// Properties
	QQmlComponent* _vertex;
	QQmlComponent* _edge;
	Procedure* _procedure;

	std::unordered_map<po::proc_wloc,cache_type> _cache;
	QSignalMapper _mapper;
	QFutureWatcher<std::pair<po::proc_wloc,layout_type>> _layoutWatcher;
	QFutureWatcher<std::pair<po::proc_wloc,route_type>> _routeWatcher;
	bool _routingNeeded;
	std::mutex _mutex;

	void positionNode(itmgraph::vertex_descriptor v, itmgraph const& graph, std::tuple<unsigned int,unsigned int,unsigned int> pos);
	void positionEdgeDecoration(itmgraph::edge_descriptor e, cache_type const& cache);
	void updateEdgeDecorations(itmgraph::edge_descriptor e, cache_type& cache);
	void redoAttached(void);
};
