#include <QDebug>

#pragma once

/*!
 * \brief Continuous selection of characters in a Column.
 *
 * All lines have the same column (character) count and every character
 * represents the same amount of bytes. Used by the view part of the
 * application to encapsulate its selections.
 *
 * \ingroup gui
 */
class ElementSelection
{
public:
	/**
	 * New ElementSelection spanning from line \e anc_line, column \e anc_col
	 * to line \e cur_line, column \e cur_col.
	 */
	ElementSelection(quint64 anc_line, quint64 anc_col, quint64 cur_line, quint64 cur_col);

	/**
	 * \returns Anchor line.
	 */
	quint64 anchorLine(void) const;

	/**
	 * \returns Anchor column.
	 */
	quint64 anchorColumn(void) const;

	/**
	 * \returns Cursor line.
	 */
	quint64 cursorLine(void) const;

	/**
	 * \returns Cursor column.
	 */
	quint64 cursorColumn(void) const;

	/**
	 * First (as in torwards smaller addesses) line the selection
	 * bounds to.
	 */
	quint64 firstLine(void) const;

	/**
	 * First (as in torwards smaller addesses) column the selection
	 * bounds to.
	 */
	quint64 firstColumn(void) const;

	/**
	 * First (as in torwards higher addesses) line the selection
	 * bounds to.
	 */
	quint64 lastLine(void) const;

	/**
	 * First (as in torwards higher addesses) column the selection
	 * bounds to.
	 */
	quint64 lastColumn(void) const;

	/**
	 * \returns True if anchor != cursor, i.e. the object selects
	 * more than one element.
	 */
	bool hasSelection(void) const;

	/**
	 * Moves the cursor to line \e line, column \e col.
	 */
	void setCursor(quint64 line,quint64 col);

	/**
	 * \returns True if s ⊄ this
	 */
	bool includes(const ElementSelection &s) const;
	bool includes(quint64 column, quint64 line) const;

	/**
	 * \returns True if disjoint from sel
	 */
	bool disjoint(const ElementSelection &sel) const;

private:
	quint64 m_anchorLine;
	quint64 m_anchorColumn;

	quint64 m_cursorLine;
	quint64 m_cursorColumn;
};

uint qHash(const ElementSelection &s);
bool operator==(const ElementSelection &s1,const ElementSelection &s2);
QDebug operator<<(QDebug dbg, const ElementSelection &c);
