#include <QList>
#include <QString>
#include <QStringList>
#include <QSharedPointer>
#include <QtQuick>

class Delegate;
class Line;

typedef QSharedPointer<Line> LineRef;

#include <source.hh>
#include <elementselection.hh>
#include <boost/optional.hpp>

#pragma once

/*!
 * \class QObject
 * \ingroup qt
 */
/*
class Element : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString data READ data NOTIFY dataChanged)
	Q_PROPERTY(bool selected READ selected NOTIFY selectedChanged)

public:
	Element(void);
	Element(const QString &h, bool sel);
	virtual ~Element(void);

	QString data(void) const;
	bool selected(void) const;

signals:
	void dataChanged(void);
	void selectedChanged(void);

private:
	QString m_data;
	bool m_selected;
};*/

/*!
 * \brief Single line to display in a column.
 *
 * This class transports rendered lines from the delegate
 * to the column. Includes text, color, indetation and optional
 * action.
 * Text is split into elements that are layout as columns.
 */
/*
class Line
{
public:
	unsigned int identation;
	bool align;
	using Element = QString;
	QList<Element> elements;
};*/

/**
 * \brief Delegates connect Columns to a database.
 *
 * Delegate and its subclasses create strings for the columns
 * to render and translate cursor/mouse movements from byte- to
 * elementoriented objects.
 * \ingroup gui
 */
class Delegate : public QObject
{
	Q_OBJECT

public:
	/**
	 * New delegate for \e sec of database \e db. Dimensions will be \e lines height,
	 *
	 * The parent object is used if the database \e db is private. In this case the parent object
	 * will recvieve the MouseSelection and CursorSelection events.
	 */
	Delegate(const po::address_space &as, const po::rrange &r, QObject *parent = 0);

	virtual ~Delegate(void);

	/**
	 * Strings for a supplied line.
	 * Returns a list of strings representing the columns
	 * of a line. The length of the list can be less than \ref columns()!
	 */
	virtual QQuickItem *data(unsigned int i) = 0;

	/*!
	 * Number of lines this delegate spans.
	 */
	virtual unsigned int rows(void) const = 0;

	/*!
	 * Current cursor selection or NULL if nothing is selected. Used to initialize Columns
	 * after this have been created.
	 * Caller take ownership otf the returned object.
	 *
	virtual const boost::optional<ElementSelection>& cursor(void) const = 0;*/

	/*!
	 * Current mouse selection or NULL. See cursor().
	 * Caller take ownership otf the returned object.
	 *
	virtual const boost::optional<ElementSelection>& mouse(void) const = 0;*/

	const po::address_space &space(void) const;
	const po::rrange &range(void) const;

	//virtual boost::optional<ElementSelection> elementSelection(const po::rrange &sel);
	//virtual po::rrange byteSelection(const boost::optional<ElementSelection> &sel);

//public slots:
	//virtual void setMouse(const boost::optional<ElementSelection> &pos) = 0;
	//virtual void setCursor(const boost::optional<ElementSelection> &sel) = 0;

signals:
	void modified(const boost::optional<ElementSelection> &pos);

private:
	po::address_space m_space;
	po::rrange m_range;
};

class TestDelegateContext : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString address READ address NOTIFY addressChanged)
	Q_PROPERTY(QVariantList data READ data NOTIFY dataChanged)
	Q_PROPERTY(int row READ row NOTIFY rowChanged)

public:
	TestDelegateContext(QObject *parent = nullptr);
	TestDelegateContext(const QString &a, const QVariantList &d, int row, QObject *parent = nullptr);

	QString address(void) const;
	QVariantList data(void) const;
	int row(void) const;

signals:
	void addressChanged(void);
	void dataChanged(void);
	void rowChanged(void);

private:
	QString m_address;
	QVariantList m_data;
	int m_row;
};

class TestDelegate : public Delegate
{
	Q_OBJECT

public:
	TestDelegate(const po::address_space &as, const po::rrange &r, unsigned int width, QQmlEngine *, QObject *p = 0);
	virtual ~TestDelegate(void);

	virtual QQuickItem *data(unsigned int l);
	virtual unsigned int rows(void) const;
	//unsigned int width(unsigned int l) const;

	void setCursor(const boost::optional<ElementSelection> &sel);

public slots:
	void elementClicked(int,int);
	void elementEntered(int,int);

private:
	unsigned int m_width;
	QQmlEngine *m_engine;
	QQmlComponent m_component;
	boost::optional<ElementSelection> m_cursor;
};
