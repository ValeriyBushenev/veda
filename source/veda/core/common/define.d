/**
 * define
 */
module veda.core.common.define;

import std.concurrency, std.file, std.stdio, core.atomic;
import veda.common.type;

// variable process_name static mirror of g_process_name
string process_name;
static this()
{
    get_g_process_name();
}
/////////////////////////////// g_process_name //////////////////////////
private shared string g_process_name;
public string get_g_process_name(){
    process_name = atomicLoad(g_process_name);
    return process_name;
}

public void set_g_process_name(string new_data){
    atomicStore(g_process_name, new_data);
    process_name = new_data;
}

long     max_size_of_individual = 1024 * 512;

string[] access_list_predicates = [ "v-s:canCreate", "v-s:canRead", "v-s:canUpdate", "v-s:canDelete" ];

enum CNAME : byte {
    COUNT_PUT        = 0,
    COUNT_GET        = 1,
    WORKED_TIME      = 2,
    LAST_UPDATE_TIME = 3
}

alias immutable(long)[] const_long_array;

const byte              asObject = 0;
const byte              asArray  = 1;
const byte              asString = 2;

interface Outer
{
    void put(string data);
}

enum EVENT : byte {
    CREATE    = 1,
    UPDATE    = 2,
    REMOVE    = 3,
    NONE      = 4,
    ERROR     = 5,
    NOT_READY = 6
}

const string acl_indexes_db_path   = "./data/acl-indexes";
const string attachments_db_path   = "./data/files";
const string dbs_data              = "./data";
const string uris_db_path          = "./data/uris";
const string tmp_path              = "./data/tmp";
const string queue_db_path         = "./data/queue";
const string onto_path             = "./ontology";
const string xapian_info_path      = "./data/xapian-info";
const string module_info_path      = "./data/module-info";
const string trails_path           = "./data/trails";
const string logs_path             = "./logs";
const string individuals_db_path0  = "./data/lmdb-individuals";
const string tickets_db_path0      = "./data/lmdb-tickets";

const string main_queue_name       = "individuals-flow";
const string ft_indexer_queue_name = "fulltext_indexer0";

string[]     paths_list            =
[
    tmp_path, logs_path, attachments_db_path, dbs_data, uris_db_path, queue_db_path,
    xapian_info_path, module_info_path, trails_path, acl_indexes_db_path, individuals_db_path0, tickets_db_path0
];


private string[ string ] _xapian_search_db_path;
private void init_xapiab_db_paths(){
    _xapian_search_db_path =
    [ "base":"data/xapian-search-base", "system":"data/xapian-search-system", "deleted":"data/xapian-search-deleted", "az":"data/xapian-search-az" ];
}
public string get_xapiab_db_path(string db_name){
    if (_xapian_search_db_path.length == 0)
        init_xapiab_db_paths();
    return _xapian_search_db_path.get(db_name, null);
}
public string[] get_xapian_db_names(){
    if (_xapian_search_db_path.length == 0)
        init_xapiab_db_paths();
    return _xapian_search_db_path.keys();
}


public const int xapian_db_type = 1;

void create_folder_struct(){
    foreach (path; paths_list) {
        try
        {
            mkdir(path);
            writeln("create folder: ", path);
        }
        catch (Exception ex)
        {
        }
    }
}

/// id подсистем
public enum SUBSYSTEM : ubyte {
    NONE              = 0,
    STORAGE           = 1,
    FULL_TEXT_INDEXER = 4,
    FANOUT_EMAIL      = 8,
    SCRIPTS           = 16,
    FANOUT_SQL        = 32,
    USER_MODULES_TOOL = 64
}

private string[ SUBSYSTEM ] sn;
private SUBSYSTEM[ string ] ns;

public SUBSYSTEM get_subsystem_id_of_name(string name){
    if (ns.length == 0) {
        ns[ "FANOUT_EMAIL" ]      = SUBSYSTEM.FANOUT_EMAIL;
        ns[ "FANOUT_SQL" ]        = SUBSYSTEM.FANOUT_SQL;
        ns[ "FULL_TEXT_INDEXER" ] = SUBSYSTEM.FULL_TEXT_INDEXER;
        ns[ "SCRIPTS" ]           = SUBSYSTEM.SCRIPTS;
        ns[ "STORAGE" ]           = SUBSYSTEM.STORAGE;
        ns[ "USER_MODULES_TOOL" ] = SUBSYSTEM.USER_MODULES_TOOL;
    }

    return ns.get(name, SUBSYSTEM.NONE);
}

public string get_name_of_subsystem_id(SUBSYSTEM id){
    if (sn.length == 0) {
        sn[ SUBSYSTEM.FANOUT_EMAIL ]      = "FANOUT_EMAIL";
        sn[ SUBSYSTEM.FANOUT_SQL ]        = "FANOUT_SQL";
        sn[ SUBSYSTEM.FULL_TEXT_INDEXER ] = "FULL_TEXT_INDEXER";
        sn[ SUBSYSTEM.SCRIPTS ]           = "SCRIPTS";
        sn[ SUBSYSTEM.STORAGE ]           = "STORAGE";
        sn[ SUBSYSTEM.USER_MODULES_TOOL ] = "USER_MODULES_TOOL";
    }

    return sn.get(id, "");
}

/// id компонентов
public enum COMPONENT : ubyte {
    /// сохранение индивидуалов
    subject_manager  = 1,

    /// Полнотекстовое индексирование
    fulltext_indexer = 4,

    /// Отправка email
    fanout_email     = 8,

    /// исполнение скриптов, normal priority
    scripts_main     = 16,

    /// Выдача и проверка тикетов
    ticket_manager   = 29,

    /// Выгрузка в sql, низкоприоритетное исполнение
    fanout_sql_lp    = 30,

    /// Выгрузка в sql, высокоприоритетное исполнение
    fanout_sql_np    = 32,

    /// исполнение скриптов, low priority
    scripts_lp       = 33,

    /// исполнение скриптов, low priority1
    scripts_lp1      = 50,

    ///////////////////////////////////////

    /// Сохранение накопленных в памяти данных
    commiter          = 36,

    n_channel         = 38,

    user_modules_tool = 64
}


/// id процессов
public enum P_MODULE : ubyte {
    ticket_manager  = COMPONENT.ticket_manager,
    subject_manager = COMPONENT.subject_manager,
    commiter        = COMPONENT.commiter,
    n_channel       = COMPONENT.n_channel,
}

/// id модулей обрабатывающих очередь
public enum MODULE : ubyte {
    ticket_manager    = COMPONENT.ticket_manager,
    subject_manager   = COMPONENT.subject_manager,
    fulltext_indexer  = COMPONENT.fulltext_indexer,
    scripts_main      = COMPONENT.scripts_main,
    scripts_lp        = COMPONENT.scripts_lp,
    scripts_lp1       = COMPONENT.scripts_lp1,
    user_modules_tool = COMPONENT.user_modules_tool,
    fanout_sql_np     = COMPONENT.fanout_sql_np,
    fanout_sql_lp     = COMPONENT.fanout_sql_lp,
}

/// Команды используемые процессами
/// Сохранить
byte CMD_PUT    = 1;

/// Найти
byte CMD_FIND   = 2;

/// Коммит
byte CMD_COMMIT = 16;

byte CMD_MSG    = 17;

byte CMD_EXIT   = 49;

/// Установить
byte CMD_SET    = 50;

public string subsystem_byte_to_string(long src){
    string res = "";

    foreach (el; [ SUBSYSTEM.FANOUT_EMAIL, SUBSYSTEM.FANOUT_SQL, SUBSYSTEM.FULL_TEXT_INDEXER, SUBSYSTEM.SCRIPTS, SUBSYSTEM.STORAGE, SUBSYSTEM.USER_MODULES_TOOL ]) {
        if ((src & el) == el) {
            if (res != "")
                res ~= ",";
            res ~= get_name_of_subsystem_id(el);
        }
    }

    return res;
}
