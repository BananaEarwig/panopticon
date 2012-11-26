#ifndef MODEL_HH
#define MODEL_HH

#include <deflate.hh>
#include <QAbstractItemModel>
#include <QFont>
#include <QCache>
#include <QDebug>
#include <QHash>

#include <unordered_map>
#include <functional> 
#include <vector>

struct Path
{
	enum Type
	{
		FlowgraphType = 0,
		ProcedureType = 1,
		BasicBlockType = 2,
		MnemonicType = 3,
		ValueType = 4,
	};

	Path(void);

	bool operator==(const Path &) const;
	bool operator!=(const Path &) const;

	Type type;
	po::flowgraph *flow;
	po::procedure *proc;
	po::basic_block *bblock;
	const po::mnemonic *mne;
	const po::rvalue *value;
};

inline uint qHash(const Path &key)
{
	return key.type ^ (uint)key.flow ^ (uint)key.proc ^ (uint)key.bblock ^ (uint)key.mne ^ (uint)key.value;
}

class Model : public QAbstractItemModel
{
	Q_OBJECT

public:
	Model(po::flow_ptr f, QObject *parent = 0);
	~Model(void);

	// reading
	virtual QModelIndex index (int row, int column, const QModelIndex &parent = QModelIndex()) const;
	virtual QModelIndex parent(const QModelIndex &index) const;
	virtual int rowCount(const QModelIndex &parent = QModelIndex()) const;
	virtual int columnCount(const QModelIndex &parent = QModelIndex()) const;
	virtual QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const;

	// writing
	virtual Qt::ItemFlags flags(const QModelIndex &index) const;
	virtual bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole);

	enum Column
	{
		// FlowgraphType
		//NameColumn = 0,
		ProceduresColumn = 1,			// ProcedureType
		LastFlowgraphColumn = 2,

		// ProcedureType
		NameColumn = 0,
		EntryPointColumn = 1,			// BasicBlockType
		BasicBlocksColumn = 2,		// BasicBlockType
		CalleesColumn = 3,				// ProcedureType
		UniqueIdColumn = 4,
		LastProcedureColumn = 5,

		// BasicBlockType
		AreaColumn = 0,
		MnemonicsColumn = 1,			// MnemonicType,
		PredecessorsColumn = 2,		// BasicBlockType
		SuccessorsColumn = 3,			// BasicBlockType
		//UniqueIdColumn = 4,
		LastBasicBlockColumn = 5,

		// MnemonicType
		//AreaColumn = 0,
		OpcodeColumn = 1,
		OperandsColumn = 2,				// ValueType
		FormatsColumn = 3,				// QString parallel to OperandsColumn
		InstructionsColumn = 4,		// TODO
		LastMnemonicColumn = 5,

		// ValueType
		ValueColumn = 0,
		PositionColumn = 1, 			// QPoint
		LastValueColumn = 2
	};

private:
	QVariant displayData(const QModelIndex &index) const;
	bool setDisplayData(const QModelIndex &index, const std::string &value);
	QModelIndex createIndex(int row, int col, po::flowgraph *flow, po::procedure *proc = 0, po::basic_block *bblock = 0, const po::mnemonic *mne = 0, const po::rvalue *val = 0) const;
	const Path &path(uint p) const;

	mutable ptrdiff_t m_nextId;
	mutable QHash<uint,const Path *> m_idToPath;
	mutable QHash<const Path,uint> m_pathToId;
	po::deflate *m_deflate;
	std::vector<po::flow_ptr> m_flowgraphs;
};

#endif
