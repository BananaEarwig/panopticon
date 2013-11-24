#include <QtQuick>
#include <session.hh>
#include <delegate.hh>

#pragma once

class Header : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString name READ name NOTIFY nameChanged)
	Q_PROPERTY(bool collapsed READ collapsed NOTIFY collapsedChanged)
	Q_PROPERTY(int id READ id NOTIFY idChanged)

public:
	Header(void);
	Header(const QString &n, bool col, int id);
	virtual ~Header(void);

	QString name(void) const;
	bool collapsed(void) const;
	int id(void) const;

signals:
	void nameChanged(void);
	void collapsedChanged(void);
	void idChanged(void);

private:
	QString m_name;
	bool m_collapsed;
	int m_id;
};

struct LinearViewBlock
{
	enum Type
	{
		Data,
		Header,
		HeaderCollapsed,
	};

	LinearViewBlock(void);
	LinearViewBlock(Type t, QSharedPointer<Delegate> d, int id);
	LinearViewBlock(const LinearViewBlock &r);

	bool operator==(const LinearViewBlock &r) const;
	LinearViewBlock &operator+=(const LinearViewBlock &r);

	Type type;
	QSharedPointer<Delegate> delegate;
	int id;	///< Key when in m_hidden
};

class LinearViewContext : public QObject
{
	Q_OBJECT
	Q_PROPERTY(qreal columnWidth READ columnWidth WRITE setColumnWidth NOTIFY columnWidthChanged)

public:
	LinearViewContext(QObject *parent = 0);

	qreal columnWidth(void) const;
	void setColumnWidth(qreal);

signals:
	void columnWidthChanged(void);

private:
	qreal m_columnWidth;
};

class LinearView : public QQuickItem
{
	Q_OBJECT
	Q_PROPERTY(Session* session READ session WRITE setSession NOTIFY sessionChanged)

public:
	LinearView(QQuickItem *parent = 0);
	virtual ~LinearView(void);

	Session *session(void);
	void setSession(Session *);

public slots:
	void scrollViewport(qreal);
	void test(void);

	// From QML
	//void setSelect(int firstRow, int firstCol, int lastRow, int lastCol);
	//void setVisibility(int blkid, bool vis);

	//void delegateModified(const boost::optional<ElementSelection> &);

signals:
	void sessionChanged(void);

protected:
	virtual void wheelEvent(QWheelEvent *event);
	virtual void mouseMoveEvent(QMouseEvent *event);
	virtual void mousePressEvent(QMouseEvent *event);
	virtual void geometryChanged(const QRectF &newGeometry, const QRectF &oldGeometry);

private:
	QQmlEngine m_engine;
	QQmlComponent m_component;
	LinearViewContext m_context;
	boost::optional<Session*> m_session;
	std::list<QQuickItem*> m_viewport;
	int m_viewportIndex;
	boost::icl::split_interval_map<int,LinearViewBlock> m_currentView;
	std::unordered_map<int,LinearViewBlock> m_hidden;

	void addRows(bool up = false);
	QQuickItem *data(int);
	unsigned int rows(void) const;
};
