#include <QDebug>
#include <QApplication>
#include <QDesktopWidget>

#include <string>
#include <cassert>

#include <graph.hh>

extern "C" {
#include <gvc.h>
}

Arrow::Arrow(QPainterPath &pp, QGraphicsObject *f, QGraphicsObject *t)
: m_path(pp,this), m_from(f), m_to(t), m_highlighted(false), m_label("Test",this)
{
	setZValue(-1);
	m_head << QPointF(0,0) << QPointF(3*-1.3,3*3) << QPointF(0,3*2.5) << QPointF(3*1.3,3*3) << QPointF(0,0);
	setPath(pp);
	m_label.setBrush(QBrush(Qt::red));
}

QRectF Arrow::boundingRect(void) const
{
	QRectF a = mapFromItem(m_from,m_from->boundingRect().adjusted(-2,-2,2,2)).boundingRect();
	QRectF b = mapFromItem(m_to,m_to->boundingRect().adjusted(-2,-2,2,2)).boundingRect();
	QRectF c = mapFromItem(&m_path,m_path.boundingRect().adjusted(-2,-2,2,2)).boundingRect();
	QRectF d = mapFromItem(&m_label,m_label.boundingRect().adjusted(-2,-2,2,2)).boundingRect();

	return a | b | c | d;
}

void Arrow::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{	
	QPen p(m_highlighted ? Qt::blue : Qt::red,1);

	m_path.setPen(p);

	painter->save();
	painter->setPen(p);
	painter->setBrush(QBrush(m_highlighted ? Qt::blue : Qt::red));
	painter->translate(m_path.path().pointAtPercent(1));
	painter->rotate(90 - m_path.path().angleAtPercent(1));
	painter->drawConvexPolygon(m_head);
	painter->restore();
}

QGraphicsObject *Arrow::from(void)
{
	return m_from;
}

QGraphicsObject *Arrow::to(void)
{
	return m_to;
}

void Arrow::setHighlighted(bool t)
{
	m_highlighted = t;
}

void Arrow::setPath(QPainterPath &pp)
{
	prepareGeometryChange();
	m_path.setPath(pp);

	QSizeF sz = m_label.boundingRect().size();
	m_label.setPos(m_path.path().pointAtPercent(0.5) - QPointF(sz.width() / 2.0,sz.height() / 2.0));
}

Graph::Graph(void)
{
	return;
}

QList<QGraphicsObject *> &Graph::nodes(void)
{
	return m_nodes;
}

QList<Arrow *> &Graph::edges(void)
{
	return m_edges;
}

std::pair<Graph::iterator,Graph::iterator> Graph::out_edges(QGraphicsObject *n) 
{
	std::function<bool(Arrow *)> pred = [=](Arrow *a) { return a && a->from() == n; };
	return std::make_pair(iterator(pred,m_incidence.lowerBound(n),m_incidence.end()),iterator(pred,m_incidence.upperBound(n),m_incidence.end()));
}

std::pair<Graph::iterator,Graph::iterator> Graph::in_edges(QGraphicsObject *n) 
{
	std::function<bool(Arrow *)> pred = [=](Arrow *a) { return a &&  a->to() == n; };
	return std::make_pair(iterator(pred,m_incidence.lowerBound(n),m_incidence.end()),iterator(pred,m_incidence.upperBound(n),m_incidence.end()));
}

QRectF Graph::graphLayout(QString algorithm)
{
	if(nodes().empty())
		return QRectF();

	GVC_t *gvc = gvContext();
	Agraph_t *graph = agopen((char *)std::string("g").c_str(),AGDIGRAPH);
	QMap<QGraphicsObject *,Agnode_t *> node_proxies;
	QMap<Agedge_t *,Arrow *> edge_proxies;

	agsafeset(graph,(char *)std::string("sep").c_str(),(char *)std::string("0").c_str(),(char *)std::string("0").c_str());
	agsafeset(graph,(char *)std::string("esep").c_str(),(char *)std::string("1").c_str(),(char *)std::string("1").c_str());

	// allocate Graphviz nodes shadowing the Nodes in the scene
	QListIterator<QGraphicsObject *> i(nodes());
	while(i.hasNext())
	{
		QGraphicsObject *n = i.next();
		std::string name = std::to_string((ptrdiff_t)n);
		QRectF bb = n->boundingRect();
		Agnode_t *p = agnode(graph,(char *)name.c_str());

		agsafeset(p,(char *)std::string("width").c_str(),(char *)std::to_string(bb.width()/72.0).c_str(),(char *)std::string("1").c_str());
		agsafeset(p,(char *)std::string("height").c_str(),(char *)std::to_string(bb.height()/72.0).c_str(),(char *)std::string("1").c_str());
		agsafeset(p,(char *)std::string("fixedsize").c_str(),(char *)std::string("true").c_str(),(char *)std::string("1").c_str());
		agsafeset(p,(char *)std::string("shape").c_str(),(char *)std::string("record").c_str(),(char *)std::string("1").c_str());
		agsafeset(p,(char *)std::string("label").c_str(),(char *)std::string("").c_str(),(char *)std::string("1").c_str());

		node_proxies.insert(n,p);
	}

	// add Graphviz edges 
	QMapIterator<QGraphicsObject *, Arrow *> j(m_incidence);
	while(j.hasNext())
	{
		j.next();
		Arrow *a = j.value();
		Agnode_t *from = node_proxies[a->from()], *to = node_proxies[a->to()];
		Agedge_t *e = agedge(graph,from,to);

		agsafeset(e,(char *)std::string("headport").c_str(),(char *)std::string("n").c_str(),(char *)std::string("n").c_str());
		agsafeset(e,(char *)std::string("tailport").c_str(),(char *)std::string("s").c_str(),(char *)std::string("s").c_str());
		agsafeset(e,(char *)std::string("arrowhead").c_str(),(char *)std::string("none").c_str(),(char *)std::string("none").c_str());
		agsafeset(e,(char *)std::string("arrowtail").c_str(),(char *)std::string("none").c_str(),(char *)std::string("none").c_str());
		edge_proxies.insert(e,a);
	}

	gvLayout(gvc,graph,(char *)algorithm.toStdString().c_str());
	//gvRender(gvc,graph,"xdot",stdout);

	QPointF off(GD_bb(graph).UR.x,GD_bb(graph).UR.y);

	// move Nodes accoring to Graphviz node_proxies
	QMapIterator<QGraphicsObject *,Agnode_t *> k(node_proxies);
	while(k.hasNext())
	{
		k.next();
		QGraphicsObject *n = k.key();
		Agnode_t *p = k.value();
		QSizeF sz(ND_width(p)*72,ND_height(p)*72);
		QPointF orig(ND_coord(p).x,off.y()-ND_coord(p).y);

		n->setPos(orig - QPointF(sz.width() / 2.0,sz.height() / 2.0));
	}

	QMapIterator<Agedge_t *,Arrow *> m(edge_proxies);
	while(m.hasNext())
	{
		m.next();

		Arrow *a = m.value();
		Agedge_t *e = m.key();
		QPainterPath pp;
		int i = 1;
		const pointf *p = ED_spl(e)->list[0].list;
		QPointF p0(p[0].x,off.y()-p[0].y);

		assert(((ED_spl(e)->list[0].size - 1) % 3) == 0);
		pp.moveTo(p0);

		while(i < ED_spl(e)->list[0].size)
		{
			QPointF p1(p[i].x,off.y()-p[i].y), 
							p2(p[i+1].x,off.y()-p[i+1].y), 
							p3(p[i+2].x,off.y()-p[i+2].y);	// points

			pp.cubicTo(p1,p2,p3);
			i += 3;
		}

		a->setPath(pp);
	}

	agclose(graph);
	gvFreeContext(gvc);
	
	return QRectF();
}

void Graph::insert(QGraphicsObject *n)
{
	assert(!m_nodes.contains(n));
	
	addItem(n);
	m_nodes.append(n);
}

void Graph::connect(QGraphicsObject *from, QGraphicsObject *to)
{
	assert(m_nodes.contains(from) && m_nodes.contains(to));
	
	QPainterPath pp;
	Arrow *e = new Arrow(pp,from,to);

	addItem(e);
	m_edges.append(e);
	m_incidence.insert(from,e);
}

void Graph::clear(void)
{
	QGraphicsScene::clear();
	m_incidence.clear();
	m_edges.clear();
	m_nodes.clear();
}

