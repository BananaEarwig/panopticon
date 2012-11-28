#include <QDebug>
#include <QCoreApplication>
#include <QScrollBar>

#include <graphwidget.hh>

GraphWidget::GraphWidget(QAbstractItemModel *m, QModelIndex i, QWidget *parent)
: QGraphicsView(parent), m_model(m), m_root(i)
{
	setScene(&m_scene);
	setRenderHints(QPainter::Antialiasing | QPainter::SmoothPixmapTransform | QPainter::TextAntialiasing | QPainter::HighQualityAntialiasing);
	//setDragMode(QGraphicsView::ScrollHandDrag);
	setSceneRect(QRectF());
}

GraphWidget::~GraphWidget(void)
{
	return;
}

void GraphWidget::setRootIndex(const QModelIndex &i)
{
	m_root = i;
	QPointF p = populate();

	resetTransform();
	setSceneRect(m_scene.sceneRect().adjusted(-1000,-1000,2000,2000));
	if(!p.isNull())
		centerOn(p);
	else
		centerOn(sceneRect().center());
}

void GraphWidget::wheelEvent(QWheelEvent *event)
{
	double fac = (double)(event->delta()) / 150.f;
	fac = fac > 0.0f ? 1 / fac : -fac;
	scale(fac,fac);
}

void GraphWidget::mouseMoveEvent(QMouseEvent *event)
{
	if(event->buttons() & Qt::MiddleButton)
	{
		QPointF p = m_lastDragPos - event->pos();

		horizontalScrollBar()->setValue(horizontalScrollBar()->value() + p.x());
		verticalScrollBar()->setValue(verticalScrollBar()->value() + p.y());
		m_lastDragPos = event->pos();
	}
	else
		QGraphicsView::mouseMoveEvent(event);
}

void GraphWidget::mousePressEvent(QMouseEvent *event)
{
	if(event->button() == Qt::MiddleButton)
	{
		m_lastDragPos = event->pos();
		viewport()->setCursor(Qt::ClosedHandCursor);
	}
	else
		QGraphicsView::mousePressEvent(event);
}

void GraphWidget::mouseReleaseEvent(QMouseEvent *event)
{	
	if(event->button() == Qt::MiddleButton)
		viewport()->setCursor(Qt::ArrowCursor);
	else
		QGraphicsView::mouseReleaseEvent(event);
}
