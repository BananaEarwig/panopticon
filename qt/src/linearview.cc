#include <cassert>
#include "linearview.hh"

Header::Header(void) : QObject(), m_name(""), m_collapsed(false), m_id(-1) {}
Header::Header(const QString &h, bool col, int id) : QObject(), m_name(h), m_collapsed(col), m_id(id) {}
Header::~Header(void) {}

QString Header::name(void) const { return m_name; }
bool Header::collapsed(void) const { return m_collapsed; }
int Header::id(void) const { return m_id; }

LinearViewBlock::LinearViewBlock(void)
: type(Data), delegate(), id(-1)
{}

LinearViewBlock::LinearViewBlock(LinearViewBlock::Type t, QSharedPointer<Delegate> d, int i)
: type(t), delegate(d), id(i)
{}

LinearViewBlock::LinearViewBlock(const LinearViewBlock &o)
: type(o.type), delegate(o.delegate), id(o.id)
{}

bool LinearViewBlock::operator==(const LinearViewBlock &r) const
{
	return type == r.type && delegate == r.delegate && id == r.id;
}

LinearViewBlock &LinearViewBlock::operator+=(const LinearViewBlock &r)
{
	return *this;
}

LinearViewContext::LinearViewContext(QObject *parent)
: QObject(parent), m_columnWidth(0)
{}

qreal LinearViewContext::columnWidth(void) const
{
	return m_columnWidth;
}

void LinearViewContext::setColumnWidth(qreal r)
{
	if(r != m_columnWidth)
	{
		m_columnWidth = r;
		emit columnWidthChanged();
	}
}

std::shared_ptr<Delegate> operator+=(std::shared_ptr<Delegate> &a, const std::shared_ptr<Delegate> &b)
{
	return (a = b);
}

LinearView::LinearView(QQuickItem *parent)
: QQuickItem(parent), _engine(), _context(), _session(nullptr), _delegates(), _globalRowIndex(0), _yOffset(0), _visibleRows(), _references()
{
	setFlags(QQuickItem::ItemHasContents);
	setAcceptedMouseButtons(Qt::LeftButton);

	_engine.rootContext()->setContextProperty("linearViewContext",&_context);
	scrollViewport();
	setClip(true);
}

LinearView::~LinearView(void)
{}

Session *LinearView::session(void)
{
	return _session;
}

void LinearView::setSession(Session *s)
{
	if(s != _session)
	{
		_session = s;
		_references.clear();
		_delegates.clear();

		rowIndex gri = 0;
		for(auto p: po::projection(_session->graph()))
		{
			std::shared_ptr<Delegate> del = std::make_shared<TestDelegate>(p.second,10,&_engine,this);
			auto len = del->rowCount();

			_delegates += std::make_pair(decltype(_delegates)::interval_type::right_open(gri,gri+len),del);
			connect(del.get(),SIGNAL(modified()),this,SLOT(delegateModified()));
			gri += len;
		}

		emit sessionChanged();
	}
}

void LinearView::delegateModified(void)
{
	/*
	auto del = qobject_cast<Delegate*>(sender());

	assert(del);
	auto i = m_visibleRows.begin();
	qreal offset = (*i)->y();

	while(i != m_visibleRows.end())
	{
		auto row = std::distance(m_visibleRows.begin(),i) + m_visibleTopRow;
		auto j = m_availableBlocks.find(row);

		assert(j != m_availableBlocks.end());
		if(j->second.type == LinearViewBlock::Header)
			j->second.delegate->deleteHead(*i);
		else
			j->second.delegate->deleteRow(*i);

		++i;
	}

	auto bak = m_availableBlocks;
	int id = 0, k = 0;

	m_visibleRows.clear();
	m_availableBlocks.clear();
	for(auto j: bak)
	{
		if(j.second.type == LinearViewBlock::Header)
		{
			auto len = j.second.delegate->rowCount();

			m_availableBlocks.add(std::make_pair(decltype(m_availableBlocks)::interval_type::right_open(k,k + 1),LinearViewBlock(LinearViewBlock::Header,j.second.delegate,id)));
			m_availableBlocks.add(std::make_pair(decltype(m_availableBlocks)::interval_type::right_open(k + 1,k + 1 + len),LinearViewBlock(LinearViewBlock::Data,j.second.delegate,id)));

			k += len + 1;
			id += 1;
		}
	}

	scrollViewport(0);
	scrollViewport(offset);
*/
	qWarning() << "LinearView::delegateModified not implemented";
}

void LinearView::rowHeightChanged(void)
{/*
	QQuickItem *prev = 0;
	std::for_each(std::find(m_visibleRows.begin(),m_visibleRows.end(),sender()),m_visibleRows.end(),[&](QQuickItem *itm)
	{
		if(prev)
			itm->setY(prev->y() + prev->height());
		prev = itm;
	});*/
	qWarning() << "LinearView::rowHeightChanged not implemented";
}

void LinearView::wheelEvent(QWheelEvent *event)
{
	scrollViewport(event->angleDelta().y() / 8);
}

void LinearView::mouseMoveEvent(QMouseEvent *event)
{
	if(event->buttons() & Qt::LeftButton)
	{
		QPointF ptn = event->localPos();
		auto i = std::find_if(_visibleRows.begin(),_visibleRows.end(),[&](const std::pair<rowIndex,QQuickItem *> &j)
			{ return QRectF(QPointF(j.second->x(),j.second->y()),j.second->boundingRect().size()).contains(ptn); });

		if(i != _visibleRows.end())
		{
			QQuickItem *itm = i->second;

			QVariant ret;
			QMetaObject::invokeMethod(itm,"mouseMoved",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,ptn.x() - itm->x()),Q_ARG(QVariant,ptn.y() - itm->y()));
			event->accept();
		}
	}
}

void LinearView::mousePressEvent(QMouseEvent *event)
{
	if(event->buttons() & Qt::LeftButton)
	{
		QPointF ptn = event->localPos();
		auto i = std::find_if(_visibleRows.begin(),_visibleRows.end(),[&](const std::pair<rowIndex,QQuickItem *> &j)
			{ return QRectF(QPointF(j.second->x(),j.second->y()),j.second->boundingRect().size()).contains(ptn); });

		if(i != _visibleRows.end())
		{
			QQuickItem *itm = i->second;

			QVariant ret;
			QMetaObject::invokeMethod(itm,"mousePressed",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,ptn.x() - itm->x()),Q_ARG(QVariant,ptn.y() - itm->y()));
			event->accept();
		}
	}
}

void LinearView::geometryChanged(const QRectF&, const QRectF&)
{
	scrollViewport();
}

void LinearView::scrollViewport(float delta)
{
	QRectF bb(x(),y(),width(),height());
	rowIndex firstVisibleRowIndex = std::numeric_limits<rowIndex>::max();
	auto j = _visibleRows.begin();

	// move rows and delete those out of sight
	while(j != _visibleRows.end())
	{
		const std::pair<rowIndex,QQuickItem*> &p = *j;
		const QRectF &itemBB = p.second->boundingRect();

		if(bb.intersects(QRectF(mapFromItem(p.second,itemBB.topLeft() + QPointF(0,delta)),itemBB.size())))
		{
			firstVisibleRowIndex = std::min(firstVisibleRowIndex,p.first);
			p.second->setY(p.second->y() + delta);
			++j;
		}
		else
		{
			std::shared_ptr<Delegate> del = _delegates.find(p.first)->second;

			del->deleteRow(p.second);
			j = _visibleRows.erase(j);
		}
	}

	// insert empty space w/ new rows
	if(firstVisibleRowIndex == std::numeric_limits<rowIndex>::max())
	{
		insertRows(y(),0,false);
	}
	else
	{
		if(firstVisibleRowIndex > 0 && _visibleRows[firstVisibleRowIndex]->y() > bb.top())
			insertRows(_visibleRows[firstVisibleRowIndex]->y(),firstVisibleRowIndex-1,true);

		auto last = std::prev(_visibleRows.end());
		if(last->second->y() + last->second->height() < bb.bottom())
			insertRows(last->second->y() + last->second->height(),last->first + 1,false);
	}

	if(_visibleRows.size())
	{
		// prevent scrolling above the first row
		if(_visibleRows.begin()->first == 0 && _visibleRows.begin()->second->y() > 0)
			return scrollViewport(-_visibleRows.begin()->second->y());

		// prevent scrolling below the last row
		auto last = std::prev(_visibleRows.end());
		float bot = last->second->y() + last->second->height();
		if((_visibleRows.begin()->first != 0 || _visibleRows.begin()->second->y() < 0) && last->first == boost::icl::length(_delegates) - 1 && bot < bb.bottom())
		{
			float bot_adj = bb.bottom() - bot;
			float top_adj = (_visibleRows.begin()->first == 0 ? -_visibleRows.begin()->second->y() : std::numeric_limits<float>::max());

			return scrollViewport(std::min<float>(bot_adj,top_adj));
		}
	}
}

void LinearView::insertRows(float y, rowIndex gri, bool up)
{
	QRectF bb(x(),QQuickItem::y(),width(),height());

	while(gri >= 0 && gri < boost::icl::length(_delegates) && bb.contains(QPointF(bb.x(),y)))
	{
		auto j = _delegates.find(gri);

		assert(j != _delegates.end());
		{
			rowIndex l = gri - boost::icl::first(j->first);
			QQuickItem *itm = j->second->createRow(l);

			if(itm)
			{
				itm->setParentItem(this);
				itm->setX(0);
				itm->setY(y + (up ? -itm->height() : 0));

				connect(itm,SIGNAL(heightChanged()),this,SLOT(rowHeightChanged()));
				assert(_visibleRows.emplace(gri,itm).second);

				y += (up ? -itm->height() : itm->height());
			}

			gri += (up ? -1 : 1);
		}
	}
}
