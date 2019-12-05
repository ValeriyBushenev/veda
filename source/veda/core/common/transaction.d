module veda.core.common.transaction;

import std.stdio, std.json;
import veda.core.common.context, veda.common.type, veda.core.common.type, veda.onto.individual, veda.onto.resource, veda.onto.lang, veda.onto.onto;

struct TransactionItem {
    INDV_OP    cmd;
    string     ticket_id;
    string     event_id;
    string     user_uri;

    string     uri;

    long       update_counter;
    long       op_id;

    string     prev_binobj;
    string     new_binobj;

    Individual prev_indv;
    Individual new_indv;

    long       assigned_subsystems;

    ResultCode rc;

    immutable this(INDV_OP _cmd, string _user_uri, string _uri, string _prev_binobj, string _new_binobj, long _update_counter, string _event_id
                   , long _assigned_subsystems)
    {
        cmd                 = _cmd;
        user_uri            = _user_uri;
        uri                 = _uri;
        prev_binobj         = _prev_binobj;
        new_binobj          = _new_binobj;
        update_counter      = _update_counter;
        event_id            = _event_id;
        assigned_subsystems = _assigned_subsystems;
    }

    immutable this(TransactionItem ti)
    {
        cmd                 = ti.cmd;
        user_uri            = ti.user_uri;
        uri                 = ti.uri;
        prev_binobj         = ti.prev_binobj;
        new_binobj          = ti.new_binobj;
        update_counter      = ti.update_counter;
        event_id            = ti.event_id;
        assigned_subsystems = ti.assigned_subsystems;
    }
}

TransactionItem from_json(JSONValue jsn){
    TransactionItem res;

    res.cmd                 = cast(INDV_OP)jsn[ "cmd" ].integer;
    res.user_uri            = jsn[ "user_uri" ].str;
    res.uri                 = jsn[ "uri" ].str;
    res.prev_binobj         = jsn[ "prev_binobj" ].str;
    res.new_binobj          = jsn[ "new_binobj" ].str;
    res.update_counter      = jsn[ "update_counter" ].integer;
    res.event_id            = jsn[ "event_id" ].str;
    res.assigned_subsystems = jsn[ "assigned_subsystems" ].integer;

    return res;
}

struct Transaction {
    private
    {
        TransactionItem *[ string ] buff;
        TransactionItem[]            queue;
        immutable(TransactionItem)[] immutable_queue;
    }

    string      src;
    long        id;
    ResultCode  rc;
    int         count;


    public void add_immutable(ref immutable TransactionItem _ti){
        immutable_queue ~= _ti;
    }

    public void add(TransactionItem ti){
        queue ~= ti;
        TransactionItem *tii = &queue[ count ];
        string          kk   = ti.new_indv.uri.dup;
        buff[ kk ] = tii;
        count++;
    }

    public void reset(){
        if (buff.length > 0)
            buff = buff.init;

        if (queue.length > 0)
            queue = queue.init;

        if (immutable_queue.length > 0)
            immutable_queue = immutable_queue.init;

        count = 0;
    }

    public TransactionItem *get(string uri){
        return buff.get(uri, null);
    }

    public TransactionItem[] get_queue(){
        return queue;
    }

    public ref immutable(TransactionItem)[] get_immutable_queue(){
        return immutable_queue;
    }
}
