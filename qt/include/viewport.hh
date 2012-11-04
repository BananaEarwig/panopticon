#ifndef VIEWPORT_HH
#define VIEWPORT_HH

#include <QGraphicsView>
#include <QWheelEvent>
#include <QItemSelectionModel>

#include <graph.hh>
#include <model.hh>

class Viewport : public QGraphicsView
{
	Q_OBJECT

public:
	Viewport(QAbstractItemModel *m, QModelIndex i, QItemSelectionModel *s, QWidget *parent = 0);
	~Viewport(void);

	void setRootIndex(QModelIndex i);

protected:
	virtual void wheelEvent(QWheelEvent *event);

private slots:
	void sceneRectChanged(const QRectF &r);
	void sceneSelectionChanged(void);
	void modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected);

private:
	QAbstractItemModel *m_model;
	QItemSelectionModel *m_selection;
	Graph m_graph;
	QPersistentModelIndex m_root;

	std::map<ptrdiff_t,Node *> m_uid2procedure;
	std::map<Node *,int> m_procedure2row;

	void populate(void);
};

#endif
